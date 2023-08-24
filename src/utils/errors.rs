// Copyright 2023 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use anyhow::anyhow;
use wws_config::errors::ConfigError;
use wws_project::errors::FetchError;

pub type Result<T> = std::result::Result<T, UtilsError>;

#[derive(Debug)]
pub enum UtilsError {
    ConfigError(wws_config::errors::ConfigError),
    FetchError(FetchError),
    MissingRuntime { runtime: String, version: String },
}

impl std::fmt::Display for UtilsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConfigError(err) => write!(f, "Configuration error: {}", err),
            Self::FetchError(err) => write!(f, "Fetch error: {}", err),
            Self::MissingRuntime { runtime, version } => {
                write!(f, "Missing runtime {} with version {}", runtime, version)
            }
        }
    }
}

impl From<wws_config::errors::ConfigError> for UtilsError {
    fn from(error: wws_config::errors::ConfigError) -> Self {
        UtilsError::ConfigError(error)
    }
}

impl From<FetchError> for UtilsError {
    fn from(error: FetchError) -> Self {
        UtilsError::FetchError(error)
    }
}

impl From<UtilsError> for anyhow::Error {
    fn from(error: UtilsError) -> Self {
        match error {
            UtilsError::ConfigError(error) => match error {
                ConfigError::CannotLoadConfig(error) => anyhow!("Error opening file: {}", error),
                ConfigError::CannotParseConfig(error) => anyhow!(
                    "Error loading file: {}. The file format is not correct",
                    error
                ),
                ConfigError::CannotSaveConfig => anyhow!("Error saving configuration"),
            },
            UtilsError::FetchError(error) => anyhow!("Error fetching repository: {}", error),
            UtilsError::MissingRuntime { runtime, version } => anyhow!(
                "The runtime with name = '{}' and version = '{}' is not present in the repository",
                runtime,
                version
            ),
        }
    }
}
