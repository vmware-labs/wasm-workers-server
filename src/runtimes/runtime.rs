// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use super::modules::{javascript::JavaScriptRuntime, native::NativeRuntime};
use anyhow::{anyhow, Result};
use serde::Deserialize;
use std::path::Path;
use wasmtime_wasi::WasiCtxBuilder;

/// Define the status of a runtime in a target repository
#[derive(Deserialize)]
pub enum RuntimeStatus {
    Active,
    Yanked,
    Deprecated,
    Unknown,
}

impl From<&str> for RuntimeStatus {
    /// Create a RuntimeStatus variant from a &str. It uses predefined
    /// values
    fn from(value: &str) -> Self {
        match value {
            "active" => RuntimeStatus::Active,
            "yanked" => RuntimeStatus::Yanked,
            "deprecated" => RuntimeStatus::Deprecated,
            _ => RuntimeStatus::Unknown,
        }
    }
}

/// Define the behavior a Runtime must have. This includes methods
/// to initialize the environment for the given runtime as well as
/// the Wasi Context to process the request.
pub trait Runtime {
    /// Allow a runtime to prepare the run environment if it's required.
    /// This method is called when loading the different workers from the
    /// filesystem. This method is only called once before the service
    /// is ready to start processing requests.
    fn prepare(&self) -> Result<()> {
        Ok(())
    }

    /// Append the required properties to the given
    /// WASI context builder. This allow runtimes to mount
    /// specific lib folders, source code and adding
    /// environment variables.
    fn prepare_wasi_ctx(&self, builder: WasiCtxBuilder) -> Result<WasiCtxBuilder> {
        Ok(builder)
    }

    /// Returns a reference raw bytes of the Wasm module that should
    /// run this worker. It can be directly the contents of the file
    /// that was identified as a worker (.wasm / native) or a shared
    /// runtime like JS or Python.
    fn module_bytes(&self) -> Result<Vec<u8>>;
}

/// Initializes a runtime based on the file extension. In the future,
/// This will contain a more complete struct that will identify local
/// runtimes.
pub fn init_runtime(path: &Path) -> Result<Box<dyn Runtime + Sync + Send>> {
    if let Some(ext) = path.extension() {
        let ext_as_str = ext.to_str().unwrap();

        match ext_as_str {
            "js" => Ok(Box::new(JavaScriptRuntime::new(path.to_path_buf())?)),
            "wasm" => Ok(Box::new(NativeRuntime::new(path.to_path_buf()))),
            _ => Err(anyhow!(format!(
                "The '{}' extension does not have an associated runtime",
                ext_as_str
            ))),
        }
    } else {
        Err(anyhow!("The given file does not have a valid extension"))
    }
}
