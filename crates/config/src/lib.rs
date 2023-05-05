// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};
use wws_runtimes_manager::{check_runtime, metadata::Runtime};

/// Default repository name
pub const DEFAULT_REPO_NAME: &str = "wasmlabs";
/// Default repository URL
pub const DEFAULT_REPO_URL: &str = "https://workers.wasmlabs.dev/repository/v1/index.toml";

/// Config file name
const CONFIG_FILENAME: &str = ".wws.toml";

/// Loads the data from the Project definition file or .wws.toml.
/// This file contains information about the different runtimes
/// required for this project. You can think of those as dependencies.
///
/// If your project requires to run workers using any interpreted
/// language (except Js, which it's embedded), you will need to install
/// a language runtime.
///
/// For reproducibility, this file can be commited to the project
/// repository so other developers can download them directly.
#[derive(Deserialize, Serialize)]
pub struct Config {
    /// Version of the .wws file
    version: u32,
    /// List of repositories
    pub repositories: Vec<ConfigRepository>,
}

impl Config {
    /// Load the config file if it's present. It not, it will create a
    /// new empty config.
    pub fn load(project_root: &Path) -> Result<Self> {
        let config_path = Self::config_path(project_root);

        if config_path.exists() {
            toml::from_str(&fs::read_to_string(config_path)?).map_err(|_| {
                anyhow!("Error opening the .wws.toml file. The file format is not correct")
            })
        } else {
            Ok(Self::default())
        }
    }

    /// Save a new installed runtime
    pub fn save_runtime(&mut self, repo_name: &str, repo_url: &str, runtime: &Runtime) {
        let repo = self.repositories.iter_mut().find(|r| r.name == repo_name);

        // Shadow to init an empty one if required
        match repo {
            Some(r) => r.runtimes.push(runtime.clone()),
            None => {
                let new_repo = ConfigRepository {
                    name: repo_name.to_string(),
                    url: repo_url.to_string(),
                    runtimes: vec![runtime.clone()],
                };

                self.repositories.push(new_repo);
            }
        };
    }

    /// Remove an existing runtime if it's present.
    pub fn remove_runtime(&mut self, repository: &str, name: &str, version: &str) {
        let repo = self.repositories.iter_mut().find(|r| r.name == repository);

        // Shadow to init an empty one if required
        if let Some(repo) = repo {
            repo.runtimes
                .retain(|r| r.name != name && r.version != version);
        };
    }

    /// Get a given runtime from the current configuration if it's available.
    pub fn get_runtime(&self, repository: &str, name: &str, version: &str) -> Option<&Runtime> {
        let repo = self.repositories.iter().find(|r| r.name == repository);

        if let Some(repo) = repo {
            repo.runtimes
                .iter()
                .find(|r| r.name == name && r.version == version)
        } else {
            None
        }
    }

    /// Check if there're missing runtimes based on the current configuration
    pub fn is_missing_any_runtime(&self, project_root: &Path) -> bool {
        for repo in &self.repositories {
            if repo
                .runtimes
                .iter()
                .any(|r| !check_runtime(project_root, &repo.name, r))
            {
                return true;
            }
        }

        false
    }

    /// Write the current configuration into the `.wws.toml` file. It will
    /// store it in the project root folder
    pub fn save(&self, project_root: &Path) -> Result<()> {
        let contents = toml::to_string_pretty(self)?;

        fs::write(Self::config_path(project_root), contents)
            .map_err(|_| anyhow!("Error saving the .wws.toml file"))
    }

    /// Retrieve the configuration path from the project root
    fn config_path(project_root: &Path) -> PathBuf {
        project_root.join(CONFIG_FILENAME)
    }

    /// Provides a list of all file extensions handled by the runtimes
    /// that are currently installed in `project_root`
    pub fn get_runtime_extensions(&self, project_root: &Path) -> Vec<String> {
        let mut extensions: Vec<String> = vec![String::from("js"), String::from("wasm")];

        for repo in &self.repositories {
            for runtime in &repo.runtimes {
                for ext in &runtime.extensions {
                    if check_runtime(project_root, &repo.name, runtime) && !extensions.contains(ext)
                    {
                        extensions.push(ext.clone());
                    }
                }
            }
        }

        extensions
    }
}

impl Default for Config {
    // Initialize an empty repository by default
    fn default() -> Self {
        let new_repo = ConfigRepository {
            name: DEFAULT_REPO_NAME.to_string(),
            url: DEFAULT_REPO_URL.to_string(),
            runtimes: Vec::new(),
        };

        Self {
            version: 1,
            repositories: vec![new_repo],
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct ConfigRepository {
    /// Local name to identify the repository. It avoids collisions when installing
    /// language runtimes
    pub name: String,
    /// Set the url from which this repository was downloaded
    url: String,
    /// Installed runtimes
    pub runtimes: Vec<Runtime>,
}
