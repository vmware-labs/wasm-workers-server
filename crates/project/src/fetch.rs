// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::errors::Result;
use crate::metadata::Checksum;
use reqwest::header::USER_AGENT;

/// The current wws version
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Fetch the contents of a given file and validates it
/// using the Sha256.
pub async fn fetch<T: AsRef<str>>(file: T) -> Result<Vec<u8>> {
    let client = reqwest::Client::new();
    let user_agent_value = format!("Wasm Workers Server/{VERSION}");

    let body: Vec<u8> = client
        .get(file.as_ref())
        .header(USER_AGENT, user_agent_value)
        .send()
        .await?
        .bytes()
        .await?
        .into();

    Ok(body)
}

/// Fetch the contents of a given file and validates it
/// using the Sha256.
pub async fn fetch_and_validate<T: AsRef<str>>(file: T, checksum: &Checksum) -> Result<Vec<u8>> {
    let body: Vec<u8> = fetch(file).await?;
    checksum.validate(&body)?;

    Ok(body)
}
