// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use super::metadata::RuntimeMetadata;
use anyhow::{anyhow, Result};
use serde::Deserialize;

// TODO: Remove it when implementing the manager
#[allow(dead_code)]
/// A Repository contains the list of runtimes available on it.
/// This file is used by wws to properly show the list of available
/// repos and install them.
///
/// By default, this repository class rely on the [WebAssembly Language Runtimes](https://github.com/vmware-labs/webassembly-language-runtimes)
/// repository. It looks for a repository.toml file in the Git repo.
#[derive(Deserialize)]
pub struct Repository<'a> {
    /// Version of the repository file
    pub version: u32,
    /// The list of runtimes available in the repository
    #[serde(borrow)]
    pub runtimes: Vec<RuntimeMetadata<'a>>,
}

// TODO: Remove it when implementing the manager
#[allow(dead_code)]
impl<'a> Repository<'a> {
    /// Reads and parses the metadata from a slice of bytes. It will return
    /// a result as the deserialization may fail.
    pub fn from_slice(data: &'a [u8]) -> Result<Self> {
        toml::from_slice::<Repository>(data)
            .map_err(|_| anyhow!("wws could not deserialize the repository metadata"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn parse_index_toml() {
        let contents = fs::read("tests/data/metadata/repository.toml").unwrap();
        let repo = Repository::from_slice(&contents).unwrap();

        assert_eq!(repo.version, 1);
        assert_eq!(repo.runtimes.len(), 1);
    }
}
