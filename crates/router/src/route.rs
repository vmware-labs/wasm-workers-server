// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0
mod route_type;
mod segment;

use lazy_static::lazy_static;
use regex::Regex;
use route_type::{
    RouteType,
    RouteType::{Dynamic, Static, Tail},
};
use segment::Segment;
use std::{
    cmp::Ordering,
    cmp::Ordering::{Greater, Less},
    collections::HashMap,
    ffi::OsStr,
    path::{Component, Path, PathBuf},
    sync::RwLock,
};
use wws_config::Config as ProjectConfig;
use wws_worker::Worker;

lazy_static! {
    static ref PARAMETER_REGEX: Regex =
        Regex::new(r"\[(?P<ellipsis>\.{3})?(?P<segment>\w+)\]").unwrap();
    pub static ref WORKERS: RwLock<WorkerSet> = RwLock::new(WorkerSet::default());
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
#[derive(Clone, Debug)]
pub struct Route {
    /// The wasm module that will manage the route
    pub handler: PathBuf,
    /// The URL path
    pub path: String,
    /// The route type
    pub route_type: RouteType,
    /// The segments' URL path
    pub segments: Vec<Segment>,
    /// The associated worker
    pub worker: String,
}

/// Structure that holds the map of workers from their identifier to
/// their worker::Worker structure containing the runtime information,
/// such as the WebAssembly module itself as well as the WebAssembly
/// runtime.
///
/// This structure hides the global (but internal) hash map from
/// other crates.
#[derive(Default)]
pub struct WorkerSet {
    workers: HashMap<String, Worker>,
}

impl WorkerSet {
    pub fn get(&self, worker_id: &str) -> Option<&Worker> {
        self.workers.get(worker_id)
    }

    pub fn register(&mut self, worker_id: String, worker: Worker) {
        self.workers.insert(worker_id, worker);
    }
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
        let worker =
            Worker::new(base_path, &filepath, project_config).expect("error creating worker");
        let worker_id = worker.id.clone();

        WORKERS
            .write()
            .expect("error locking worker lock for writing")
            .register(worker_id.clone(), worker);
        let route_path = Self::retrieve_route(base_path, &filepath, prefix);
        Self {
            handler: filepath,
            route_type: RouteType::from(&route_path),
            segments: Self::get_segments(&route_path),
            path: route_path,
            worker: worker_id.clone(),
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

    /// Parse the route path into individual segments and determine their types.
    ///
    /// This function parses the provided route path into individual segments and identifies
    /// whether each segment is static, dynamic, or tail based on certain patterns.
    fn get_segments(route_path: &str) -> Vec<Segment> {
        route_path.split('/').skip(1).map(Segment::from).collect()
    }

    /// Check if the given path can be managed by this worker. This was introduced
    /// to support parameters in the URLs.
    /// Dertermine the 'RouteType' allow to shortcut the comparaison.
    pub fn can_manage(&self, path: &str) -> bool {
        let path_number_of_segments = path.chars().filter(|&c| c == '/').count();

        match self.route_type {
            Static {
                number_of_segments: _,
            } => self.path == path,
            Dynamic { number_of_segments } if number_of_segments != path_number_of_segments => {
                false
            }
            Tail { number_of_segments } if number_of_segments > path_number_of_segments => false,
            _ => path
                .split('/')
                .skip(1)
                .zip(self.segments.iter())
                .all(|zip| match zip {
                    (sp, Segment::Static(segment)) => sp == segment,
                    _ => true,
                }),
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
        match self.route_type {
            Static { .. } => false,
            Dynamic { .. } => true,
            Tail { .. } => true,
        }
    }
}

impl Ord for Route {
    fn cmp(&self, other: &Self) -> Ordering {
        match (&self.route_type, &other.route_type) {
            (
                Static {
                    number_of_segments: a,
                },
                Static {
                    number_of_segments: b,
                },
            ) => a.cmp(b),
            (Static { .. }, _) => Less,
            (_, Static { .. }) => Greater,
            (
                Dynamic {
                    number_of_segments: a,
                },
                Dynamic {
                    number_of_segments: b,
                },
            ) if a == b => self.segments.cmp(&other.segments),
            (
                Dynamic {
                    number_of_segments: a,
                },
                Dynamic {
                    number_of_segments: b,
                },
            ) => a.cmp(b),
            (Dynamic { .. }, _) => Less,
            (_, Dynamic { .. }) => Greater,
            (
                Tail {
                    number_of_segments: a,
                },
                Tail {
                    number_of_segments: b,
                },
            ) if a == b => self.segments.cmp(&other.segments),
            (
                Tail {
                    number_of_segments: a,
                },
                Tail {
                    number_of_segments: b,
                },
            ) => b.cmp(a),
        }
    }
}

impl PartialOrd for Route {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Route {}

impl PartialEq for Route {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
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
