// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

mod handlers;

use actix_files::Files;
use actix_web::dev::{fn_service, Server, ServiceRequest, ServiceResponse};
use actix_web::{
    middleware,
    web::{self, Data},
    App, HttpServer,
};
use anyhow::Result;
use handlers::assets::handle_assets;
use handlers::not_found::handle_not_found;
use handlers::worker::handle_worker;
use std::fs::OpenOptions;
use std::path::Path;
use std::sync::RwLock;
use wws_api_manage::config_manage_api_handlers;
use wws_data_kv::KV;
use wws_panel::config_panel_handlers;
use wws_router::Routes;

#[derive(Default)]
pub(crate) struct DataConnectors {
    kv: KV,
}

/// Initializes an actix-web server based on the given configuration and
/// path. It will configure the different handlers to manage static
/// assets and workers.
pub async fn serve(
    root_path: &Path,
    base_routes: Routes,
    hostname: &str,
    port: u16,
    panel: bool,
    stderr: Option<&Path>,
    cors_origins: Option<Vec<String>>,
) -> Result<Server> {
    // Initializes the data connectors. For now, just KV
    let data = Data::new(RwLock::new(DataConnectors::default()));
    let routes = Data::new(base_routes);
    let root_path = Data::new(root_path.to_owned());
    let stderr_file;

    // Configure stderr
    if let Some(path) = stderr {
        let file = OpenOptions::new().read(true).write(true).open(path)?;

        stderr_file = Data::new(Some(file));
    } else {
        stderr_file = Data::new(None);
    }

    let cors_data = Data::new(cors_origins);

    let server = HttpServer::new(move || {
        let mut app = App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            // Clean path before sending it to the service
            .wrap(middleware::NormalizePath::trim())
            .app_data(Data::clone(&routes))
            .app_data(Data::clone(&data))
            .app_data(Data::clone(&root_path))
            .app_data(Data::clone(&stderr_file))
            .app_data(Data::clone(&cors_data));

        // Configure panel
        if panel {
            app = app.configure(config_panel_handlers);
            app = app.configure(config_manage_api_handlers);
        }

        // Append routes to the current service
        for route in routes.routes.iter() {
            app = app.service(web::resource(route.actix_path()).to(handle_worker));

            // Configure KV
            if let Some(namespace) = route.worker.config.data_kv_namespace() {
                data.write().unwrap().kv.create_store(&namespace);
            }
        }

        // Serve static files from the static folder
        let mut static_prefix = routes.prefix.clone();
        if static_prefix.is_empty() {
            static_prefix = String::from("/");
        }

        app = app.service(
            Files::new(&static_prefix, root_path.join("public"))
                .index_file("index.html")
                // This handler check if there's an HTML file in the public folder that
                // can reply to the given request. For example, if someone request /about,
                // this handler will look for a /public/about.html file.
                .default_handler(fn_service(|req: ServiceRequest| async {
                    let (req, _) = req.into_parts();

                    match handle_assets(&req).await {
                        Ok(existing_file) => {
                            let res = existing_file.into_response(&req);
                            Ok(ServiceResponse::new(req, res))
                        }
                        Err(_) => {
                            let res = handle_not_found(&req).await;
                            Ok(ServiceResponse::new(req, res))
                        }
                    }
                })),
        );

        app
    })
    .bind(format!("{}:{}", hostname, port))?;

    Ok(server.run())
}
