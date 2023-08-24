// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::errors::Result;
use crate::runtime::Runtime;

use std::{fs, path::PathBuf};

pub struct NativeRuntime {
    /// Path of the given module
    path: PathBuf,
}

impl NativeRuntime {
    /// Initializes the given runtime
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl Runtime for NativeRuntime {
    /// Returns a reference to the Wasm module that should
    /// run this worker. It can be a custom (native) or a
    /// shared module (others).
    fn module_bytes(&self) -> Result<Vec<u8>> {
        fs::read(&self.path).map_err(|_| crate::errors::RuntimeError::CannotReadModule)
    }
}
