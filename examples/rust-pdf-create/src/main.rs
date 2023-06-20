use anyhow::Result;
use printpdf::*;
use std::io::BufWriter;
use wasm_workers_rs::{
    http::{self, Request, Response},
    worker, Content,
};

#[worker]
fn handler(req: Request<String>) -> Result<Response<Content>> {
    let mut buf = BufWriter::new(Vec::new());

    let (doc, page1, layer1) = PdfDocument::new("My Quote", Mm(247.0), Mm(210.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);
    let font = doc.add_builtin_font(BuiltinFont::TimesRoman).unwrap();

    current_layer.use_text("Your Quote", 24.0, Mm(20.0), Mm(190.0), &font);
    current_layer.use_text(req.body(), 18.0, Mm(20.0), Mm(170.0), &font);
    current_layer.use_text(
        "Created by a Wasm module in Wasm Workers Server",
        12.0,
        Mm(20.0),
        Mm(20.0),
        &font,
    );

    doc.save(&mut buf)?;

    let bytes = buf.into_inner()?;

    Ok(http::Response::builder()
        .status(200)
        .header("Content-Disposition", "attachment; filename=\"quote.pdf\"")
        .header("Content-Type", "application/pdf")
        .header("x-generated-by", "wasm-workers-server")
        .body(bytes.into())?)
}
