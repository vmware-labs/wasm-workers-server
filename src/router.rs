// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

//
// Declare the different routes for the project
// based on the files in the given folder
//
use crate::config::Config;
use crate::runner::Runner;
use glob::glob;
use lazy_static::lazy_static;
use regex::Regex;
use std::ffi::OsStr;
use std::fs;
use std::path::{Component, Path, PathBuf};

lazy_static! {
    static ref PARAMETER_REGEX: Regex = Regex::new(r"\[\w+\]").unwrap();
    static ref DYNAMIC_ROUTE_REGEX: Regex = Regex::new(r".*\[\w+\].*").unwrap();
}

/// Identify if a route can manage a certain URL and generates
/// a score in that case. This is required by dynamic routes as
/// different files can manage the same route. For example:
/// `/test` may be managed by `test.js` and `[id].js`. Regarding
/// the score, routes with a lower value will have a higher priority.
#[derive(PartialEq, Eq, Debug)]
pub enum RouteAffinity {
    CannotManage,
    // Score
    CanManage(i32),
}

/// Contains all registered routes
pub struct Routes {
    pub routes: Vec<Route>,
}

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

    /// Check if the given path can be managed by this worker. This was introduced
    /// to support parameters in the URLs. Note that this method returns an integer,
    /// which means the priority for this route.
    ///
    /// Note that a /a/b route may be served by:
    /// - /a/b.js
    /// - /a/[id].js
    /// - /[id]/b.wasm
    /// - /[id]/[other].wasm
    ///
    /// We need to establish a priority. The lower of the returned number,
    /// the more priority it has. This number is calculated based on the number of used
    /// parameters, as fixed routes has more priority than parameted ones.
    ///
    /// To avoid collisions like `[id]/b.wasm` vs `/a/[id].js`. Every depth level will
    /// add an extra +1 to the score. So, in case of `[id]/b.wasm` vs `/a/[id].js`,
    /// the /a/b path will be managed by `[id]/b.wasm`
    ///
    /// In case it cannot manage it, it will return -1
    pub fn affinity(&self, url_path: &str) -> RouteAffinity {
        let mut score: i32 = 0;
        let mut split_path = self.path.split('/').peekable();

        for (depth, portion) in url_path.split('/').enumerate() {
            match split_path.next() {
                Some(el) if el == portion => continue,
                Some(el) if PARAMETER_REGEX.is_match(el) => {
                    score += depth as i32;
                    continue;
                }
                _ => return RouteAffinity::CannotManage,
            }
        }

        // I should check the other iterator to confirm is empty
        if split_path.peek().is_none() {
            RouteAffinity::CanManage(score)
        } else {
            // The split path iterator still have some entries.
            RouteAffinity::CannotManage
        }
    }

    /// Returns the given path with the actix format. For dynamic routing
    /// we are using `[]` in the filenames. However, actix expects a `{}`
    /// format for parameters.
    pub fn actix_path(&self) -> String {
        // Replace [] with {} for making the path compatible with
        let mut formatted = self.path.replace('[', "{");
        formatted = formatted.replace(']', "}");

        formatted
    }

    /// Check if the current route is dynamic
    pub fn is_dynamic(&self) -> bool {
        DYNAMIC_ROUTE_REGEX.is_match(&self.path)
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

/// Based on a set of routes and a given path, it provides the best
/// match based on the parametrized URL score. See the [`Route::can_manage_path`]
/// method to understand how to calculate the score.
pub fn retrieve_best_route<'a>(routes: &'a Routes, path: &str) -> Option<&'a Route> {
    // Keep it to avoid calculating the score twice when iterating
    // to look for the best route
    let mut best_score = -1;

    routes
        .routes
        .iter()
        .fold(None, |acc, item| match item.affinity(path) {
            RouteAffinity::CanManage(score) if best_score == -1 || score < best_score => {
                best_score = score;
                Some(item)
            }
            _ => acc,
        })
}

/// Defines a prefix in the context of the application.
/// This prefix will be used for the static assets and the
/// workers.
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

    #[test]
    fn route_path_affinity() {
        let build_route = |file: &str| -> Route {
            Route::new(
                Path::new("./tests/data/params"),
                PathBuf::from(format!("./tests/data/params{}", file)),
                "",
            )
        };

        // Route initializes the Wasm module. We create these
        // variables to avoid loading the same Wasm module multiple times
        let param_route = build_route("/[id].wasm");
        let fixed_route = build_route("/fixed.wasm");
        let param_folder_route = build_route("/[id]/fixed.wasm");
        let param_sub_route = build_route("/sub/[id].wasm");

        let tests = [
            (&param_route, "/a", RouteAffinity::CanManage(1)),
            (&fixed_route, "/fixed", RouteAffinity::CanManage(0)),
            (&fixed_route, "/a", RouteAffinity::CannotManage),
            (&param_folder_route, "/a", RouteAffinity::CannotManage),
            (&param_folder_route, "/a/fixed", RouteAffinity::CanManage(1)),
            (&param_sub_route, "/a/b", RouteAffinity::CannotManage),
            (&param_sub_route, "/sub/b", RouteAffinity::CanManage(2)),
        ];

        for t in tests {
            assert_eq!(t.0.affinity(t.1), t.2);
        }
    }

    #[test]
    fn best_route_by_affinity() {
        let build_route = |file: &str| -> Route {
            Route::new(
                Path::new("./tests/data/params"),
                PathBuf::from(format!("./tests/data/params{}", file)),
                "",
            )
        };

        // Route initializes the Wasm module. We create these
        // variables to avoid loading the same Wasm module multiple times
        let param_route = build_route("/[id].wasm");
        let fixed_route = build_route("/fixed.wasm");
        let param_folder_route = build_route("/[id]/fixed.wasm");
        let param_sub_route = build_route("/sub/[id].wasm");

        // I'm gonna use this values for comparison as `routes` consumes
        // the Route elements.
        let param_path = param_route.path.clone();
        let fixed_path = fixed_route.path.clone();
        let param_folder_path = param_folder_route.path.clone();
        let param_sub_path = param_sub_route.path.clone();

        let routes = Routes {
            routes: vec![
                param_route,
                fixed_route,
                param_folder_route,
                param_sub_route,
            ],
        };

        let tests = [
            ("/a", Some(param_path)),
            ("/fixed", Some(fixed_path)),
            ("/a/fixed", Some(param_folder_path)),
            ("/sub/b", Some(param_sub_path)),
            ("/donot/exist", None),
        ];

        for t in tests {
            let route = retrieve_best_route(&routes, t.0);

            if let Some(path) = t.1 {
                assert!(route.is_some());
                assert_eq!(route.unwrap().path, path);
            } else {
                assert!(route.is_none());
            }
        }
    }

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
