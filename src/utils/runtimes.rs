// Copyright 2023 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use anyhow::{anyhow, Result};
use std::{env, path::Path};
use wws_config::Config;
use wws_project::{check_runtime, install_runtime, metadata::Repository};

use crate::commands::runtimes::Runtimes;

/// Default repository name
pub const DEFAULT_REPO_NAME: &str = "wasmlabs";
/// Default repository URL
pub const DEFAULT_REPO_URL: &str = "https://workers.wasmlabs.dev/repository/v1/index.toml";

/// Environment variable to set the repository name
pub const WWS_REPO_NAME: &str = "WWS_REPO_NAME";
pub const WWS_REPO_URL: &str = "WWS_REPO_URL";

/// Loads the local configuration and installs any missing runtime from it.
/// It will check all the different repositories and install missing
/// runtimes inside them.
pub async fn install_missing_runtimes(project_root: &Path) -> Result<()> {
    println!("⚙️  Checking local configuration...");
    // Retrieve the configuration
    let config = Config::load(project_root)?;

    for repo in &config.repositories {
        for runtime in &repo.runtimes {
            let is_installed = check_runtime(project_root, &repo.name, runtime);

            if !is_installed {
                println!(
                    "🚀 Installing: {} - {} / {}",
                    &repo.name, &runtime.name, &runtime.version
                );
                install_runtime(project_root, &repo.name, runtime).await?;
            }
        }
    }

    println!("✅ Done");
    Ok(())
}

/// Retrieves the remote repository and installs the desired runtime.
/// It will return an error if the desired runtime is not present in
/// the repo.
pub async fn install_from_repository(
    project_root: &Path,
    args: &Runtimes,
    name: &str,
    version: &str,
) -> Result<()> {
    let repo_name = get_repo_name(args);
    let repo_url = get_repo_url(args);

    println!("⚙️  Fetching data from the repository...");
    let repo = Repository::from_remote_file(&repo_url).await?;
    let runtime = repo.find_runtime(name, version);

    if let Some(runtime) = runtime {
        if check_runtime(project_root, &repo_name, runtime) {
            println!("✅ The runtime is already installed");
            Ok(())
        } else {
            println!("🚀 Installing the runtime...");
            install_runtime(project_root, &repo_name, runtime).await?;

            // Update the configuration
            let mut config = Config::load(project_root)?;
            config.save_runtime(&repo_name, &repo_url, runtime);
            config.save(project_root)?;

            println!("✅ Done");
            Ok(())
        }
    } else {
        Err(anyhow!(
            "The runtime with name = '{}' and version = '{}' is not present in the repository",
            name,
            version
        ))
    }
}

/// Utility to retrieve the repository name for the given command.
/// It will look first for the flag and fallback to the default value.
pub fn get_repo_name(args: &Runtimes) -> String {
    let default_value = env::var(WWS_REPO_NAME).unwrap_or_else(|_| DEFAULT_REPO_NAME.into());
    args.repo_name
        .as_ref()
        .unwrap_or(&default_value)
        .to_string()
}

/// Utility to retrieve the repository url for the given command.
/// It will look first for the flag and fallback to the default value.
pub fn get_repo_url(args: &Runtimes) -> String {
    let default_value = env::var(WWS_REPO_URL).unwrap_or_else(|_| DEFAULT_REPO_URL.into());
    args.repo_url.as_ref().unwrap_or(&default_value).to_string()
}
