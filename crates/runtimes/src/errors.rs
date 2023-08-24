// Copyright 2023 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

pub type Result<T> = std::result::Result<T, RuntimeError>;

#[derive(Debug)]
pub enum RuntimeError {
    CannotReadModule,
    InvalidExtension { extension: Option<String> },
    InvalidWrapper,
    IOError(std::io::Error),
    MissingRuntime { extension: String },
    StoreError(wws_store::errors::StoreError),
    WasiContextError,
    WasiError(Option<wasmtime_wasi::Error>),
}

impl From<wws_store::errors::StoreError> for RuntimeError {
    fn from(error: wws_store::errors::StoreError) -> Self {
        RuntimeError::StoreError(error)
    }
}

impl From<std::string::FromUtf8Error> for RuntimeError {
    fn from(_error: std::string::FromUtf8Error) -> Self {
        RuntimeError::InvalidWrapper
    }
}

impl From<std::io::Error> for RuntimeError {
    fn from(error: std::io::Error) -> Self {
        RuntimeError::IOError(error)
    }
}

impl From<wasmtime_wasi::Error> for RuntimeError {
    fn from(error: wasmtime_wasi::Error) -> Self {
        RuntimeError::WasiError(Some(error))
    }
}
