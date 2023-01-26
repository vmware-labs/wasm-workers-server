// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::runtimes::metadata::Checksum;
use anyhow::Result;

// TODO: Remove it when implementing the manager
#[allow(dead_code)]
/// Fetch the contents of a given file and validates it
/// using the Sha256.
pub async fn fetch<T: AsRef<str>>(file: T) -> Result<Vec<u8>> {
    let body: Vec<u8> = reqwest::get(file.as_ref()).await?.bytes().await?.into();

    Ok(body)
}

// TODO: Remove it when implementing the manager
#[allow(dead_code)]
/// Fetch the contents of a given file and validates it
/// using the Sha256.
pub async fn fetch_and_validate<T: AsRef<str>>(file: T, checksum: &Checksum) -> Result<Vec<u8>> {
    let body: Vec<u8> = fetch(file).await?;
    checksum.validate(&body)?;

    Ok(body)
}
