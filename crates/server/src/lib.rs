// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

mod errors;
use errors::{Result, ServeError};

mod handlers;

use actix_files::Files;
use actix_web::dev::{fn_service, Server, ServiceRequest, ServiceResponse};
use actix_web::{
    middleware,
    web::{self, Data},
    App, HttpServer,
};
use handlers::assets::handle_assets;
use handlers::not_found::handle_not_found;
use handlers::worker::handle_worker;
use std::{path::PathBuf, sync::RwLock};
use wws_api_manage::config_manage_api_handlers;
use wws_data_kv::KV;
use wws_panel::config_panel_handlers;
use wws_router::{Routes, WORKERS};

#[derive(Clone, PartialEq)]
pub enum Panel {
    Enabled,
    Disabled,
}

impl From<bool> for Panel {
    fn from(panel_enabled: bool) -> Self {
        if panel_enabled {
            Panel::Enabled
        } else {
            Panel::Disabled
        }
    }
}

#[derive(Default)]
pub(crate) struct DataConnectors {
    kv: KV,
}

#[derive(Clone)]
pub struct ServeOptions {
    pub root_path: PathBuf,
    pub base_routes: Routes,
    pub hostname: String,
    pub port: u16,
    pub panel: Panel,
    pub cors_origins: Option<Vec<String>>,
}

#[derive(Default)]
pub struct AppData {
    routes: Routes,
    root_path: PathBuf,
    cors_origins: Option<Vec<String>>,
}

impl From<ServeOptions> for AppData {
    fn from(serve_options: ServeOptions) -> Self {
        AppData {
            routes: serve_options.base_routes,
            root_path: serve_options.root_path.clone(),
            cors_origins: serve_options.cors_origins.clone(),
        }
    }
}

/// Initializes an actix-web server based on the given configuration and
/// path. It will configure the different handlers to manage static
/// assets and workers.
pub async fn serve(serve_options: ServeOptions) -> Result<Server> {
    // Initializes the data connectors. For now, just KV
    let data_connectors = Data::new(RwLock::new(DataConnectors::default()));

    let (hostname, port) = (serve_options.hostname.clone(), serve_options.port);
    let serve_options = serve_options.clone();

    let server = HttpServer::new(move || {
        // Initializes the app data for handlers
        let app_data: Data<AppData> = Data::new(
            <ServeOptions as TryInto<AppData>>::try_into(serve_options.clone())
                .expect("failed initializing server"),
        );

        let mut app = App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            // Clean path before sending it to the service
            .wrap(middleware::NormalizePath::trim())
            .app_data(Data::clone(&app_data))
            .app_data(Data::clone(&data_connectors));

        // Configure panel
        if serve_options.panel == Panel::Enabled {
            app = app.configure(config_panel_handlers);
            app = app.configure(config_manage_api_handlers);
        }

        let workers = WORKERS
            .read()
            .expect("error locking worker lock for reading");

        // Append routes to the current service
        for route in app_data.routes.iter() {
            app = app.service(web::resource(route.actix_path()).to(handle_worker));

            let worker = workers
                .get(&route.worker)
                .expect("unexpected missing worker");

            // Configure KV
            if let Some(namespace) = worker.config.data_kv_namespace() {
                data_connectors
                    .write()
                    .expect("cannot retrieve shared data")
                    .kv
                    .create_store(&namespace);
            }
        }

        // Serve static files from the static folder
        let mut static_prefix = app_data.routes.prefix.clone();
        if static_prefix.is_empty() {
            static_prefix = String::from("/");
        }

        app = app.service(
            Files::new(&static_prefix, app_data.root_path.join("public"))
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
    .bind(format!("{}:{}", hostname, port))
    .map_err(|_| ServeError::InitializeServerError)?;

    Ok(server.run())
}
