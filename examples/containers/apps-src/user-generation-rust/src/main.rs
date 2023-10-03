use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;

use wasm_workers_rs::{
    bindings,
    http::{Request, Response},
    worker, Cache, Content,
};

#[derive(Deserialize, Serialize)]
struct User {
    first_name: String,
    last_name: String,
    username: String,
    email: String,
}

#[derive(Deserialize, Serialize)]
struct ResponseData {
    user: User,
    some_file_contents: String,
    generated_users: u32,
}

const GENERATED_USERS_COUNTER: &str = "generated_users_counter";

#[worker(cache)]
fn reply(_req: Request<String>, cache: &mut Cache) -> Result<Response<Content>> {
    let external_request = Request::builder()
        .uri("https://random-data-api.com/api/v2/users")
        .body(String::new())
        .map_err(|err| anyhow!("could not build request: {:?}", err))?;
    let response = bindings::send_http_request(external_request)
        .map_err(|err| anyhow!("could not fetch data from remote service: {:?}", err))?;
    let user: User = serde_json::from_slice(response.body())
        .map_err(|err| anyhow!("invalid data returned by remote service: {:?}", err))?;

    let generated_users_counter = match cache.get(GENERATED_USERS_COUNTER) {
        Some(counter) => counter.parse::<u32>().unwrap_or(0),
        None => 0,
    } + 1;

    cache.insert(GENERATED_USERS_COUNTER.to_string(), generated_users_counter.to_string());

    let response = ResponseData {
        user,
        some_file_contents: read_to_string("/tmp/file.txt")?,
        generated_users: generated_users_counter,
    };

    Ok(Response::builder()
        .status(200)
        .header("x-generated-by", "wasm-workers-server")
        .body(
            serde_json::to_string(&response)
                .map_err(|err| anyhow!("could not marshal result: {:?}", err))?
                .into(),
        )
        .map_err(|err| anyhow!("could not retrieve remote service result: {:?}", err))?)
}
