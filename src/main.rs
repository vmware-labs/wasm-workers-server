// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

#[macro_use]
extern crate lazy_static;

mod commands;
mod config;
mod data;
mod fetch;
mod router;
mod runtimes;
mod store;
mod workers;

use crate::config::Config;
use actix_files::{Files, NamedFile};
use actix_web::dev::{fn_service, ServiceRequest, ServiceResponse};
use actix_web::{
    http::StatusCode,
    middleware,
    web::{self, Bytes, Data},
    App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use clap::Parser;
use commands::main::Main;
use commands::runtimes::RuntimesCommands;
use data::kv::KV;
use router::Routes;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use std::process::exit;
use std::{collections::HashMap, sync::RwLock};
use workers::wasm_io::WasmOutput;

// Provide a static root_path so it can be used in the default_worker to manage
// static assets.
lazy_static! {
    static ref ARGS: Args = Args::parse();
    static ref ROOT_PATH: PathBuf = ARGS.path.clone();
}

// Arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Hostname to initiate the server
    #[arg(long = "host", default_value = "127.0.0.1")]
    hostname: String,

    /// Port to initiate the server
    #[arg(short, long, default_value_t = 8080)]
    port: u16,

    /// Folder to read WebAssembly modules from
    #[arg(value_parser, default_value = ".")]
    path: PathBuf,

    /// Prepend the given path to all URLs
    #[arg(long, default_value = "")]
    prefix: String,

    /// Manage language runtimes in your project
    #[command(subcommand)]
    commands: Option<Main>,
}

struct DataConnectors {
    kv: KV,
}

/// This method tries to render a custom 404 error file from the static
/// folder. If not, it will render an empty 404
async fn not_found_html(req: &HttpRequest) -> HttpResponse {
    let public_404_path = ROOT_PATH.join("public").join("404.html");

    if let Ok(file) = NamedFile::open_async(public_404_path).await {
        file.into_response(req)
    } else {
        HttpResponse::NotFound().body("")
    }
}

/// Find a static HTML file in the `public` folder. This function is used
/// when there's no direct file to be served. It will look for certain patterns
/// like "public/{uri}/index.html" and "public/{uri}.html".
///
/// If no file is present, it will try to get a default "public/404.html"
async fn find_static_html(uri_path: &str) -> Result<NamedFile, Error> {
    // File path. This is required for the wasm_handler as dynamic routes may capture static files
    let file_path = ROOT_PATH.join(format!("public{uri_path}"));
    // A.k.a pretty urls. We may access /about and this matches to /about/index.html
    let index_folder_path = ROOT_PATH.join(format!("public{uri_path}/index.html"));
    // Same as before, but the file is located at ./about.html
    let html_ext_path = ROOT_PATH.join(format!("public{uri_path}.html"));

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
async fn wasm_handler(req: HttpRequest, body: Bytes) -> HttpResponse {
    let routes = req.app_data::<Data<Routes>>().unwrap();
    let data_connectors = req
        .app_data::<Data<RwLock<DataConnectors>>>()
        .unwrap()
        .clone();
    // We will improve error handling
    let result: HttpResponse;

    // First, we need to identify the best suited route
    let selected_route = routes.retrieve_best_route(req.path());

    if let Some(route) = selected_route {
        // First, check if there's an existing static file. Static assets have more priority
        // than dynamic routes. However, I cannot set the static assets as the first service
        // as it's captures everything.
        if route.is_dynamic() {
            if let Ok(existing_file) = find_static_html(req.path()).await {
                return existing_file.into_response(&req);
            }
        }

        // Let's continue
        let body_str = String::from_utf8(body.to_vec()).unwrap_or_else(|_| String::from(""));

        // Init from configuration
        let empty_hash = HashMap::new();
        let mut vars = &empty_hash;
        let mut kv_namespace = None;

        match &route.config {
            Some(config) => {
                kv_namespace = config.data_kv_namespace();
                vars = &config.vars;
            }
            None => {}
        };

        let store = match &kv_namespace {
            Some(namespace) => {
                let connector = data_connectors.read().unwrap();
                let kv_store = connector.kv.find_store(namespace);

                kv_store.map(|store| store.clone())
            }
            None => None,
        };

        let (handler_result, handler_success) = match route.worker.run(&req, &body_str, store, vars)
        {
            Ok(output) => (output, true),
            Err(_) => (WasmOutput::failed(), false),
        };

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
        if handler_success && kv_namespace.is_some() {
            data_connectors
                .write()
                .unwrap()
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
        result = not_found_html(&req).await;
    }

    result
}

async fn debug(req: HttpRequest) -> impl Responder {
    let value = req.app_data::<Data<Routes>>().unwrap();
    HttpResponse::Ok().body(format!("Routes: {}", value.routes.len()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = &*ARGS;

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // Check the given subcommand
    if let Some(Main::Runtimes(sub)) = &args.commands {
        match &sub.runtime_commands {
            RuntimesCommands::List(list) => {
                if let Err(err) = list.run(sub).await {
                    println!("❌ There was an error listing the runtimes from the repository");
                    println!("👉 {err}");
                    exit(1);
                }
            }
            RuntimesCommands::Install(install) => {
                if let Err(err) = install.run(&args.path, sub).await {
                    println!("❌ There was an error installing the runtime from the repository");
                    println!("👉 {err}");
                    exit(1);
                }
            }
            RuntimesCommands::Uninstall(uninstall) => {
                if let Err(err) = uninstall.run(&args.path, sub) {
                    println!("❌ There was an error uninstalling the runtime");
                    println!("👉 {err}");
                    exit(1);
                }
            }
            RuntimesCommands::Check(check) => {
                if let Err(err) = check.run(&args.path) {
                    println!("❌ There was an error checking the local runtimes");
                    println!("👉 {err}");
                    exit(1);
                }
            }
        };

        Ok(())
    } else {
        // TODO(Angelmmiguel): refactor this into a separate command!
        // Initialize the routes

        // Loading the local configuration if available.
        let config = match Config::load(&args.path) {
            Ok(c) => c,
            Err(err) => {
                println!("⚠️  There was an error reading the .wws.toml file. It will be ignored");
                println!("⚠️  Error: {err}");

                Config::default()
            }
        };

        println!("⚙️  Loading routes from: {}", &args.path.display());
        let routes = Data::new(Routes::new(&args.path, &args.prefix, &config));

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
                "    - http://{}:{}{}\n      => {} (name: {})",
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
                app = app.service(web::resource(route.actix_path()).to(wasm_handler));

                // Configure KV
                if let Some(namespace) = route.config.as_ref().and_then(|c| c.data_kv_namespace()) {
                    data.write().unwrap().kv.create_store(&namespace);
                }
            }

            // Serve static files from the static folder
            let mut static_prefix = routes.prefix.clone();
            if static_prefix.is_empty() {
                static_prefix = String::from("/");
            }

            app = app.service(
                Files::new(&static_prefix, args.path.join("public"))
                    .index_file("index.html")
                    // This handler check if there's an HTML file in the public folder that
                    // can reply to the given request. For example, if someone request /about,
                    // this handler will look for a /public/about.html file.
                    .default_handler(fn_service(|req: ServiceRequest| async {
                        let (req, _) = req.into_parts();

                        match find_static_html(req.path()).await {
                            Ok(existing_file) => {
                                let res = existing_file.into_response(&req);
                                Ok(ServiceResponse::new(req, res))
                            }
                            Err(_) => {
                                let res = not_found_html(&req).await;
                                Ok(ServiceResponse::new(req, res))
                            }
                        }
                    })),
            );

            app
        })
        .bind(format!("{}:{}", args.hostname.as_str(), args.port))?;

        println!(
            "🚀 Start serving requests at http://{}:{}\n",
            &args.hostname, args.port
        );

        server.run().await
    }
}
