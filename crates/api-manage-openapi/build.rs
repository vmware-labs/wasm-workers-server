// Copyright 2023 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

// Trick to generate the OpenAPI spec on build time.
// See: https://github.com/juhaku/utoipa/issues/214#issuecomment-1179589373

use std::fs;
use utoipa::OpenApi;
use wws_api_manage::ApiDoc;

fn main() {
    let spec = ApiDoc::openapi().to_pretty_json().unwrap();
    fs::write("./src/openapi.json", spec).expect("Error writing the OpenAPI documentation");
}
