// Copyright 2023 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, WorkerError>;

#[derive(Debug)]
pub enum WorkerError {
    BadWasmModule,
    CannotLoadConfig,
    CannotParseConfig {
        path: PathBuf,
        error: toml::de::Error,
    },
    ConfigureRuntimeError,
    DeserializeConfigError,
    FailedToInitialize,
    RuntimeError(wws_runtimes::errors::RuntimeError),
    WorkerBodyReadError,
}

impl From<toml::de::Error> for WorkerError {
    fn from(_error: toml::de::Error) -> Self {
        WorkerError::CannotLoadConfig
    }
}

impl From<wws_runtimes::errors::RuntimeError> for WorkerError {
    fn from(error: wws_runtimes::errors::RuntimeError) -> Self {
        WorkerError::RuntimeError(error)
    }
}

impl std::fmt::Display for WorkerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WorkerError::BadWasmModule => write!(f, "Bad Wasm module"),
            WorkerError::CannotLoadConfig => write!(f, "Could not load configuration"),
            WorkerError::CannotParseConfig { path, error } => write!(
                f,
                "Could not parse configuration at {:?}: {:?}",
                path, error
            ),
            WorkerError::ConfigureRuntimeError => write!(f, "Error configuring runtime"),
            WorkerError::DeserializeConfigError => write!(f, "Error deserializing configuration"),
            WorkerError::FailedToInitialize => write!(f, "Failed to initialize"),
            WorkerError::RuntimeError(error) => {
                write!(f, "Error on Wasm module runtime: {:?}", error)
            }
            WorkerError::WorkerBodyReadError => write!(f, "Error reading body from worker"),
        }
    }
}
