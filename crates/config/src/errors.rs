// Copyright 2023 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

pub type Result<T> = std::result::Result<T, ConfigError>;

#[derive(Debug)]
pub enum ConfigError {
    CouldNotLoadConfig(std::io::Error),
    CouldNotParseConfig(toml::de::Error),
    CouldNotSaveConfig,
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CouldNotLoadConfig(err) => write!(f, "Could not load configuration: {}", err),
            Self::CouldNotParseConfig(err) => write!(f, "Could not parse configuration: {}", err),
            Self::CouldNotSaveConfig => write!(f, "Could not save configuration"),
        }
    }
}

impl From<std::io::Error> for ConfigError {
    fn from(error: std::io::Error) -> Self {
        ConfigError::CouldNotLoadConfig(error)
    }
}

impl From<toml::de::Error> for ConfigError {
    fn from(error: toml::de::Error) -> ConfigError {
        ConfigError::CouldNotParseConfig(error)
    }
}

impl From<toml::ser::Error> for ConfigError {
    fn from(_: toml::ser::Error) -> ConfigError {
        ConfigError::CouldNotSaveConfig
    }
}
