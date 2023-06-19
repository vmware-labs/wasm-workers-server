// Copyright 2022-2023 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

mod fetch;
pub mod metadata;
pub mod options;
pub mod types;

use anyhow::{bail, Result};
use fetch::fetch_and_validate;
use metadata::{RemoteFile, Runtime};
use options::Options;
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};
use types::git::prepare_git_project;
use wws_store::Store;

pub enum ProjectType {
    Local,
    Git,
}

/// Prepare a project from the given String. This argument could represent
/// different things:
///
/// - A local path
/// - A git repository
/// - Etc.
///
/// Depending on the type, the project preparation requires different steps.
/// For example, a git repository requires to clone it.
///
/// However, the result of any type is the same: a local folder to point to.
/// This is the value we return from this function.
pub async fn prepare_project(
    location: &Path,
    force_type: Option<ProjectType>,
    options: Options,
) -> Result<PathBuf> {
    let project_type = if force_type.is_some() {
        force_type.unwrap()
    } else {
        identify_type(location)?
    };

    match project_type {
        ProjectType::Local => Ok(PathBuf::from(location)),
        ProjectType::Git => prepare_git_project(location, options),
    }
}

/// Identify the type of the project based on different rules related to the location.
/// For example, an URL that ends in .git is considered a git repository. For any
/// unknown pattern, it will default to "Local"
pub fn identify_type(location: &Path) -> Result<ProjectType> {
    if (location.starts_with("https://") || location.starts_with("http://"))
        && location
            .extension()
            .filter(|e| *e == OsStr::new("git"))
            .is_some()
    {
        Ok(ProjectType::Git)
    } else {
        let path = Path::new(location);

        if path.exists() {
            Ok(ProjectType::Local)
        } else {
            bail!("The given path does not exist in the local filesystem.")
        }
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use path_slash::PathBufExt as _;

    #[test]
    fn identify_local_locations() {
        let tests = ["tests", "tests/data", "./tests", "./tests/data"];

        for test in tests {
            let file_route = PathBuf::from_slash(test);

            match identify_type(&file_route) {
                Ok(project_type) => {
                    assert!(matches!(project_type, ProjectType::Local));
                }
                Err(err) => panic!("Error identifying a the project type: {err}"),
            }
        }
    }

    #[test]
    fn identify_local_error_when_missing() {
        let tests = [
            "missing",
            "missing/missing",
            "./missing/missing",
            "./missing/missing",
        ];

        for test in tests {
            let file_route = PathBuf::from_slash(test);

            match identify_type(&file_route) {
                Ok(_) => {
                    panic!("The folder doesn't exist, so identifying it should fail.");
                }
                Err(err) => assert!(err.to_string().contains("does not exist")),
            }
        }
    }

    #[test]
    fn identify_git_repository_locations() {
        let location = Path::new("https://github.com/vmware-labs/wasm-workers-server.git");

        match identify_type(location) {
            Ok(project_type) => {
                assert!(matches!(project_type, ProjectType::Git));
            }
            Err(err) => panic!("Error identifying a the project type: {err}"),
        }
    }
}
