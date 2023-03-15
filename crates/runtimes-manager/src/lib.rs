// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

mod fetch;
pub mod metadata;

use anyhow::Result;
use fetch::fetch_and_validate;
use metadata::{RemoteFile, Runtime};
use std::path::Path;
use wws_store::Store;

/// Install a runtime locally. It reads the provided configuration and
/// dowload the files. All files are saved in a store that references
/// the repository, the runtime name and version
pub async fn install_runtime(
    project_root: &Path,
    repository: &str,
    metadata: &Runtime,
) -> Result<()> {
    let store = Store::create(
        project_root,
        &["runtimes", repository, &metadata.name, &metadata.version],
    )?;

    // Install the different files
    download_file(&metadata.binary, &store).await?;

    if let Some(polyfill) = &metadata.polyfill {
        download_file(polyfill, &store).await?;
    }

    if let Some(wrapper) = &metadata.wrapper {
        download_file(wrapper, &store).await?;
    }

    if let Some(template) = &metadata.template {
        download_file(template, &store).await?;
    }

    Ok(())
}

/// Checks if the given runtime is already installed locally. It loads
/// the metadata and try to find the files in the store.
pub fn check_runtime(project_root: &Path, repository: &str, runtime: &Runtime) -> bool {
    // Check the different files
    let store = Store::new(
        project_root,
        &["runtimes", repository, &runtime.name, &runtime.version],
    );

    // Check the existence of the different files
    let binary = store.check_file(&[&runtime.binary.filename]);
    let mut template = true;
    let mut polyfill = true;
    let mut wrapper = true;

    if let Some(template_file) = &runtime.template {
        template = store.check_file(&[&template_file.filename]);
    }

    if let Some(wrapper_file) = &runtime.wrapper {
        wrapper = store.check_file(&[&wrapper_file.filename]);
    }

    if let Some(polyfill_file) = &runtime.polyfill {
        polyfill = store.check_file(&[&polyfill_file.filename]);
    }

    binary && template && polyfill && wrapper
}

/// Uninstall a runtime from the store. I loads the path from the metadata
/// and delete the folder.
pub fn uninstall_runtime(project_root: &Path, repository: &str, metadata: &Runtime) -> Result<()> {
    // Delete the current folder
    Store::new(
        project_root,
        &["runtimes", repository, &metadata.name, &metadata.version],
    )
    .delete_root_folder()
}

/// Downloads a remote file and saves into the given store. This
/// method also validates the file against the checksum provided
/// in the metadata.
async fn download_file(file: &RemoteFile, store: &Store) -> Result<()> {
    let contents = fetch_and_validate(&file.url, &file.checksum).await?;
    store.write(&[&file.filename], &contents)
}
