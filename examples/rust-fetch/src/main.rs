use anyhow::Result;
use serde::{Deserialize, Serialize};
use wasm_workers_rs::{
    bindings,
    http::{self, Request, Response},
    worker, Content,
};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Post {
    id: i32,
    title: String,
    body: String,
    user_id: i32,
}

#[worker]
fn handler(_req: Request<String>) -> Result<Response<Content>> {
    let external_request = Request::builder()
        .uri("https://jsonplaceholder.typicode.com/posts/1")
        .body(String::new())
        .unwrap();

    // Get the request
    let res = bindings::send_http_request(external_request).unwrap();

    // Parse the response
    let data = res.body();
    let data_str = String::from_utf8_lossy(&data);

    eprintln!("API response: {data_str}");

    let post: Post = serde_json::from_slice(&data).unwrap();

    // Applied changes here to use the Response method. This requires changes
    // on signature and how it returns the data.
    let response = format!(
        "<!DOCTYPE html>
<head>
    <title>Wasm Workers Server</title>
    <meta name=\"viewport\" content=\"width=device-width,initial-scale=1\">
    <meta charset=\"UTF-8\">
    <link rel=\"stylesheet\" href=\"https://cdn.jsdelivr.net/npm/water.css@2/out/water.css\">
    <style>
        body {{ max-width: 1000px; }}
        main {{ margin: 5rem 0; }}
        h1, p {{ text-align: center; }}
        h1 {{ margin-bottom: 2rem; }}
        pre {{ font-size: .9rem; }}
        pre > code {{ padding: 2rem; }}
        p {{ margin-top: 2rem; }}
    </style>
</head>
<body>
    <main>
        <h1>{}</h1>
        <p>{}</p>
    </main>
</body>",
        &post.title, &post.body
    );

    Ok(http::Response::builder()
        .status(200)
        .header("x-generated-by", "wasm-workers-server")
        .body(response.into())?)
}
