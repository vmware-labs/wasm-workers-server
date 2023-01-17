// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use super::runtimes::{javascript::JavaScriptRuntime, native::NativeRuntime};
use anyhow::{anyhow, Result};
use std::path::Path;
use wasmtime_wasi::WasiCtxBuilder;

/// Define the behavior a Runtime must have. This includes methods
/// to initialize the environment for the given runtime as well as
/// the Wasi Context to process the request.
pub trait Runtime {
    /// Prepare the environment. This method
    /// initializes folders and create files if required
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

    /// Check if the given path can be managed by this runtime.
    // fn can_manage(path: &Path) -> bool;

    /// Returns a reference to the Wasm module that should
    /// run this worker. It can be a custom (native) or a
    /// shared module (others).
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
