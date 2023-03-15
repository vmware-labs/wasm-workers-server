// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use actix_files::NamedFile;
use actix_web::{web::Data, HttpRequest, HttpResponse};
use std::path::Path;

/// This method tries to render a custom 404 error file from the static
/// folder. If not, it will render an empty 404
pub async fn handle_not_found(req: &HttpRequest) -> HttpResponse {
    let root_path = req.app_data::<Data<&Path>>().unwrap();
    let public_404_path = root_path.join("public").join("404.html");

    if let Ok(file) = NamedFile::open_async(public_404_path).await {
        file.into_response(req)
    } else {
        HttpResponse::NotFound().body("")
    }
}
