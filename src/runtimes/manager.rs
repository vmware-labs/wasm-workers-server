//// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use super::{
    metadata::{RemoteFile, Runtime as RuntimeMetadata},
    modules::{javascript::JavaScriptRuntime, native::NativeRuntime},
    runtime::Runtime,
};
use crate::{fetch::fetch_and_validate, store::Store};
use anyhow::{anyhow, Result};
use std::path::Path;

// A collection of methods to manage runtimes

/// Initializes a runtime based on the file extension. In the future,
/// This will contain a more complete struct that will identify local
/// runtimes.
pub fn init_runtime(project_root: &Path, path: &Path) -> Result<Box<dyn Runtime + Sync + Send>> {
    if let Some(ext) = path.extension() {
        let ext_as_str = ext.to_str().unwrap();

        match ext_as_str {
            "js" => Ok(Box::new(JavaScriptRuntime::new(
                project_root,
                path.to_path_buf(),
            )?)),
            "wasm" => Ok(Box::new(NativeRuntime::new(path.to_path_buf()))),
            _ => Err(anyhow!(format!(
                "The '{}' extension does not have an associated runtime",
                ext_as_str
            ))),
        }
    } else {
        Err(anyhow!("The given file does not have a valid extension"))
    }
}

// TODO: Remove it when implementing the full logic
#[allow(dead_code)]
// Install a given runtime based on its metadata
pub async fn install_runtime(
    project_root: &Path,
    repository: &str,
    metadata: &RuntimeMetadata,
) -> Result<()> {
    let store = Store::new(
        project_root,
        &["runtimes", repository, &metadata.name, &metadata.version],
    )?;

    // Install the different files
    download_file(&metadata.binary, &store).await?;

    if let Some(polyfill) = &metadata.polyfill {
        download_file(polyfill, &store).await?;
    }

    if let Some(template) = &metadata.template {
        download_file(template, &store).await?;
    }

    Ok(())
}

// TODO: Remove it when implementing the full logic
async fn download_file(file: &RemoteFile, store: &Store) -> Result<()> {
    let contents = fetch_and_validate(&file.url, &file.checksum).await?;
    store.write(&[&file.filename], &contents)
}
