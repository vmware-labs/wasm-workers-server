// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::errors::Result;

use wasmtime_wasi::WasiCtxBuilder;

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
