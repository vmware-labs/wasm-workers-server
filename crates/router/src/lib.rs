// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

//
// Declare the different routes for the project
// based on the files in the given folder
//

mod files;
mod route;
use files::Files;
use std::path::{Path, PathBuf};
use std::time::Instant;
use wws_config::Config;

pub use route::{Route, WORKERS};

/// Contains all registered routes
#[derive(Clone, Default)]
pub struct Routes {
    pub routes: Vec<Route>,
    pub prefix: String,
}

impl Routes {
    /// Initialize the list of routes from the given folder. This method will look for
    /// different files and will create the associated routes. This routing approach
    /// is pretty popular in web development and static sites.
    pub fn new(
        path: &Path,
        base_prefix: &str,
        ignore_patterns: Vec<String>,
        config: &Config,
    ) -> Self {
        let mut routes = Vec::new();
        let prefix = Self::format_prefix(base_prefix);
        let runtime_extensions = config.get_runtime_extensions(path);

        let files = Files::new(path, runtime_extensions, ignore_patterns);

        let mut route_paths: Vec<PathBuf> = Vec::new();
        for entry in files.walk() {
            route_paths.push(entry.into_path());
        }

        println!("⏳ Loading workers from {} routes...", route_paths.len());
        let start = Instant::now();
        for route_path in route_paths {
            routes.push(Route::new(path, route_path, &prefix, config));
        }
        routes.sort();
        println!("✅ Workers loaded in {:?}.", start.elapsed());

        Self { routes, prefix }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Route> {
        self.routes.iter()
    }

    /// Provides the **first** route that can handle the given path.
    /// This only works because the routes are already sorted.
    /// Because a '/a/b' route may be served by:
    /// - /a/b.js
    /// - /a/[id].js
    /// - /[id]/b.wasm
    /// - /[id]/[other].wasm
    /// - /[id]/[..all].wasm
    pub fn retrieve_best_route<'a>(&'a self, path: &str) -> Option<&'a Route> {
        self.iter().find(|r| r.can_manage(path))
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

    #[test]
    fn routes_sorted_on_creation() {
        let project_config = Config::default();
        let router = Routes::new(
            Path::new("../../tests/data/params"),
            "",
            Vec::new(),
            &project_config,
        );

        let mut sorted_router = Routes::new(
            Path::new("../../tests/data/params"),
            "",
            Vec::new(),
            &project_config,
        );

        sorted_router.routes.sort();

        assert_eq!(router.routes, sorted_router.routes);
    }

    #[test]
    fn retrieve_best_route() {
        let project_config = Config::default();
        let router = Routes::new(
            Path::new("../../tests/data/params"),
            "",
            Vec::new(),
            &project_config,
        );

        let tests = [
            ("/any", Some("/[id]")),
            ("/fixed", Some("/fixed")),
            ("/any/fixed", Some("/[id]/fixed")),
            ("/any/sub", Some("/[id]/sub")),
            ("/sub/any", Some("/sub/[id]")),
            ("/sub/any/catch/all/routes", Some("/sub/[...all]")),
            ("/sub/sub/any/catch/all/routes", Some("/sub/sub/[...all]")),
            ("/donot/exist", None),
        ];

        for (given_path, expected_path) in tests {
            let route = router.retrieve_best_route(given_path);

            if let Some(path) = expected_path {
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
