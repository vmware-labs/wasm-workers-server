// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::path::Path;

use anyhow::{anyhow, Result};
use clap::{Args, Parser, Subcommand};
use prettytable::{format, Cell, Row, Table};
use std::env;
use wws_config::Config;
use wws_project::{check_runtime, install_runtime, metadata::Repository, uninstall_runtime};

/// Default repository name
pub const DEFAULT_REPO_NAME: &str = "wasmlabs";
/// Default repository URL
pub const DEFAULT_REPO_URL: &str = "https://workers.wasmlabs.dev/repository/v1/index.toml";

/// Environment variable to set the repository name
pub const WWS_REPO_NAME: &str = "WWS_REPO_NAME";
pub const WWS_REPO_URL: &str = "WWS_REPO_URL";

/// Manage the language runtimes in your project
#[derive(Parser, Debug)]
pub struct Runtimes {
    /// Set a different repository URL
    #[arg(long)]
    repo_url: Option<String>,
    /// Gives a name to the given repository URL
    #[arg(long)]
    repo_name: Option<String>,

    #[command(subcommand)]
    pub runtime_commands: RuntimesCommands,
}

#[derive(Subcommand, Debug)]
pub enum RuntimesCommands {
    Install(Install),
    List(List),
    Check(Check),
    Uninstall(Uninstall),
}

/// Install a new language runtime (like Ruby, Python, etc)
#[derive(Args, Debug)]
pub struct Install {
    /// Name of the desired runtime
    pub name: Option<String>,
    /// Version of the desired runtime
    pub version: Option<String>,
}

impl Install {
    /// Install the given runtime to the project. It will look for
    /// the runtimes in the defined repository
    pub async fn run(&self, project_root: &Path, args: &Runtimes) -> Result<()> {
        match (&self.name, &self.version) {
            (Some(name), Some(version)) => {
                self.install_from_repository(project_root, args, name, version)
                    .await
            }
            (Some(_), None) | (None, Some(_)) => Err(anyhow!(
                "The name and version are mandatory when installing a runtime from a repository"
            )),
            (None, None) => self.install_missing_runtimes(project_root).await,
        }
    }

    /// Retrieves the remote repository and install the desired runtime.
    /// It will return an error if the desired runtime is not present in
    /// the repo.
    async fn install_from_repository(
        &self,
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

    /// Loads the local configuration and install any missing runtime from it.
    /// It will check all the different repositories and install missing
    /// runtimes inside them.
    async fn install_missing_runtimes(&self, project_root: &Path) -> Result<()> {
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
}

/// List all available runtimes to install. By default, it uses the WebAssembly
/// Language Runtime repository
#[derive(Args, Debug)]
pub struct List {}

impl List {
    /// Retrieve the list of runtimes from the remote repository and
    /// show it as a list
    pub async fn run(&self, args: &Runtimes) -> Result<()> {
        let repo_url = get_repo_url(args);

        println!("⚙️  Fetching data from the repository...");
        let repo = Repository::from_remote_file(&repo_url).await?;

        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_BOX_CHARS);

        table.add_row(Row::new(vec![
            Cell::new("Name"),
            Cell::new("Version"),
            Cell::new("Tags"),
            Cell::new("Extension"),
            Cell::new("Binary"),
        ]));

        for runtime in &repo.runtimes {
            let mut tags = runtime.tags.join(", ");

            if tags.is_empty() {
                tags.push('-');
            }

            table.add_row(Row::new(vec![
                Cell::new(&runtime.name),
                Cell::new(&runtime.version),
                Cell::new(&tags),
                Cell::new(&runtime.extensions.join(", ")),
                Cell::new(&runtime.binary.filename),
            ]));
        }

        table.printstd();

        Ok(())
    }
}

/// List of locally installed runtimes
#[derive(Args, Debug)]
pub struct Check {}

impl Check {
    /// Displays the .wws.toml file dependencies and checks if they are
    /// installed in the current project root.
    pub fn run(&self, project_root: &Path) -> Result<()> {
        // Retrieve the configuration
        let config = Config::load(project_root)?;
        let mut is_missing = false;
        let mut total = 0;

        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_BOX_CHARS);

        table.add_row(Row::new(vec![
            Cell::new("Installed"),
            Cell::new("Name"),
            Cell::new("Version"),
            Cell::new("Tags"),
            Cell::new("Extension"),
            Cell::new("Binary"),
        ]));

        for repo in &config.repositories {
            for runtime in &repo.runtimes {
                let mut tags = runtime.tags.join(", ");
                let is_installed = check_runtime(project_root, &repo.name, runtime);

                if tags.is_empty() {
                    tags.push('-');
                }

                if !is_installed {
                    is_missing = true;
                }

                table.add_row(Row::new(vec![
                    Cell::new(if is_installed { "✅" } else { "❌" }),
                    Cell::new(&runtime.name),
                    Cell::new(&runtime.version),
                    Cell::new(&tags),
                    Cell::new(&runtime.extensions.join(", ")),
                    Cell::new(&runtime.binary.filename),
                ]));

                total += 1;
            }
        }

        table.printstd();

        // Provide a hint
        if is_missing {
            println!("\n💡 Tip: there are missing language runtimes. You can install them with `wws runtimes install`");
        }

        if total == 0 {
            println!("\n💡 Tip: you can check the available language runtimes by running `wws runtimes list`");
        }

        Ok(())
    }
}

/// Uninstall a language runtime
#[derive(Args, Debug)]
pub struct Uninstall {
    /// Name of the desired runtime
    name: String,
    /// Version of the desired runtime
    version: String,
}

impl Uninstall {
    /// Uninstall the given runtime from the local system. This will
    /// remove the files from the `.wws` folder and the runtime metadata
    /// from the .wws.toml file
    pub fn run(&self, project_root: &Path, args: &Runtimes) -> Result<()> {
        // Retrieve the configuration
        let mut config = Config::load(project_root)?;
        let repo_name = get_repo_name(args);
        let runtime = config.get_runtime(&repo_name, &self.name, &self.version);

        if let Some(runtime) = runtime {
            println!(
                "🗑  Uninstalling: {} - {} / {}",
                &repo_name, &runtime.name, &runtime.version
            );
            uninstall_runtime(project_root, &repo_name, runtime)?;
            config.remove_runtime(&repo_name, &self.name, &self.version);
            config.save(project_root)?;
        } else {
            println!(
                "🗑  The runtime was not installed: {} - {} / {}",
                &repo_name, &self.name, &self.version
            );
        }

        println!("✅ Done");
        Ok(())
    }
}

/// Utility to retrieve the repository name for the given command.
/// It will look first for the flag and fallback to the default value.
fn get_repo_name(args: &Runtimes) -> String {
    let default_value = env::var(WWS_REPO_NAME).unwrap_or_else(|_| DEFAULT_REPO_NAME.into());
    args.repo_name
        .as_ref()
        .unwrap_or(&default_value)
        .to_string()
}

/// Utility to retrieve the repository url for the given command.
/// It will look first for the flag and fallback to the default value.
fn get_repo_url(args: &Runtimes) -> String {
    let default_value = env::var(WWS_REPO_URL).unwrap_or_else(|_| DEFAULT_REPO_URL.into());
    args.repo_url.as_ref().unwrap_or(&default_value).to_string()
}
