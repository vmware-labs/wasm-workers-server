use anyhow::Result;
use base64::{engine::general_purpose, Engine as _};
use image2tensor::{ColorOrder, TensorType as ImageTensorType};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use wasi_nn::{ExecutionTarget, GraphBuilder, GraphEncoding, TensorType};
use wasm_workers_rs::{
    http::{self, Request, Response},
    worker, Content,
};

/// Result labels
#[derive(Deserialize)]
pub struct Dataset {
    pub names: Vec<String>,
}

// A wrapper for label and probability
#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct InferenceResult(String, f32);

pub fn inference(path: &str, labels: &Dataset) -> Result<Vec<InferenceResult>, wasi_nn::Error> {
    let model = env::var("MODEL").unwrap();

    let model_xml = format!("/tmp/model/{model}.xml");
    let model_bin = format!("/tmp/model/{model}.bin");

    let input_dim = vec![1, 3, 224, 224];
    let mut output_buffer = vec![0f32; 1001];

    let graph = GraphBuilder::new(GraphEncoding::Openvino, ExecutionTarget::CPU)
        .build_from_files([&model_xml, &model_bin])
        .unwrap();

    eprintln!("Load graph");
    let mut ctx = graph.init_execution_context()?;
    eprintln!("Init execution context");

    let width: u32 = 224;
    let height: u32 = 224;

    let bytes = image2tensor::convert_image_to_bytes(
        path,
        width,
        height,
        ImageTensorType::F32,
        ColorOrder::BGR,
    )
    .unwrap();

    ctx.set_input(0, TensorType::F32, &input_dim, &bytes)?;

    // Do the inference.
    ctx.compute()?;
    eprintln!("Run graph inference");

    // Retrieve the output.
    ctx.get_output(0, &mut output_buffer)?;

    Ok(sort_results(&output_buffer, labels))
}

// Sort the buffer of probabilities.
fn sort_results(buffer: &[f32], labels: &Dataset) -> Vec<InferenceResult> {
    let mut results: Vec<InferenceResult> = buffer
        .iter()
        // In this specific case, the inference probabilities start at index 1.
        // TODO: research more about where this issue may come from.
        .skip(1)
        .enumerate()
        .map(|(c, p)| InferenceResult(labels.names.get(c).unwrap().to_string(), *p))
        .collect();
    results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    results
}

#[derive(Serialize)]
struct Results {
    data: Vec<InferenceResult>,
}

#[worker]
fn handler(req: Request<String>) -> Result<Response<Content>> {
    let body = req.body();

    let image = general_purpose::STANDARD.decode(&body).unwrap();
    // Save the image for image2tensor
    // TODO: Avoid this step by processing the bytes directly
    fs::write("/tmp/images/image.jpg", &image).unwrap();

    let labels_file = File::open("/tmp/model/dataset.json").unwrap();
    let reader = BufReader::new(labels_file);
    let labels: Dataset = serde_json::from_reader(reader).unwrap();

    let result = inference("/tmp/images/image.jpg", &labels).unwrap();

    // Applied changes here to use the Response method. This requires changes
    // on signature and how it returns the data.
    let results = Results {
        data: result[..10].to_vec().into(),
    };
    let response = serde_json::to_string(&results).unwrap();

    Ok(http::Response::builder()
        .status(200)
        .header("x-generated-by", "wasm-workers-server")
        .body(response.into())?)
}
