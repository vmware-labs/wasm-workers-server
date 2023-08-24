// Copyright 2023 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, StoreError>;

#[derive(Debug)]
pub enum StoreError {
    CannotCreateDirectory {
        path: PathBuf,
        error: std::io::Error,
    },
    CannotDeleteDirectory {
        path: PathBuf,
        error: std::io::Error,
    },
    CannotReadFile {
        path: PathBuf,
        error: std::io::Error,
    },
    CannotWriteFile {
        path: PathBuf,
        error: std::io::Error,
    },
}

impl std::fmt::Display for StoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CannotCreateDirectory { path, error } => {
                write!(
                    f,
                    "Could not create directory {}: {}",
                    path.display(),
                    error
                )
            }
            Self::CannotDeleteDirectory { path, error } => {
                write!(
                    f,
                    "Could not delete directory {}: {}",
                    path.display(),
                    error
                )
            }
            Self::CannotReadFile { path, error } => {
                write!(f, "Could not read file {}: {}", path.display(), error)
            }
            Self::CannotWriteFile { path, error } => {
                write!(f, "Could not write to file {}: {}", path.display(), error)
            }
        }
    }
}
