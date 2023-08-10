// Copyright 2023 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use actix_web::{
    web::{self},
    HttpResponse, Responder,
};
use mime_guess::from_path;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "client/dist/"]
struct Asset;

#[actix_web::get("/_panel{_:.*}")]
pub async fn handle_static_panel(path: web::Path<String>) -> impl Responder {
    let path = if path.is_empty() {
        "index.html"
    } else {
        path.as_str().strip_prefix('/').unwrap()
    };

    let (content_type, content_data) = Asset::get(path)
        .map(|content| {
            (
                from_path(path).first_or_octet_stream().to_string(),
                content.data.into_owned(),
            )
        })
        .unwrap_or_else(|| {
            let default_content = Asset::get("index.html").unwrap();
            (
                "text/html; charset=utf-8".to_string(),
                default_content.data.into_owned(),
            )
        });

    HttpResponse::Ok()
        .content_type(content_type)
        .body(content_data)
}
