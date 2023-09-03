// Copyright 2023 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

pub type Result<T> = std::result::Result<T, ConfigError>;

#[derive(Debug)]
pub enum ConfigError {
    CannotLoadConfig(std::io::Error),
    CannotParseConfig(toml::de::Error),
    CannotSaveConfig,
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CannotLoadConfig(err) => write!(f, "Could not load configuration: {}", err),
            Self::CannotParseConfig(err) => write!(f, "Could not parse configuration: {}", err),
            Self::CannotSaveConfig => write!(f, "Could not save configuration"),
        }
    }
}

impl From<std::io::Error> for ConfigError {
    fn from(error: std::io::Error) -> Self {
        ConfigError::CannotLoadConfig(error)
    }
}

impl From<toml::de::Error> for ConfigError {
    fn from(error: toml::de::Error) -> ConfigError {
        ConfigError::CannotParseConfig(error)
    }
}

impl From<toml::ser::Error> for ConfigError {
    fn from(_: toml::ser::Error) -> ConfigError {
        ConfigError::CannotSaveConfig
    }
}
