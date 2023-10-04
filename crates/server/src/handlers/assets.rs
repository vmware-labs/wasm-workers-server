// Copyright 2022-2023 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::AppData;
use actix_files::NamedFile;
use actix_web::{web::Data, HttpRequest};
use std::io::{Error, ErrorKind};
use std::path::{Component, Path};

/// Checks if the given component is not normal or absolute. If the component
/// is not valid, it returns true.
fn component_is_valid(component: Component) -> bool {
    matches!(component, Component::Normal(_) | Component::RootDir)
}

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

    // Parse the URI as a filesystem path
    let parsed_path = Path::new(uri_path);

    // Double-check the given path path does not contain any unexpected value.
    // It was previously sanitized, but this is a double check.
    if !parsed_path.components().all(component_is_valid) {
        return Err(Error::new(ErrorKind::NotFound, "The file is not present"));
    }

    let public_folder = root_path.join("public");

    // File path. This is required for the wasm_handler as dynamic routes may capture static files
    let file_path = public_folder.join(parsed_path);
    // A.k.a pretty urls. We may access /about and this matches to /about/index.html
    let index_folder_path = public_folder.join(parsed_path).join("index.html");

    if file_path.exists() {
        NamedFile::open_async(file_path).await
    } else if uri_path.ends_with('/') && index_folder_path.exists() {
        NamedFile::open_async(index_folder_path).await
    } else {
        Err(Error::new(ErrorKind::NotFound, "The file is not present"))
    }
}
