// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::errors::Result;
use crate::runtime::Runtime;

use std::path::{Path, PathBuf};
use wasmtime_wasi::{ambient_authority, Dir, WasiCtxBuilder};
use wws_store::Store;

static JS_ENGINE_WASM: &[u8] =
    include_bytes!("../../../../kits/javascript/wasm-workers-quick-js-engine.wasm");

pub struct JavaScriptRuntime {
    /// Path of the given module
    path: PathBuf,
    /// Utils to store temporary files for this runtime
    store: Store,
}

impl JavaScriptRuntime {
    /// Initializes the JavaScript runtime. This runtime includes a
    /// compiled QuickJS Wasm module. To run a worker, we need to
    /// mount the JS file into /src/index.js and the runtime will
    /// automatically pick and run it. We use the Data struct for
    /// this purpose
    pub fn new(project_root: &Path, path: PathBuf) -> Result<Self> {
        let hash = Store::file_hash(&path)?;
        let store = Store::create(project_root, &["workers", "js", &hash])?;

        Ok(Self { path, store })
    }
}

impl Runtime for JavaScriptRuntime {
    /// Prepare the environment to run this specific worker. Since
    /// the current folder received by argument may include multiple
    /// files (workers), we use the Data struct to write the JS source
    /// file into an isolated and separate folder. Then, we will mount
    /// it during the [prepare_wasi_ctx] call.
    fn prepare(&self) -> Result<()> {
        self.store.copy(&self.path, &["index.js"])?;

        Ok(())
    }

    /// Mount the source code in the WASI context so it can be
    /// processed by the engine
    fn prepare_wasi_ctx(&self, builder: WasiCtxBuilder) -> Result<WasiCtxBuilder> {
        let dir = Dir::open_ambient_dir(&self.store.folder, ambient_authority())?;
        Ok(builder.preopened_dir(dir, "/src")?)
    }

    /// Returns a reference to the Wasm module that should
    /// run this worker. It can be a custom (native) or a
    /// shared module (others).
    fn module_bytes(&self) -> Result<Vec<u8>> {
        Ok(JS_ENGINE_WASM.to_vec())
    }
}
