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

/// Find the static assets of the administration panel
#[actix_web::get("/_panel{_:.*}")]
pub async fn handle_static_panel(path: web::Path<String>) -> impl Responder {
    let path = if path.len() == 0 {
        "index.html"
    } else {
        path.as_str().strip_prefix('/').unwrap()
    };

    match Asset::get(path) {
        Some(content) => HttpResponse::Ok()
            .content_type(from_path(path).first_or_octet_stream().as_ref())
            .body(content.data.into_owned()),
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}
