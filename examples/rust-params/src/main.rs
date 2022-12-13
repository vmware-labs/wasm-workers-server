use anyhow::Result;
use std::collections::HashMap;
use wasm_workers_rs::{
    http::{self, Request, Response},
    worker, Content,
};

#[worker(params)]
fn reply(_req: Request<String>, params: &HashMap<String, String>) -> Result<Response<Content>> {
    let unknown_id = String::from("the value is not available");
    let response = format!(
        "<!DOCTYPE html>
<head>
    <title>Wasm Workers Server</title>
    <meta name=\"viewport\" content=\"width=device-width,initial-scale=1\">
    <meta charset=\"UTF-8\">
    <link rel=\"stylesheet\" href=\"/water.min.css\">
    <link rel=\"stylesheet\" href=\"/main.css\">
</head>
<body>
    <main>
    <h1>Hello from Wasm Workers Server 👋</h1>
    <p>
        This is a dynamic route! The <code>[id]/fixed.js</code> worker is replying this URL.
        The <code>id</code> parameter value is: <code>{}</code>
    </p>
    <p>Read more about dynamic routes <a href=\"https://workers.wasmlabs.dev/docs/features/dynamic-routes\">in the documentation</a></p>
    </main>
</body>",
        params.get("id").unwrap_or_else(|| &unknown_id)
    );

    Ok(http::Response::builder()
        .status(200)
        .header("x-generated-by", "wasm-workers-server")
        .body(response.into())?)
}
