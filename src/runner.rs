// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use actix_web::{http::header::HeaderMap, http::StatusCode, HttpRequest};
use anyhow::Result;
use base64::decode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use wasi_common::{pipe::ReadPipe, pipe::WritePipe};
use wasmtime::*;
use wasmtime_wasi::sync::WasiCtxBuilder;

// Load the QuickJS compiled engine from kits/javascript
static JS_ENGINE_WASM: &[u8] =
    include_bytes!("../kits/javascript/wasm-workers-quick-js-engine.wasm");

/// JSON input for wasm modules. This information is passed via STDIN / WASI
/// to the module.
#[derive(Serialize, Deserialize)]
pub struct WasmInput {
    /// Request full URL
    url: String,
    /// Request method
    method: String,
    /// Request headers
    headers: HashMap<String, String>,
    /// Request body
    body: String,
    /// Key / Value store content if available
    kv: HashMap<String, String>,
    /// The list of parameters in the URL
    params: HashMap<String, String>,
}

impl WasmInput {
    /// Generates a new struct to pass the data to wasm module. It's based on the
    /// HttpRequest, body and the Key / Value store (if available)
    pub fn new(request: &HttpRequest, body: String, kv: Option<HashMap<String, String>>) -> Self {
        let mut params = HashMap::new();

        for (k, v) in request.match_info().iter() {
            params.insert(k.to_string(), v.to_string());
        }

        Self {
            url: request.uri().to_string(),
            method: String::from(request.method().as_str()),
            headers: build_headers_hash(request.headers()),
            body,
            kv: kv.unwrap_or_default(),
            params,
        }
    }
}

/// JSON output from a wasm module. This information is passed via STDOUT / WASI
/// from the module.
#[derive(Deserialize, Debug)]
pub struct WasmOutput {
    /// Response headers
    pub headers: HashMap<String, String>,
    /// Response HTTP status
    pub status: u16,
    /// New state of the K/V store if available
    pub kv: HashMap<String, String>,
    /// Response body data
    data: String,
    /// Internal value to indicate if the body is base64 encoded
    #[serde(default = "default_base64_encoding")]
    base64: bool,
}

fn default_base64_encoding() -> bool {
    false
}

impl WasmOutput {
    /// Initializes a new WasmOutput object
    pub fn new(
        body: &str,
        headers: HashMap<String, String>,
        status: u16,
        kv: HashMap<String, String>,
    ) -> Self {
        Self {
            data: String::from(body),
            base64: false,
            headers,
            status,
            kv,
        }
    }

    /// Build a default WasmOutput for a failed run. It will
    /// return a generic error message and the proper 503
    /// status code
    pub fn failed() -> Self {
        Self::new(
            "<p>There was an error running this function</p>",
            HashMap::from([("content-type".to_string(), "text/html".to_string())]),
            StatusCode::SERVICE_UNAVAILABLE.as_u16(),
            HashMap::new(),
        )
    }

    /// Return the content body as bytes. It will automatically
    /// decode the data if the base64 flag is enabled.
    pub fn body(&self) -> Result<Vec<u8>> {
        if self.base64 {
            Ok(decode(&self.data)?)
        } else {
            Ok(self.data.as_bytes().into())
        }
    }
}

/// Create HashMap from a HeadersMap
pub fn build_headers_hash(headers: &HeaderMap) -> HashMap<String, String> {
    let mut parsed_headers = HashMap::new();

    for (key, value) in headers.iter() {
        parsed_headers.insert(
            String::from(key.as_str()),
            String::from(value.to_str().unwrap()),
        );
    }

    parsed_headers
}

#[derive(Clone)]
pub enum RunnerWorkerType {
    Wasm,
    JavaScript,
}

/// A runner is composed by a Wasmtime engine instance and a preloaded
/// wasm module.
#[derive(Clone)]
pub struct Runner {
    /// Engine that runs the actual Wasm module
    engine: Engine,
    /// The type of the required runner
    runner_type: RunnerWorkerType,
    /// Preloaded Module
    module: Module,
    /// Source code if required
    source: String,
}

impl Runner {
    /// Creates a Runner. It will preload the module from the given wasm file
    pub fn new(path: &PathBuf) -> Result<Self> {
        let engine = Engine::default();
        let (runner_type, module, source) = if Self::is_js_file(path) {
            let module = Module::from_binary(&engine, JS_ENGINE_WASM)?;

            (
                RunnerWorkerType::JavaScript,
                module,
                fs::read_to_string(path)
                    .unwrap_or_else(|_| panic!("Error reading {}", path.display())),
            )
        } else {
            let module = Module::from_file(&engine, path)?;

            (RunnerWorkerType::Wasm, module, String::new())
        };

        Ok(Self {
            engine,
            runner_type,
            module,
            source,
        })
    }

    fn is_js_file(path: &Path) -> bool {
        match path.extension() {
            Some(os_str) => os_str == "js",
            None => false,
        }
    }

    /// Run the wasm module. To inject the data, it already receives the JSON input
    /// from the WasmInput serialization. It initializes a new WASI context with
    /// the required pipes. Then, it sends the data and read the output from the wasm
    /// run.
    pub fn run(
        &self,
        request: &HttpRequest,
        body: String,
        kv: Option<HashMap<String, String>>,
        vars: &HashMap<String, String>,
    ) -> Result<WasmOutput> {
        let input = serde_json::to_string(&WasmInput::new(request, body, kv)).unwrap();
        let stdin = match self.runner_type {
            RunnerWorkerType::Wasm => ReadPipe::from(input),
            RunnerWorkerType::JavaScript => {
                let mut contents = String::new();
                contents.push_str(&self.source);
                // Separator
                contents.push_str("[[[input]]]");
                contents.push_str(&input);

                ReadPipe::from(contents)
            }
        };
        let stdout = WritePipe::new_in_memory();
        let stderr = WritePipe::new_in_memory();

        let mut linker = Linker::new(&self.engine);
        wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;

        // Configure environment variables
        let tuple_vars: Vec<(String, String)> =
            vars.iter().map(|(k, v)| (k.clone(), v.clone())).collect();

        // WASI context
        let wasi = WasiCtxBuilder::new()
            .stdin(Box::new(stdin))
            .stdout(Box::new(stdout.clone()))
            .stderr(Box::new(stderr))
            .envs(&tuple_vars)?
            .inherit_args()?
            .build();
        let mut store = Store::new(&self.engine, wasi);

        linker.module(&mut store, "", &self.module)?;
        linker
            .get_default(&mut store, "")?
            .typed::<(), ()>(&store)?
            .call(&mut store, ())?;

        drop(store);

        let contents: Vec<u8> = stdout
            .try_into_inner()
            .map_err(|_err| anyhow::Error::msg("Nothing to show"))?
            .into_inner();

        let output: WasmOutput = serde_json::from_slice(&contents)?;

        Ok(output)
    }
}
