// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use super::{assets::handle_assets, not_found::handle_not_found};
use crate::{AppData, DataConnectors};
use actix_web::{
    http::StatusCode,
    web::{Bytes, Data},
    HttpRequest, HttpResponse,
};
use std::sync::RwLock;
use wws_router::WORKERS;
use wws_worker::io::WasmOutput;

const CORS_HEADER: &str = "Access-Control-Allow-Origin";

/// Process an HTTP request by passing it to the right Runner. The Runner
/// will prepare the WASI environment and call the Wasm module with the data.
///
/// Note that here we have to select the runner by checking the path. This
/// responsibility is duplicated as Actix already maps paths to handlers.
/// However, there are several reasons why this is reasonable in this project:
///
/// - We fully control collisions when a request can be served by a parametrized
///   route. Actix will reply with the first handler that matches. However, users
///   cannot set the handlers manually, so we want to ensure a consistent behavior
/// - To map an actix path with a runner, we need to create factory service. Note
///   that Actix will create an instance per thread (worker), so Runners cannot be
///   shared. This will require multiple instances of the same Wasm module, so
///   the resource consumption will be increased.
///
/// For these reasons, we are selecting the right handler at this point and not
/// allowing Actix to select it for us.
pub async fn handle_worker(req: HttpRequest, body: Bytes) -> HttpResponse {
    let app_data = req
        .app_data::<Data<AppData>>()
        .expect("error fetching app data");
    let data_connectors = req
        .app_data::<Data<RwLock<DataConnectors>>>()
        .expect("error fetching data connectors");
    let result: HttpResponse;

    // First, we need to identify the best suited route
    let selected_route = app_data.routes.retrieve_best_route(req.path());
    if let Some(route) = selected_route {
        // First, check if there's an existing static file. Static assets have more priority
        // than dynamic routes. However, I cannot set the static assets as the first service
        // as it's captures everything.
        if route.is_dynamic() {
            if let Ok(existing_file) = handle_assets(&req).await {
                return existing_file.into_response(&req);
            }
        }

        let workers = WORKERS
            .read()
            .expect("error locking worker lock for reading");

        let worker = workers
            .get(&route.worker)
            .expect("unexpected missing worker");

        // Let's continue
        let body_str = String::from_utf8(body.to_vec()).unwrap_or_else(|_| String::from(""));

        // Init from configuration
        let vars = &worker.config.vars;
        let kv_namespace = worker.config.data_kv_namespace();

        let store = match &kv_namespace {
            Some(namespace) => {
                let connector = data_connectors
                    .read()
                    .expect("error locking data connectors lock for reading");
                let kv_store = connector.kv.find_store(namespace);

                kv_store.map(|store| store.clone())
            }
            None => None,
        };

        let (handler_result, handler_success) = match worker.run(&req, &body_str, store, vars) {
            Ok(output) => (output, true),
            Err(_) => (WasmOutput::failed(), false),
        };

        let mut builder = HttpResponse::build(
            StatusCode::from_u16(handler_result.status).unwrap_or(StatusCode::OK),
        );
        // Default content type
        builder.insert_header(("Content-Type", "text/html"));

        // Check if cors config has any origins to register
        if let Some(origins) = app_data.cors_origins.as_ref() {
            // Check if worker has overridden the header, if not
            if !handler_result.headers.contains_key(CORS_HEADER) {
                // insert those origins in 'Access-Control-Allow-Origin' header
                let header_value = origins.join(",");
                builder.insert_header((CORS_HEADER, header_value));
            }
        }

        for (key, val) in handler_result.headers.iter() {
            // Note that QuickJS is replacing the "-" character
            // with "_" on property keys. Here, we rollback it
            builder.insert_header((key.replace('_', "-").as_str(), val.as_str()));
        }

        // Write to the state if required
        if handler_success && kv_namespace.is_some() {
            data_connectors
                .write()
                .expect("error locking data connectors lock for writing")
                .kv
                .replace_store(&kv_namespace.unwrap(), &handler_result.kv)
        }

        result = match handler_result.body() {
            Ok(res) => builder.body(res),
            Err(_) => {
                HttpResponse::ServiceUnavailable().body("There was an error running the worker")
            }
        }
    } else {
        result = handle_not_found(&req).await;
    }

    result
}
