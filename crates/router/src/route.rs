// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use lazy_static::lazy_static;
use regex::Regex;
use std::{
    ffi::OsStr,
    path::{Component, Path, PathBuf},
};
use wws_config::Config as ProjectConfig;
use wws_worker::Worker;

lazy_static! {
    static ref PARAMETER_REGEX: Regex =
        Regex::new(r"\[(?P<ellipsis>\.{3})?(?P<segment>\w+)\]").unwrap();
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
pub struct Route {
    /// The wasm module that will manage the route
    pub handler: PathBuf,
    /// The URL path
    pub path: String,
    /// The associated worker
    pub worker: Worker,
}

impl Route {
    /// Initialize a new route from the given folder and filepath. It will calculate the
    /// proper URL path based on the filename.
    ///
    /// This method also initializes the Runner and loads the Config if available.
    pub fn new(
        base_path: &Path,
        filepath: PathBuf,
        prefix: &str,
        project_config: &ProjectConfig,
    ) -> Self {
        let worker = Worker::new(base_path, &filepath, project_config).unwrap();

        Self {
            path: Self::retrieve_route(base_path, &filepath, prefix),
            handler: filepath,
            worker,
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
    fn normalize_path_to_url(path: &Path) -> String {
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
    /// - /[id]/[..all].wasm
    ///
    /// We need to establish a priority. The lower of the returned number,
    /// the more priority it has. This number is calculated based on the number of used
    /// parameters, as fixed routes has more priority than parameted ones.
    ///
    /// To avoid collisions like `[id]/b.wasm` vs `/a/[id].js`. Every depth level will
    /// add an extra +1 to the score. So, in case of `[id]/b.wasm` vs `/a/[id].js`,
    /// the /a/b path will be managed by `/a/[id].js`
    ///
    /// In case it cannot manage it, it will return -1
    pub fn affinity(&self, url_path: &str) -> RouteAffinity {
        let mut score: i32 = 0;
        let mut split_path = self.path.split('/').peekable();

        for (depth, segment) in url_path.split('/').enumerate() {
            match split_path.next() {
                None => return RouteAffinity::CannotManage,
                Some(el) if el == segment => continue,
                Some(el) => match PARAMETER_REGEX.captures(el) {
                    None => return RouteAffinity::CannotManage,
                    Some(caps) => match (caps.name("ellipsis"), caps.name("segment")) {
                        (Some(_), Some(_)) => return RouteAffinity::CanManage(i32::MAX),
                        (_, Some(_)) => score += depth as i32,
                        _ => return RouteAffinity::CannotManage,
                    },
                },
            }
        }

        // I should check the other iterator to confirm if it is empty
        match split_path.peek() {
            None => RouteAffinity::CanManage(score),
            Some(_) => RouteAffinity::CannotManage,
        }
    }

    /// Returns the given path with the actix format. For dynamic routing
    /// we are using `[]` in the filenames. However, actix expects a `{}`
    /// format for parameters.
    pub fn actix_path(&self) -> String {
        PARAMETER_REGEX
            .replace_all(&self.path, |caps: &regex::Captures| {
                match (caps.name("ellipsis"), caps.name("segment")) {
                    (Some(_), Some(segment)) => format!("{{{}:.*}}", segment.as_str()),
                    (_, Some(segment)) => format!("{{{}}}", segment.as_str()),
                    _ => String::new(),
                }
            })
            .into()
    }

    /// Check if the current route is dynamic
    pub fn is_dynamic(&self) -> bool {
        PARAMETER_REGEX.is_match(&self.path)
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
}
