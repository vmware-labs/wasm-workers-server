// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::data::kv::KVConfigData;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use toml::from_slice;

/// Handlers configuration. These files are optional when no configuration change is required.
#[derive(Deserialize, Clone)]
pub struct Config {
    /// Handler name. For logging purposes
    pub name: Option<String>,
    /// Mandatory version of the file
    pub version: String,
    /// Optional data configuration
    pub data: Option<ConfigData>,
}

/// Configure a data plugin for the handler
#[derive(Deserialize, Clone)]
pub struct ConfigData {
    /// Creates a Key/Value store associated to the given handler
    pub kv: Option<KVConfigData>,
}

impl Config {
    /// Try to read the configuration from a TOML file. The path contains the local path
    /// to the handler configuration. The file should use the same name as the handler,
    /// with the .toml extension
    ///
    /// # Examples
    ///
    /// ```
    /// name = "todos"
    /// version = "1"
    ///
    /// [data]
    ///
    /// [data.kv]
    /// namespace = "todos"
    /// ```
    pub fn try_from_file(path: PathBuf) -> Result<Self, String> {
        let contents = fs::read(&path).expect("The configuration file was not properly loaded");

        let try_config: Result<Config, toml::de::Error> = from_slice(&contents);

        match try_config {
            Ok(c) => Ok(c),
            Err(err) => Err(format!(
                "Error reading the configuration file at {}: {}",
                &path.to_str().unwrap_or("?"),
                err
            )),
        }
    }

    /// Returns a data Key/Value configuration if available
    pub fn data_kv_config(&self) -> Option<&KVConfigData> {
        self.data.as_ref()?.kv.as_ref()
    }

    /// Returns the data Key/Value namespace if available
    pub fn data_kv_namespace(&self) -> Option<String> {
        Some(self.data_kv_config()?.namespace.clone())
    }
}
