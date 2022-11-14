// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

mod config;
mod data;
mod router;
mod runner;

use actix_files::Files;
use actix_web::{
    http::StatusCode,
    middleware,
    web::{self, Bytes, Data},
    App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use clap::Parser;
use data::kv::KV;
use runner::WasmOutput;
use std::path::PathBuf;
use std::{collections::HashMap, sync::RwLock};

// Arguments
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Hostname to initiate the server
    #[clap(long = "host", default_value = "127.0.0.1")]
    hostname: String,

    /// Port to initiate the server
    #[clap(short, long, default_value_t = 8080)]
    port: u16,

    /// Folder to read WebAssembly modules from
    #[clap(value_parser, default_value = ".")]
    path: PathBuf,
}

// Common structures
struct Routes {
    routes: Vec<router::Route>,
}

struct DataConnectors {
    kv: KV,
}

async fn wasm_handler(req: HttpRequest, body: Bytes) -> HttpResponse {
    let routes = req.app_data::<Data<Routes>>().unwrap();
    let data_connectors = req
        .app_data::<Data<RwLock<DataConnectors>>>()
        .unwrap()
        .clone();
    // We will improve error handling
    let mut result: HttpResponse = HttpResponse::ServiceUnavailable().body("Error");

    for route in routes.routes.iter() {
        if route.path == req.path() {
            let body_str = String::from_utf8(body.to_vec()).unwrap_or_else(|_| String::from(""));

            // Init KV
            let kv_namespace = match &route.config {
                Some(config) => config.data_kv_namespace(),
                None => None,
            };

            let store = match &kv_namespace {
                Some(namespace) => {
                    let connector = data_connectors.read().unwrap();
                    let kv_store = connector.kv.find_store(namespace);

                    kv_store.map(|store| store.clone())
                }
                None => None,
            };

            let handler_result = route
                .runner
                .run(&runner::build_wasm_input(&req, body_str, store))
                .unwrap_or(WasmOutput {
                    body: String::from("<p>There was an error running this function</p>"),
                    headers: HashMap::from([("content-type".to_string(), "text/html".to_string())]),
                    status: StatusCode::SERVICE_UNAVAILABLE.as_u16(),
                    kv: HashMap::new(),
                });

            let mut builder = HttpResponse::build(
                StatusCode::from_u16(handler_result.status).unwrap_or(StatusCode::OK),
            );
            // Default content type
            builder.insert_header(("Content-Type", "text/html"));

            for (key, val) in handler_result.headers.iter() {
                // Note that QuickJS is replacing the "-" character
                // with "_" on property keys. Here, we rollback it
                builder.insert_header((key.replace('_', "-").as_str(), val.as_str()));
            }

            // Write to the state if required
            if kv_namespace.is_some() {
                data_connectors
                    .write()
                    .unwrap()
                    .kv
                    .replace_store(&kv_namespace.unwrap(), handler_result.kv)
            }

            result = builder.body(String::from(&handler_result.body))
        }
    }

    result
}

async fn debug(req: HttpRequest) -> impl Responder {
    let value = req.app_data::<Data<Routes>>().unwrap();
    HttpResponse::Ok().body(format!("Routes: {}", value.routes.len()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    println!("⚙️  Loading routes from: {}", &args.path.display());
    let routes = Data::new(Routes {
        routes: router::initialize_routes(&args.path),
    });

    let data = Data::new(RwLock::new(DataConnectors { kv: KV::new() }));

    println!("🗺  Detected routes:");
    for route in routes.routes.iter() {
        let default_name = String::from("default");
        let name = if let Some(config) = &route.config {
            config.name.as_ref().unwrap_or(&default_name)
        } else {
            &default_name
        };

        println!(
            "    - http://{}:{}{}\n      => {} (handler: {})",
            &args.hostname,
            args.port,
            route.path,
            route.handler.display(),
            name
        );
    }

    let server = HttpServer::new(move || {
        let mut app = App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            // Clean path before sending it to the service
            .wrap(middleware::NormalizePath::trim())
            .app_data(Data::clone(&routes))
            .app_data(Data::clone(&data))
            .service(web::resource("/_debug").to(debug));

        // Append routes to the current service
        for route in routes.routes.iter() {
            app = app.service(web::resource(&route.path).to(wasm_handler));

            // Configure KV
            if let Some(namespace) = route.config.as_ref().and_then(|c| c.data_kv_namespace()) {
                data.write().unwrap().kv.create_store(&namespace);
            }
        }

        // Serve static files from the static folder
        app = app.service(Files::new("/", &args.path.join("public")));

        app
    })
    .bind(format!("{}:{}", args.hostname.as_str(), args.port))?;

    println!(
        "🚀 Start serving requests at http://{}:{}\n",
        &args.hostname, args.port
    );

    server.run().await
}
