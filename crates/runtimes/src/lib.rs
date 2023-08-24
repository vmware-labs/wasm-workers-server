//// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

pub mod errors;
use errors::Result;

mod modules;
mod runtime;

use modules::{external::ExternalRuntime, javascript::JavaScriptRuntime, native::NativeRuntime};
use std::path::Path;
use wws_config::Config;

pub use runtime::Runtime;

// A collection of methods to manage runtimes

/// Initializes a runtime based on the file extension. In the future,
/// This will contain a more complete struct that will identify local
/// runtimes.
pub fn init_runtime(
    project_root: &Path,
    path: &Path,
    config: &Config,
) -> Result<Box<dyn Runtime + Sync + Send>> {
    if let Some(ext) = path.extension() {
        let ext_as_str = ext.to_str().unwrap();

        match ext_as_str {
            "js" => Ok(Box::new(JavaScriptRuntime::new(
                project_root,
                path.to_path_buf(),
            )?)),
            "wasm" => Ok(Box::new(NativeRuntime::new(path.to_path_buf()))),
            other => init_external_runtime(project_root, config, path, other),
        }
    } else {
        Err(errors::RuntimeError::InvalidExtension { extension: None })
    }
}

/// Initialize an external runtime. It looks for the right runtime in the configuration
/// metadata. Then, it will init the runtime with it.
fn init_external_runtime(
    project_root: &Path,
    config: &Config,
    path: &Path,
    extension: &str,
) -> Result<Box<dyn Runtime + Sync + Send>> {
    let mut runtime_config = None;
    let mut repo_name = "";
    let other_string = extension.to_string();

    'outer: for repo in &config.repositories {
        for r in &repo.runtimes {
            if r.extensions.contains(&other_string) {
                runtime_config = Some(r);
                repo_name = &repo.name;
                break 'outer;
            }
        }
    }

    if let Some(runtime_config) = runtime_config {
        Ok(Box::new(ExternalRuntime::new(
            project_root,
            path.to_path_buf(),
            repo_name,
            runtime_config.clone(),
        )?))
    } else {
        Err(errors::RuntimeError::MissingRuntime {
            extension: extension.to_string(),
        })
    }
}
