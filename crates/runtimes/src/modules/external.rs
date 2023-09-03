// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::errors::{self, Result};

use crate::runtime::Runtime;
use std::{
    fs,
    path::{Path, PathBuf},
};
use wasmtime_wasi::{ambient_authority, Dir, WasiCtxBuilder};
use wws_project::metadata::Runtime as RuntimeMetadata;
use wws_store::Store;

/// Run language runtimes that were downloaded externally. This
/// runtime prepare the worker and configure the WASI context
/// based on the given metadata.
pub struct ExternalRuntime {
    /// Path of the given module
    path: PathBuf,
    /// Utils to store temporary files for this runtime
    store: Store,
    /// Associated runtime metadata
    metadata: RuntimeMetadata,
    /// Runtime store to load different files
    runtime_store: Store,
}

impl ExternalRuntime {
    /// Initializes the External runtime. This runtime will use
    /// the associated metadata to properly prepare the worker
    /// and the WASI environment
    pub fn new(
        project_root: &Path,
        path: PathBuf,
        repository: &str,
        metadata: RuntimeMetadata,
    ) -> Result<Self> {
        let hash = Store::file_hash(&path)?;
        // TODO: May move to a different folder strucuture when having multiple extensions?
        let worker_folder = metadata.extensions.first().unwrap_or(&metadata.name);
        let store = Store::create(project_root, &["workers", worker_folder, &hash])?;
        let runtime_store = Store::new(
            project_root,
            &["runtimes", repository, &metadata.name, &metadata.version],
        );

        Ok(Self {
            path,
            store,
            metadata,
            runtime_store,
        })
    }
}

impl Runtime for ExternalRuntime {
    /// Prepare the environment to run this specific worker. Since
    /// the current folder received by argument may include multiple
    /// files (workers), we use the Data struct to write the JS source
    /// file into an isolated and separate folder. Then, we will mount
    /// it during the [prepare_wasi_ctx] call.
    fn prepare(&self) -> Result<()> {
        let filename = format!("index.{}", self.metadata.extensions.first().unwrap());

        // If wrapper, modify the worker and write the data
        if let Some(wrapper) = &self.metadata.wrapper {
            let wrapper_data = String::from_utf8(self.runtime_store.read(&[&wrapper.filename])?)?;
            let source_data = fs::read_to_string(&self.path)?;

            self.store.write(
                &[&filename],
                wrapper_data.replace("{source}", &source_data).as_bytes(),
            )?;
        } else {
            // If not, copy the worker
            self.store.copy(&self.path, &[&filename])?;
        }

        // Copy polyfills file if available
        if let Some(polyfill) = &self.metadata.polyfill {
            self.store.copy(
                &self.runtime_store.build_folder_path(&[&polyfill.filename]),
                &[&polyfill.filename],
            )?;
        }

        Ok(())
    }

    /// Mount the source code in the WASI context so it can be
    /// processed by the engine
    fn prepare_wasi_ctx(&self, builder: WasiCtxBuilder) -> Result<WasiCtxBuilder> {
        let dir = Dir::open_ambient_dir(&self.store.folder, ambient_authority())?;

        builder
            .preopened_dir(dir, "/src")?
            .args(&self.metadata.args)
            .map_err(|_| errors::RuntimeError::WasiContextError)
    }

    /// Returns a reference to the Wasm module that should
    /// run this worker. It can be a custom (native) or a
    /// shared module (others).
    fn module_bytes(&self) -> Result<Vec<u8>> {
        self.runtime_store
            .read(&[&self.metadata.binary.filename])
            .map_err(|_| errors::RuntimeError::CannotReadModule)
    }
}
