// Copyright 2023 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

pub type Result<T> = std::result::Result<T, ServeError>;

#[derive(Debug)]
pub enum ServeError {
    InitializeServerError,
}

impl std::error::Error for ServeError {}

impl std::fmt::Display for ServeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServeError::InitializeServerError => write!(f, "Error initializing server"),
        }
    }
}
