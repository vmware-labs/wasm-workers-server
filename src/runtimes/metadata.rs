// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use super::{remote_file::RemoteFile, runtime::RuntimeStatus};
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
    /// The reference to a remote binary (url + checksum)
    binary: RemoteFile<'a>,
    /// The reference to a remote polyfill file (url + checksum)
    polyfill: Option<RemoteFile<'a>>,
    /// The refernmece to a template file for the worker. It will wrap the
    /// source code into a template that can include imports,
    /// function calls, etc.
    template: Option<RemoteFile<'a>>,
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

#[cfg(test)]
mod tests {
    use crate::runtimes::remote_file::Checksum;

    use super::*;
    use std::{any::Any, fs};

    #[test]
    fn parse_runtime_toml() {
        let contents = fs::read("tests/data/metadata/runtime.toml").unwrap();
        let metadata = RuntimeMetadata::from_slice(&contents).unwrap();

        assert_eq!(metadata.name, "ruby");
        assert_eq!(metadata.version, "3.2.0+20230118-8aec06d");
        assert_eq!(metadata.status.type_id(), RuntimeStatus::Active.type_id());
        assert_eq!(metadata.binary.url, "https://github.com/vmware-labs/webassembly-language-runtimes/releases/download/ruby%2F3.2.0%2B20230118-8aec06d/ruby-3.2.0.wasm");

        let Checksum::Sha256 { value } = metadata.binary.checksum;
        assert_eq!(
            value,
            "e2d91cff05ec59ed9c88aadbd3b477842092054bf24c5d944d5ad6dbafdd3b32"
        );

        // Optionals
        let polyfill = metadata.polyfill.unwrap();
        assert_eq!(
            polyfill.url,
            "https://raw.githubusercontent.com/Angelmmiguel/wws-index-test/main/ruby/poly.rb"
        );
    }
}
