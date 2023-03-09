// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::fetch::fetch;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use sha256::digest as sha256_digest;
use std::collections::HashMap;
use std::str::FromStr;
use url::Url;

/// Identify the current max repository version this build can manage.
const MAX_REPOSITORY_VERSION: u32 = 1;

/// A Repository contains the list of runtimes available on it.
/// This file is used by wws to properly show the list of available
/// repos and install them.
///
/// By default, this repository class rely on the
/// [WebAssembly Language Runtimes](https://github.com/vmware-labs/webassembly-language-runtimes)
/// repository. It looks for a repository.toml file in the Git repo.
#[derive(Deserialize)]
pub struct Repository {
    /// Version of the repository file
    pub version: u32,
    /// The list of runtimes available in the repository. By default, it will be
    /// filled with an empty vector. The goal is to keep this repository
    /// compatible with future changes. If we don't add this value and change the
    /// runtimes key to something else in the future, the CLI won't deserialize
    /// the version.
    #[serde(default)]
    pub runtimes: Vec<Runtime>,
}

impl Repository {
    /// Retrieve a repository from a remote file. It will download the content
    /// using reqwest and initializing the repository with it.
    pub async fn from_remote_file(repository_url: &str) -> Result<Self> {
        let url = Url::parse(repository_url)?;
        let data = fetch(&url).await?;
        let str_data = String::from_utf8(data)?;

        let repo = Repository::from_str(&str_data)?;

        if repo.version > MAX_REPOSITORY_VERSION {
            println!(
                "⚠️  The repository index version ({}) is not supported by your wws installation.",
                repo.version
            );
            println!("⚠️  This may cause unexpected or missing behaviors. Please, update wws and try it again");
        }

        Ok(repo)
    }

    pub fn find_runtime(&self, name: &str, version: &str) -> Option<&Runtime> {
        self.runtimes.iter().find(|r| {
            r.name == name && (r.version == version || r.tags.contains(&String::from(version)))
        })
    }
}

impl FromStr for Repository {
    type Err = anyhow::Error;

    /// Reads and parses the metadata from a slice of bytes. It will return
    /// a result as the deserialization may fail.
    fn from_str(data: &str) -> Result<Self> {
        toml::from_str::<Repository>(data).map_err(|err| {
            println!("Err: {err}");
            anyhow!("wws could not deserialize the repository metadata")
        })
    }
}

/// Metadata associated to a Runtime. It contains information
/// about a certain runtime like name, version and all the
/// details to run workers with it.
///
/// A runtime is a Wasm binary + configuration that can run
/// a source code as a worker. The configuration includes
/// different pieces like polyfills files, templates,
/// arguments, etc.
#[derive(Deserialize, Serialize, Clone)]
pub struct Runtime {
    /// Name of the runtime (like ruby, python, etc)
    pub name: String,
    /// Specific version of the runtime
    pub version: String,
    /// Optional aliases for the version
    #[serde(default)]
    pub tags: Vec<String>,
    /// Current status in the repository
    pub status: RuntimeStatus,
    /// Associated extensions
    pub extensions: Vec<String>,
    /// Arguments to pass to the Wasm module via WASI
    pub args: Vec<String>,
    /// A list of environment variables that must be configured
    /// for the runtime to work.
    pub envs: Option<HashMap<String, String>>,
    /// The reference to a remote binary (url + checksum)
    pub binary: RemoteFile,
    /// The reference to a remote polyfill file (url + checksum)
    pub polyfill: Option<RemoteFile>,
    /// The reference to a wrapper file for the worker. It will wrap the
    /// source code into a template that can include imports,
    /// function calls, etc.
    pub wrapper: Option<RemoteFile>,
    /// The reference to an example file of a functional worker for this
    /// runtime. It will be used to quickly bootstrap new workers.
    pub template: Option<RemoteFile>,
}

/// Implement comparison by checking the name and version of a given repository.
/// For now, we will rely on this simple comparison as a repository shouldn't
/// include two runtimes with the same name and version
impl PartialEq for Runtime {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.version == other.version
    }
}

/// Define the status of a runtime in a target repository
#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum RuntimeStatus {
    Active,
    Yanked,
    Deprecated,
    Unknown,
}

impl From<&str> for RuntimeStatus {
    /// Create a RuntimeStatus variant from a &str. It uses predefined
    /// values
    fn from(value: &str) -> Self {
        match value {
            "active" => RuntimeStatus::Active,
            "yanked" => RuntimeStatus::Yanked,
            "deprecated" => RuntimeStatus::Deprecated,
            _ => RuntimeStatus::Unknown,
        }
    }
}

/// A file represents a combination of both a remote URL, filename
/// and checksum.
#[derive(Deserialize, Serialize, Clone)]
pub struct RemoteFile {
    /// URL pointing to the file
    pub url: String,
    /// Checksum to validate the given file
    pub checksum: Checksum,
    /// Provide a filename
    pub filename: String,
}

/// A list of available checksums. For now, we will support only sha256
#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum Checksum {
    Sha256 { value: String },
}

impl Checksum {
    /// Validate the provided slice of bytes with the given checksum.
    /// Depending on the type, it will calculate a different digest.
    pub fn validate(&self, bytes: &[u8]) -> Result<()> {
        match self {
            Checksum::Sha256 { value } if value == &sha256_digest(bytes) => Ok(()),
            _ => Err(anyhow!("The checksums don't match")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{any::Any, fs};

    #[test]
    fn parse_index_toml() {
        let contents = fs::read_to_string("../../tests/data/metadata/repository.toml").unwrap();
        let repo = Repository::from_str(&contents).unwrap();

        assert_eq!(repo.version, 1);
        assert_eq!(repo.runtimes.len(), 1);
    }

    #[test]
    fn parse_runtime_toml() {
        let contents = fs::read_to_string("../../tests/data/metadata/runtime.toml").unwrap();
        let metadata = toml::from_str::<Runtime>(&contents).unwrap();

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
