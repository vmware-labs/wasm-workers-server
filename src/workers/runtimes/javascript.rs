// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use super::temp_utils::TempUtils;
use crate::workers::runtime::Runtime;
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
    temp_utils: TempUtils,
}

impl JavaScriptRuntime {
    /// Initializes the given runtime
    pub fn new(path: PathBuf) -> Result<Self> {
        let temp_utils = TempUtils::new(String::from("js"), &path)?;

        Ok(Self { path, temp_utils })
    }
}

impl Runtime for JavaScriptRuntime {
    /// Prepare the environment. This method
    /// initializes folders and create files if required
    fn prepare(&self) -> Result<()> {
        self.temp_utils.write_source(&self.path)?;

        Ok(())
    }

    /// Mount the source code in the WASI context so it can be
    /// processed by the engine
    fn prepare_wasi_ctx(&self, builder: WasiCtxBuilder) -> Result<WasiCtxBuilder> {
        let source = fs::File::open(&self.temp_utils.folder)?;
        Ok(builder.preopened_dir(Dir::from_std_file(source), "/src")?)
    }

    /// Returns a reference to the Wasm module that should
    /// run this worker. It can be a custom (native) or a
    /// shared module (others).
    fn module_bytes(&self) -> Result<Vec<u8>> {
        Ok(JS_ENGINE_WASM.to_vec())
    }
}
