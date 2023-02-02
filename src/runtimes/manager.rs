//// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use super::{
    metadata::{RemoteFile, Runtime as RuntimeMetadata},
    modules::{external::ExternalRuntime, javascript::JavaScriptRuntime, native::NativeRuntime},
    runtime::Runtime,
};
use crate::{config::Config, fetch::fetch_and_validate, store::Store};
use anyhow::{anyhow, Result};
use std::path::Path;

// A collection of methods to manage runtimes

/// Initializes a runtime based on the file extension. In the future,
/// This will contain a more complete struct that will identify local
/// runtimes.
pub fn init_runtime(
    project_root: &Path,
    path: &Path,
    config: &Config,
) -> Result<Box<dyn Runtime + Sync + Send>> {
    if let Some(ext) = path.extension() {
        let ext_as_str = ext.to_str().unwrap();

        match ext_as_str {
            "js" => Ok(Box::new(JavaScriptRuntime::new(
                project_root,
                path.to_path_buf(),
            )?)),
            "wasm" => Ok(Box::new(NativeRuntime::new(path.to_path_buf()))),
            other => {
                let mut runtime_config = None;
                let mut repo_name = "";
                let other_string = other.to_string();

                'outer: for repo in &config.repositories {
                    for r in &repo.runtimes {
                        if r.extensions.contains(&other_string) {
                            runtime_config = Some(r);
                            repo_name = &repo.name;
                            break 'outer;
                        }
                    }
                }

                if let Some(runtime_config) = runtime_config {
                    Ok(Box::new(ExternalRuntime::new(
                        project_root,
                        path.to_path_buf(),
                        repo_name,
                        runtime_config.clone(),
                    )?))
                } else {
                    Err(anyhow!(format!(
                        "The '{ext_as_str}' extension does not have an associated runtime"
                    )))
                }
            }
        }
    } else {
        Err(anyhow!("The given file does not have a valid extension"))
    }
}

// Install a given runtime based on its metadata
pub async fn install_runtime(
    project_root: &Path,
    repository: &str,
    metadata: &RuntimeMetadata,
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

/// Checks if the given [Runtime] is already installed locally.
pub fn check_runtime(project_root: &Path, repository: &str, runtime: &RuntimeMetadata) -> bool {
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

// Install a given runtime based on its metadata
pub fn uninstall_runtime(
    project_root: &Path,
    repository: &str,
    metadata: &RuntimeMetadata,
) -> Result<()> {
    // Delete the current folder
    Store::new(
        project_root,
        &["runtimes", repository, &metadata.name, &metadata.version],
    )
    .delete_root_folder()
}

/// Downloads a remote file in the given [Store].
async fn download_file(file: &RemoteFile, store: &Store) -> Result<()> {
    let contents = fetch_and_validate(&file.url, &file.checksum).await?;
    store.write(&[&file.filename], &contents)
}
