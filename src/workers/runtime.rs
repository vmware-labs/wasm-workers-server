// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
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
