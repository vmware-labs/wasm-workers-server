// Copyright 2022-2023 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

mod errors;
mod handlers;
mod static_assets;

use actix_web::dev::Server;
use actix_web::{
    middleware,
    web::{self, Data},
    App, HttpServer,
};
use errors::{Result, ServeError};
use handlers::assets::handle_assets;
use handlers::worker::handle_worker;
use static_assets::StaticAssets;
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
    let mut data = DataConnectors::default();

    let (hostname, port) = (serve_options.hostname.clone(), serve_options.port);
    let serve_options = serve_options.clone();

    let workers = WORKERS
        .read()
        .expect("error locking worker lock for reading");

    // Configure the KV store when required
    for route in serve_options.base_routes.routes.iter() {
        let worker = workers
            .get(&route.worker)
            .expect("unexpected missing worker");

        // Configure KV
        if let Some(namespace) = worker.config.data_kv_namespace() {
            data.kv.create_store(&namespace);
        }
    }

    // Pre-create the KV namespaces
    let data_connectors = Data::new(RwLock::new(data));

    // Static assets
    let mut static_assets =
        StaticAssets::new(&serve_options.root_path, &serve_options.base_routes.prefix);
    static_assets
        .load()
        .expect("Error loading the static assets");

    // Build the actix server with all the configuration
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

        // Mount static assets
        for actix_path in static_assets.paths.iter() {
            app = app.route(actix_path, web::get().to(handle_assets));
        }

        // Default all other routes to the Wasm handler
        app = app.default_service(web::route().to(handle_worker));

        app
    })
    .bind(format!("{}:{}", hostname, port))
    .map_err(|_| ServeError::InitializeServerError)?;

    Ok(server.run())
}
