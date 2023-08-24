// Copyright 2022-2023 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::errors::Result;
use crate::features::http_requests::HttpRequestsConfig;
use crate::features::wasi_nn::WasiNnConfig;
use crate::features::{data::ConfigData, folders::Folder};
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;
use std::path::PathBuf;
use std::{env, fs};
use toml::from_str;
use wws_data_kv::KVConfigData;

/// List all available features for a worker
#[derive(Deserialize, Clone, Default)]
#[serde(default)]
pub struct Features {
    /// Allow to perform http requests from a worker
    pub http_requests: HttpRequestsConfig,
    /// Enables WASI-NN bindings for Machine Learning inference
    pub wasi_nn: WasiNnConfig,
}

/// Workers configuration. These files are optional when no configuration change is required.
#[derive(Deserialize, Clone, Default)]
pub struct Config {
    /// Worker name. For logging purposes
    pub name: Option<String>,
    /// Mandatory version of the file
    pub version: String,
    /// List of features. By default, all will be inactive
    #[serde(default)]
    pub features: Features,
    /// Optional data configuration
    pub data: Option<ConfigData>,
    /// Optional folders
    pub folders: Option<Vec<Folder>>,
    /// Optional environment configuration
    #[serde(deserialize_with = "read_environment_variables", default)]
    pub vars: HashMap<String, String>,
}

impl Config {
    /// Try to read the configuration from a TOML file. The path contains the local path
    /// to the worker configuration. The file should use the same name as the worker,
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
    pub fn try_from_file(path: PathBuf) -> Result<Self> {
        Ok(from_str(&fs::read_to_string(path).map_err(|_| {
            crate::errors::WorkerError::CannotLoadConfig
        })?)?)
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

/// Deserialize the HashMap of variables. By default, this
/// function won't modify the K or the V of the HashMap. If
/// V starts with $, its value will be read from the server
/// environment variables
fn read_environment_variables<'de, D>(
    deserializer: D,
) -> core::result::Result<HashMap<String, String>, D::Error>
where
    D: Deserializer<'de>,
{
    let result: core::result::Result<Option<HashMap<String, String>>, D::Error> =
        Deserialize::deserialize(deserializer);

    match result {
        Ok(value) => match value {
            Some(mut options) => {
                for (_, value) in options.iter_mut() {
                    // Read the value from the environment variables if available.
                    // If not, it will default to an empty string
                    if value.starts_with('$') && !value.contains(' ') {
                        // Remove the $
                        value.remove(0);

                        match env::var(&value) {
                            Ok(env_value) => *value = env_value,
                            Err(_) => *value = String::new(),
                        }
                    }
                }

                Ok(options)
            }
            None => Ok(HashMap::new()),
        },
        Err(err) => Err(err),
    }
}
