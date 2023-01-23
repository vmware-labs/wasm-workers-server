// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use super::runtime::RuntimeStatus;
use anyhow::{anyhow, Result};
use serde::Deserialize;
use std::collections::HashMap;

// TODO: Remove it when implementing the manager
#[allow(dead_code)]
/// Metadata associated to a Runtime. It contains information
/// about a certain runtime like name, version and all the
/// details to run workers with it.
///
/// A runtime is a Wasm binary + configuration that can run
/// a source code as a worker. The configuration includes
/// different pieces like polyfills files, templates,
/// arguments, etc.
#[derive(Deserialize)]
pub struct RuntimeMetadata<'a> {
    /// Name of the runtime (like ruby, python, etc)
    name: &'a str,
    /// Specific version of the runtime
    version: &'a str,
    /// Current status in the repository
    status: RuntimeStatus,
    /// Associated extensions
    extensions: Vec<&'a str>,
    /// Arguments to pass to the Wasm module via WASI
    args: Vec<&'a str>,
    /// A list of environment variables that must be configured
    /// for the runtime to work.
    envs: Option<HashMap<&'a str, &'a str>>,
    /// The URL to pull the module binary
    binary: &'a str,
    /// The checksum to validate the given binary (SHA256)
    binary_checksum: &'a str,
    /// The URL to a polyfill file
    polyfill: Option<&'a str>,
    /// The checksum to validate the given polyfill file (SHA256)
    polyfill_polyfill: Option<&'a str>,
    /// The URL to a template file for the worker. It will wrap the
    /// source code into a template that can include imports,
    /// function calls, etc.
    template: Option<&'a str>,
    /// The checksum to validate the given template file (SHA256)
    template_polyfill: Option<&'a str>,
}

// TODO: Remove it when implementing the manager
#[allow(dead_code)]
impl<'a> RuntimeMetadata<'a> {
    /// Reads and parses the metadata from a slice of bytes. It will return
    /// a result as the deserialization may fail.
    pub fn from_slice(data: &'a [u8]) -> Result<Self> {
        toml::from_slice::<RuntimeMetadata>(data)
            .map_err(|_| anyhow!("wws could not deserialize the runtime metadata"))
    }
}
