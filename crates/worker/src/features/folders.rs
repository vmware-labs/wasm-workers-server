// Copyright 2023 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Deserializer};
use std::path::PathBuf;

/// A folder to mount in the worker
#[derive(Deserialize, Clone, Default)]
pub struct Folder {
    /// Local folder
    #[serde(deserialize_with = "deserialize_path", default)]
    pub from: PathBuf,
    /// Mount point
    pub to: String,
}

/// Deserialize a valid path for the given platform. This method checks and
/// split the path by the different separators and join the final path
/// using the current OS requirements.
fn deserialize_path<'de, D>(deserializer: D) -> Result<PathBuf, D::Error>
where
    D: Deserializer<'de>,
{
    let result: Result<String, D::Error> = Deserialize::deserialize(deserializer);

    match result {
        Ok(value) => {
            let split = if value.contains('/') {
                // Unix separator
                value.split('/')
            } else {
                // Windows separator
                value.split('\\')
            };

            Ok(split.fold(PathBuf::new(), |mut acc, el| {
                acc.push(el);
                acc
            }))
        }
        Err(err) => Err(err),
    }
}
