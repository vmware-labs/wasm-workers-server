// Copyright 2023 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use wws_store::errors::StoreError;

pub type Result<T> = std::result::Result<T, FetchError>;

#[derive(Debug)]
pub enum FetchError {
    DefaultBranchMissing,
    GitError(git2::Error),
    HttpError(reqwest::Error),
    InvalidURL,
    InvalidReusedRepository,
    InvalidRepository,
    InvalidChecksum,
    MissingPathInFilesystem,
    StoreError(StoreError),
}

impl std::fmt::Display for FetchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DefaultBranchMissing => write!(
                f,
                "Couldn't find the default main branch. Please, set the Git branch you want to use"
            ),
            Self::GitError(err) => write!(f, "Git error: {}", err),
            Self::HttpError(err) => write!(f, "HTTP error: {}", err),
            Self::InvalidURL => write!(f, "Invalid URL"),
            Self::InvalidReusedRepository => write!(f, "Invalid local copy of repository"),
            Self::InvalidRepository => write!(f, "Invalid repository"),
            Self::InvalidChecksum => write!(f, "Invalid checksum"),
            Self::MissingPathInFilesystem => write!(f, "Missing path in filesystem"),
            Self::StoreError(err) => write!(f, "Store error: {}", err),
        }
    }
}

impl From<git2::Error> for FetchError {
    fn from(error: git2::Error) -> Self {
        FetchError::GitError(error)
    }
}

impl From<url::ParseError> for FetchError {
    fn from(_error: url::ParseError) -> Self {
        FetchError::InvalidURL
    }
}

impl From<reqwest::Error> for FetchError {
    fn from(error: reqwest::Error) -> Self {
        FetchError::HttpError(error)
    }
}

impl From<StoreError> for FetchError {
    fn from(error: StoreError) -> Self {
        FetchError::StoreError(error)
    }
}

impl From<std::string::FromUtf8Error> for FetchError {
    fn from(_error: std::string::FromUtf8Error) -> Self {
        FetchError::InvalidRepository
    }
}
