// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

//
// Declare the different routes for the project
// based on the files in the given folder
//

mod files;
mod route;

use files::Files;
use route::{Route, RouteAffinity};
use std::path::Path;
use wws_config::Config;

/// Contains all registered routes
pub struct Routes {
    pub routes: Vec<Route>,
    pub prefix: String,
}

impl Routes {
    /// Initialize the list of routes from the given folder. This method will look for
    /// different files and will create the associated routes. This routing approach
    /// is pretty popular in web development and static sites.
    pub fn new(path: &Path, base_prefix: &str, ignore_patterns: Vec<String>, config: &Config) -> Self {
        let mut routes = Vec::new();
        let prefix = Self::format_prefix(base_prefix);
        let runtime_extensions = config.get_runtime_extensions(path);

        let files = Files::new(path, runtime_extensions, ignore_patterns);

        for entry in files.walk() {
            routes.push(Route::new(path, entry.into_path(), &prefix, config));
        }

        Self { routes, prefix }
    }

    /// Based on a set of routes and a given path, it provides the best
    /// match based on the parametrized URL score. See the [`Route::can_manage_path`]
    /// method to understand how to calculate the score.
    pub fn retrieve_best_route<'a>(&'a self, path: &str) -> Option<&'a Route> {
        // Keep it to avoid calculating the score twice when iterating
        // to look for the best route
        let mut best_score = -1;

        self.routes
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
    fn format_prefix(source: &str) -> String {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn route_path_affinity() {
        let build_route = |file: &str| -> Route {
            let project_config = Config::default();
            Route::new(
                Path::new("../../tests/data/params"),
                PathBuf::from(format!("../../tests/data/params{file}")),
                "",
                &project_config,
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
            let project_config = Config::default();
            Route::new(
                Path::new("../../tests/data/params"),
                PathBuf::from(format!("../../tests/data/params{file}")),
                "",
                &project_config,
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
            prefix: String::from("/"),
        };

        let tests = [
            ("/a", Some(param_path)),
            ("/fixed", Some(fixed_path)),
            ("/a/fixed", Some(param_folder_path)),
            ("/sub/b", Some(param_sub_path)),
            ("/donot/exist", None),
        ];

        for t in tests {
            let route = routes.retrieve_best_route(t.0);

            if let Some(path) = t.1 {
                assert!(route.is_some());
                assert_eq!(route.unwrap().path, path);
            } else {
                assert!(route.is_none());
            }
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
            assert_eq!(Routes::format_prefix(t.0), String::from(t.1))
        }
    }
}
