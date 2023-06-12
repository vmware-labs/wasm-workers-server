// Copyright 2023 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

mod handlers;
mod models;

use actix_web::web::ServiceConfig;
use models::Worker;
use utoipa::OpenApi;

/// Add the administration panel HTTP handlers to an existing
/// Actix application.
pub fn config_manage_api_handlers(cfg: &mut ServiceConfig) {
    cfg.service(handlers::v0::workers::handle_api_workers);
    cfg.service(handlers::v0::workers::handle_api_worker);
}

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Wasm Workers Server Management API",
        description = "Exposes methods to read current workers, services and to configure and run projects",
        license(
            name = "Apache 2.0",
            url = "https://github.com/vmware-labs/wasm-workers-server/blob/main/LICENSE"
        ),
        contact(),
        version = "1"
    ),
    paths(
        handlers::v0::workers::handle_api_workers,
        handlers::v0::workers::handle_api_worker
    ),
    components(schemas(Worker))
)]
pub struct ApiDoc;
