// Copyright 2023 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, StoreError>;

#[derive(Debug)]
pub enum StoreError {
    CouldNotCreateDirectory {
        path: PathBuf,
        error: std::io::Error,
    },
    CouldNotDeleteDirectory {
        path: PathBuf,
        error: std::io::Error,
    },
    CouldNotReadFile {
        path: PathBuf,
        error: std::io::Error,
    },
    CouldNotWriteFile {
        path: PathBuf,
        error: std::io::Error,
    },
}

impl std::fmt::Display for StoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CouldNotCreateDirectory { path, error } => {
                write!(
                    f,
                    "Could not create directory {}: {}",
                    path.display(),
                    error
                )
            }
            Self::CouldNotDeleteDirectory { path, error } => {
                write!(
                    f,
                    "Could not delete directory {}: {}",
                    path.display(),
                    error
                )
            }
            Self::CouldNotReadFile { path, error } => {
                write!(f, "Could not read file {}: {}", path.display(), error)
            }
            Self::CouldNotWriteFile { path, error } => {
                write!(f, "Could not write to file {}: {}", path.display(), error)
            }
        }
    }
}
