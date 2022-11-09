// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

//
// Declare the different routes for the project
// based on the files in the given folder
//
use crate::config::Config;
use crate::runner::Runner;
use glob::glob;
use std::fs;
use std::path::{Path, PathBuf};

/// An existing route in the project. It contains a reference to the handler, the URL path,
/// the runner and configuration. Note that URL paths are calculated based on the file path.
///
/// # Examples
///
/// ```
/// index.wasm          =>  /
/// api/index.wasm      =>  /api
/// api/v2/ping.wasm    =>  /api/v2/ping
/// ```
#[derive(Clone)]
pub struct Route {
    /// The wasm module that will manage the route
    pub handler: PathBuf,
    /// The URL path
    pub path: String,
    /// The preconfigured runner
    pub runner: Runner,
    /// The associated configuration if available
    pub config: Option<Config>,
}

impl Route {
    /// Initialize a new route from the given folder and filepath. It will calculate the
    /// proper URL path based on the filename.
    ///
    /// This method also initializes the Runner and loads the Config if available.
    fn new(base_path: &Path, filepath: PathBuf) -> Self {
        let runner = Runner::new(&filepath).unwrap();
        // Load configuration
        let mut config_path = filepath.clone();
        config_path.set_extension("toml");
        let mut config = None::<Config>;

        if fs::metadata(&config_path).is_ok() {
            match Config::try_from_file(config_path) {
                Ok(c) => config = Some(c),
                Err(err) => {
                    eprintln!("{}", err);
                }
            }
        }

        Self {
            path: Self::retrieve_route(base_path, &filepath),
            handler: filepath,
            runner,
            config,
        }
    }

    // Process the given path to return the proper route for the API.
    // It will transform paths like test/index.wasm into /test.
    fn retrieve_route(base_path: &Path, path: &Path) -> String {
        // TODO: Improve this entire method
        // @ref #13
        if let Some(api_path) = path.to_str() {
            let parsed_path: String = api_path
                .to_string()
                .replace(".wasm", "")
                .replace(".js", "")
                .replace(base_path.to_str().unwrap_or("./"), "");
            let mut normalized = String::from("/") + &parsed_path.replace("index", "");

            // Remove trailing / to avoid 404 errors
            if normalized.ends_with('/') && normalized.len() > 1 {
                normalized.pop();
            }

            normalized
        } else {
            // TODO: Manage better unexpected characters in paths
            String::from(path.to_str().unwrap_or("/unknown"))
        }
    }
}

/// Initialize the list of routes from the given folder. This method will look for
/// all `**/*.wasm` files and will create the associated routes. This routing approach
/// is pretty popular in web development and static sites.
pub fn initialize_routes(base_path: &Path) -> Vec<Route> {
    let mut routes = Vec::new();
    let path = Path::new(&base_path);

    // Items to iterate over
    let glob_items = glob(path.join("**/*.wasm").as_os_str().to_str().unwrap())
        .expect("Failed to read the current directory")
        .chain(
            glob(path.join("**/*.js").as_os_str().to_str().unwrap())
                .expect("Failed to read the current directory"),
        );

    for entry in glob_items {
        match entry {
            Ok(filepath) => {
                routes.push(Route::new(base_path, filepath));
            }
            Err(e) => println!("Could not read the file {:?}", e),
        }
    }

    routes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unix_route_index_path_retrieval() {
        let check_route = |path: &str, expected_route: &str| {
            assert_eq!(
                Route::retrieve_route(&Path::new("."), &PathBuf::from(path)),
                String::from(expected_route),
            )
        };

        // In a subfolder
        check_route("examples/index.js", "/examples");
        check_route("examples/index.wasm", "/examples");

        // Multiple levels
        check_route("examples/api/index.js", "/examples/api");
        check_route("examples/api/index.wasm", "/examples/api");

        // Root
        check_route("index.js", "/");
        check_route("index.wasm", "/");
    }

    #[test]
    fn unix_route_path_retrieval() {
        let check_route = |path: &str, expected_route: &str| {
            assert_eq!(
                Route::retrieve_route(&Path::new("."), &PathBuf::from(path)),
                String::from(expected_route),
            )
        };

        // In a subfolder
        check_route("examples/handler.js", "/examples/handler");
        check_route("examples/handler.wasm", "/examples/handler");

        // Multiple levels
        check_route("examples/api/handler.js", "/examples/api/handler");
        check_route("examples/api/handler.wasm", "/examples/api/handler");

        // Root
        check_route("handler.js", "/handler");
        check_route("handler.wasm", "/handler");
    }
}
