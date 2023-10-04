// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::AppData;
use actix_files::NamedFile;
use actix_web::{web::Data, HttpRequest};
use std::io::{Error, ErrorKind};

/// Find a static HTML file in the `public` folder. This function is used
/// when there's no direct file to be served. It will look for certain patterns
/// like "public/{uri}/index.html" and "public/{uri}.html".
///
/// If no file is present, it will try to get a default "public/404.html"
pub async fn handle_assets(req: &HttpRequest) -> Result<NamedFile, Error> {
    let root_path = &req
        .app_data::<Data<AppData>>()
        .expect("error fetching app data")
        .root_path;
    let uri_path = req.path();

    // File path. This is required for the wasm_handler as dynamic routes may capture static files
    let file_path = root_path.join(format!("public{uri_path}"));
    // A.k.a pretty urls. We may access /about and this matches to /about/index.html
    let index_folder_path = root_path.join(format!("public{uri_path}/index.html"));
    // Same as before, but the file is located at ./about.html
    let html_ext_path = root_path.join(format!("public{uri_path}.html"));

    if file_path.exists() {
        NamedFile::open_async(file_path).await
    } else if uri_path.ends_with('/') && index_folder_path.exists() {
        NamedFile::open_async(index_folder_path).await
    } else if !uri_path.ends_with('/') && html_ext_path.exists() {
        NamedFile::open_async(html_ext_path).await
    } else {
        Err(Error::new(ErrorKind::NotFound, "The file is not present"))
    }
}
