// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::runtimes::metadata::Runtime;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};

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
            let new_repo = ConfigRepository {
                name: "wlr".to_string(),
                runtimes: Vec::new(),
            };

            Ok(Self {
                version: 1,
                repositories: vec![new_repo],
            })
        }
    }

    /// Save a new installed runtime
    pub fn save_runtime(&mut self, repository: &str, runtime: &Runtime) {
        let repo = self.repositories.iter_mut().find(|r| r.name == repository);

        // Shadow to init an empty one if required
        match repo {
            Some(r) => r.runtimes.push(runtime.clone()),
            None => {
                let new_repo = ConfigRepository {
                    name: repository.to_string(),
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
}

#[derive(Deserialize, Serialize)]
pub struct ConfigRepository {
    /// Local name to identify the repository. It avoids collisions when installing
    /// language runtimes
    pub name: String,
    /// Installed runtimes
    pub runtimes: Vec<Runtime>,
}
