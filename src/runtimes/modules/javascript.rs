// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use super::data::Data;
use crate::runtimes::runtime::Runtime;
use anyhow::Result;
use std::{fs, path::PathBuf};
use wasmtime_wasi::Dir;
use wasmtime_wasi::WasiCtxBuilder;

static JS_ENGINE_WASM: &[u8] =
    include_bytes!("../../../kits/javascript/wasm-workers-quick-js-engine.wasm");

pub struct JavaScriptRuntime {
    /// Path of the given module
    path: PathBuf,
    /// Utils to make temporary files
    data: Data,
}

impl JavaScriptRuntime {
    /// Initializes the JavaScript runtime. This runtime includes a
    /// compiled QuickJS Wasm module. To run a worker, we need to
    /// mount the JS file into /src/index.js and the runtime will
    /// automatically pick and run it. We use the Data struct for
    /// this purpose
    pub fn new(path: PathBuf) -> Result<Self> {
        let data = Data::new(String::from("js"), &path)?;

        Ok(Self { path, data })
    }
}

impl Runtime for JavaScriptRuntime {
    /// Prepare the environment to run this specific worker. Since
    /// the current folder received by argument may include multiple
    /// files (workers), we use the Data struct to write the JS source
    /// file into an isolated and separate folder. Then, we will mount
    /// it during the [prepare_wasi_ctx] call.
    fn prepare(&self) -> Result<()> {
        self.data.write_source(&self.path)?;

        Ok(())
    }

    /// Mount the source code in the WASI context so it can be
    /// processed by the engine
    fn prepare_wasi_ctx(&self, builder: WasiCtxBuilder) -> Result<WasiCtxBuilder> {
        let source = fs::File::open(&self.data.folder)?;
        Ok(builder.preopened_dir(Dir::from_std_file(source), "/src")?)
    }

    /// Returns a reference to the Wasm module that should
    /// run this worker. It can be a custom (native) or a
    /// shared module (others).
    fn module_bytes(&self) -> Result<Vec<u8>> {
        Ok(JS_ENGINE_WASM.to_vec())
    }
}
