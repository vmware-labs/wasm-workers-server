// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

//
// Declare the different routes for the project
// based on the files in the given folder
//
use crate::config::Config;
use crate::runner::Runner;
use glob::glob;
use std::ffi::OsStr;
use std::fs;
use std::path::{Component, Path, PathBuf};

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
    fn new(base_path: &Path, filepath: PathBuf, prefix: &str) -> Self {
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
            path: Self::retrieve_route(base_path, &filepath, prefix),
            handler: filepath,
            runner,
            config,
        }
    }

    // Process the given path to return the proper route for the API.
    // It will transform paths like test/index.wasm into /test.
    fn retrieve_route(base_path: &Path, path: &Path, prefix: &str) -> String {
        // Normalize both paths
        let n_path = Self::normalize_path_to_url(path);
        let n_base_path = Self::normalize_path_to_url(base_path);

        // Remove the base_path
        match n_path.strip_prefix(&n_base_path) {
            Some(worker_path) => {
                String::from(prefix)
                    + (if worker_path.is_empty() {
                        // Index file at root
                        "/"
                    } else {
                        worker_path
                    })
            }
            None => {
                // TODO: manage errors properly and skip the route
                // @see #13
                String::from("/unknown")
            }
        }
    }

    // Prepare a path to be used as an URL. This method performs 3 main actions:
    //
    // - Remove file extension
    // - Keep only "normal" components. Others like "." or "./" are ignored
    // - Remove "index" components
    pub fn normalize_path_to_url(path: &Path) -> String {
        path.with_extension("")
            .components()
            .filter_map(|c| match c {
                Component::Normal(os_str) if os_str != OsStr::new("index") => os_str
                    .to_str()
                    .map(|parsed_str| String::from("/") + parsed_str),
                _ => None,
            })
            .collect()
    }
}

/// Initialize the list of routes from the given folder. This method will look for
/// all `**/*.wasm` files and will create the associated routes. This routing approach
/// is pretty popular in web development and static sites.
pub fn initialize_routes(base_path: &Path, prefix: &str) -> Vec<Route> {
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
            // Avoid loading static assets
            Ok(filepath) if !is_in_public_folder(&filepath) => {
                routes.push(Route::new(base_path, filepath, prefix));
            }
            Err(e) => println!("Could not read the file {:?}", e),
            _ => {}
        }
    }

    routes
}

/// Checks if the given filepath is inside the "public" folder
fn is_in_public_folder(path: &Path) -> bool {
    path.components().any(|c| match c {
        Component::Normal(os_str) => os_str == OsStr::new("public"),
        _ => false,
    })
}

/// Defines a prefix in the context of the application.
/// This prefix will be used for the static assets and the
/// handlers.
///
/// A prefix must have the format: /X. This method receives
/// the optional prefix and returns a proper String.
///
/// To be flexible, the method will manage "windows" paths too:
/// \app. This shouldn't be considered as "prefix" must be an URI
/// path. However, the check is pretty simple, so we will consider
/// it.
pub fn format_prefix(source: &str) -> String {
    let mut normalized_prefix = source.to_string();
    // Ensure the prefix doesn't include any \ character
    normalized_prefix = normalized_prefix.replace('\\', "/");

    if normalized_prefix.is_empty() {
        normalized_prefix
    } else {
        if !normalized_prefix.starts_with('/') {
            normalized_prefix = String::from('/') + &normalized_prefix;
        }

        if normalized_prefix.ends_with('/') {
            normalized_prefix.pop();
        }

        normalized_prefix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(not(target_os = "windows"))]
    #[test]
    fn unix_route_index_path_retrieval() {
        let tests = [
            // In a subfolder
            (".", "examples/index.js", "/examples"),
            (".", "examples/index.wasm", "/examples"),
            // Multiple levels
            (".", "examples/api/index.js", "/examples/api"),
            (".", "examples/api/index.wasm", "/examples/api"),
            // Root
            (".", "index.js", "/"),
            (".", "index.wasm", "/"),
            // Now, with a different root
            ("./root", "root/examples/index.js", "/examples"),
            ("./root", "root/examples/index.wasm", "/examples"),
            ("./root", "root/examples/api/index.js", "/examples/api"),
            ("./root", "root/examples/api/index.wasm", "/examples/api"),
            ("./root", "root/index.js", "/"),
            ("./root", "root/index.wasm", "/"),
            // A backslash should not change anything
            ("./root/", "root/examples/index.js", "/examples"),
            ("./root/", "root/examples/index.wasm", "/examples"),
            ("./root/", "root/examples/api/index.js", "/examples/api"),
            ("./root/", "root/examples/api/index.wasm", "/examples/api"),
            ("./root/", "root/index.js", "/"),
            ("./root/", "root/index.wasm", "/"),
        ];

        for t in tests {
            assert_eq!(
                Route::retrieve_route(Path::new(t.0), &PathBuf::from(t.1), ""),
                String::from(t.2),
            )
        }
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn win_route_index_path_retrieval() {
        let tests = [
            // In a subfolder
            (".", "examples\\index.js", "/examples"),
            (".", "examples\\index.wasm", "/examples"),
            // Multiple levels
            (".", "examples\\api\\index.js", "/examples/api"),
            (".", "examples\\api\\index.wasm", "/examples/api"),
            // Root
            (".", "index.js", "/"),
            (".", "index.wasm", "/"),
            // Now, with a different root
            (".\\root", "root\\examples\\index.js", "/examples"),
            (".\\root", "root\\examples\\index.wasm", "/examples"),
            (".\\root", "root\\examples\\api\\index.js", "/examples/api"),
            (
                ".\\root",
                "root\\examples\\api\\index.wasm",
                "/examples/api",
            ),
            (".\\root", "root\\index.js", "/"),
            (".\\root", "root\\index.wasm", "/"),
            // A backslash should not change anything
            (".\\root\\", "root\\examples\\index.js", "/examples"),
            (".\\root\\", "root\\examples\\index.wasm", "/examples"),
            (
                ".\\root\\",
                "root\\examples\\api\\index.js",
                "/examples/api",
            ),
            (
                ".\\root\\",
                "root\\examples\\api\\index.wasm",
                "/examples/api",
            ),
            (".\\root\\", "root\\index.js", "/"),
            (".\\root\\", "root\\index.wasm", "/"),
        ];

        for t in tests {
            assert_eq!(
                Route::retrieve_route(&Path::new(t.0), &PathBuf::from(t.1), ""),
                String::from(t.2),
            )
        }
    }

    #[cfg(not(target_os = "windows"))]
    #[test]
    fn unix_route_path_retrieval() {
        let tests = [
            // In a subfolder
            (".", "examples/handler.js", "/examples/handler"),
            (".", "examples/handler.wasm", "/examples/handler"),
            // Multiple levels
            (".", "examples/api/handler.js", "/examples/api/handler"),
            (".", "examples/api/handler.wasm", "/examples/api/handler"),
            // Root
            (".", "handler.js", "/handler"),
            (".", "handler.wasm", "/handler"),
            // Now, with a different root
            ("./root", "root/examples/handler.js", "/examples/handler"),
            ("./root", "root/examples/handler.wasm", "/examples/handler"),
            (
                "./root",
                "root/examples/api/handler.js",
                "/examples/api/handler",
            ),
            (
                "./root",
                "root/examples/api/handler.wasm",
                "/examples/api/handler",
            ),
            ("./root", "root/handler.js", "/handler"),
            ("./root", "root/handler.wasm", "/handler"),
            // A backslash should not change anything
            ("./root/", "root/examples/handler.js", "/examples/handler"),
            ("./root/", "root/examples/handler.wasm", "/examples/handler"),
            (
                "./root/",
                "root/examples/api/handler.js",
                "/examples/api/handler",
            ),
            (
                "./root/",
                "root/examples/api/handler.wasm",
                "/examples/api/handler",
            ),
            ("./root/", "root/handler.js", "/handler"),
            ("./root/", "root/handler.wasm", "/handler"),
        ];

        for t in tests {
            assert_eq!(
                Route::retrieve_route(Path::new(t.0), &PathBuf::from(t.1), ""),
                String::from(t.2),
            )
        }
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn win_route_path_retrieval() {
        let tests = [
            // In a subfolder
            (".", "examples/handler.js", "/examples/handler"),
            (".", "examples/handler.wasm", "/examples/handler"),
            // Multiple levels
            (".", "examples/api/handler.js", "/examples/api/handler"),
            (".", "examples/api/handler.wasm", "/examples/api/handler"),
            // Root
            (".", "handler.js", "/handler"),
            (".", "handler.wasm", "/handler"),
            // Now, with a different root
            (".\\root", "root\\examples\\handler.js", "/examples/handler"),
            (
                ".\\root",
                "root\\examples\\handler.wasm",
                "/examples/handler",
            ),
            (
                ".\\root",
                "root\\examples\\api\\handler.js",
                "/examples/api/handler",
            ),
            (
                ".\\root",
                "root\\examples\\api\\handler.wasm",
                "/examples/api/handler",
            ),
            (".\\root", "root\\handler.js", "/handler"),
            (".\\root", "root\\handler.wasm", "/handler"),
            // A backslash should not change anything
            (
                ".\\root\\",
                "root\\examples\\handler.js",
                "/examples/handler",
            ),
            (
                ".\\root\\",
                "root\\examples\\handler.wasm",
                "/examples/handler",
            ),
            (
                ".\\root\\",
                "root\\examples\\api\\handler.js",
                "/examples/api/handler",
            ),
            (
                ".\\root\\",
                "root\\examples\\api\\handler.wasm",
                "/examples/api/handler",
            ),
            (".\\root\\", "root\\handler.js", "/handler"),
            (".\\root\\", "root\\handler.wasm", "/handler"),
        ];

        for t in tests {
            assert_eq!(
                Route::retrieve_route(&Path::new(t.0), &PathBuf::from(t.1), ""),
                String::from(t.2),
            )
        }
    }

    #[cfg(not(target_os = "windows"))]
    #[test]
    fn unix_is_in_public_folder() {
        let tests = [
            ("public/index.js", true),
            ("examples/public/index.js", true),
            ("examples/public/other.js", true),
            ("public.js", false),
            ("examples/public.js", false),
            ("./examples/public.js", false),
            ("./examples/index.js", false),
        ];

        for t in tests {
            assert_eq!(is_in_public_folder(Path::new(t.0)), t.1)
        }
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn win_is_in_public_folder() {
        let tests = [
            ("public\\index.js", true),
            ("examples\\public\\index.js", true),
            ("examples\\public\\other.js", true),
            ("public.js", false),
            ("examples\\public.js", false),
            (".\\examples\\public.js", false),
            (".\\examples\\index.js", false),
        ];

        for t in tests {
            assert_eq!(is_in_public_folder(&Path::new(t.0)), t.1)
        }
    }

    #[test]
    fn format_provided_prefix() {
        let tests = [
            // Unix approach
            ("", ""),
            ("/app", "/app"),
            ("app", "/app"),
            ("app/", "/app"),
            ("/app/", "/app"),
            ("/app/test/", "/app/test"),
            ("/app/test", "/app/test"),
            ("app/test/", "/app/test"),
            // Windows approach
            ("\\app", "/app"),
            ("app", "/app"),
            ("app\\", "/app"),
            ("\\app\\", "/app"),
            ("\\app\\test\\", "/app/test"),
            ("\\app\\test", "/app/test"),
            ("app\\test\\", "/app/test"),
        ];

        for t in tests {
            assert_eq!(format_prefix(t.0), String::from(t.1))
        }
    }
}
