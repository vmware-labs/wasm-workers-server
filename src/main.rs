// Copyright 2022-2023 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

mod commands;
mod utils;

use crate::utils::options;
use crate::utils::runtimes::install_missing_runtimes;
use clap::Parser;
use commands::main::Main;
use commands::runtimes::RuntimesCommands;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use std::process::exit;
use wws_config::Config;
use wws_project::{identify_type, prepare_project, ProjectType};
use wws_router::Routes;
use wws_server::serve;

// Arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Hostname to initiate the server
    #[arg(long = "host", default_value = "127.0.0.1")]
    hostname: String,

    /// Port to initiate the server
    #[arg(short, long, default_value_t = 8080)]
    port: u16,

    /// Location of the wws project. It could be a local folder or a git repository.
    #[arg(value_parser, default_value = ".")]
    path: PathBuf,

    /// Prepend the given path to all URLs
    #[arg(long, default_value = "")]
    prefix: String,

    /// Patterns to ignore when looking for worker files
    #[arg(long, default_value = "")]
    ignore: Vec<String>,

    /// Install missing runtimes automatically.
    #[arg(long, short)]
    install_runtimes: bool,

    /// Set the commit when using a git repository as project
    #[arg(long)]
    git_commit: Option<String>,

    /// Set the tag when using a git repository as project
    #[arg(long)]
    git_tag: Option<String>,

    /// Set the branch when using a git repository as project
    #[arg(long)]
    git_branch: Option<String>,

    /// Change the directory when using a git repository as project
    #[arg(long)]
    git_folder: Option<String>,

    /// Enable the administration panel
    #[arg(long)]
    enable_panel: bool,

    /// Manage language runtimes in your project
    #[command(subcommand)]
    commands: Option<Main>,

    /// CORS headers to add to all workers if not already set by the worker
    #[arg(long)]
    cors: Option<Vec<String>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // Check the given subcommand
    if let Some(Main::Runtimes(sub)) = &args.commands {
        let mut run_result = Ok(());

        match identify_type(&args.path) {
            Ok(project_type) => match project_type {
                ProjectType::Local => {}
                _ => {
                    eprintln!("❌ You can only manage runtimes in local projects");
                    exit(1);
                }
            },
            Err(err) => {
                eprintln!("❌ There was an error preparing the project: {err}");

                exit(1);
            }
        }

        match &sub.runtime_commands {
            RuntimesCommands::List(list) => {
                if let Err(err) = list.run(sub).await {
                    println!("❌ There was an error listing the runtimes from the repository");
                    println!("👉 {err}");
                    run_result = Err(Error::new(ErrorKind::InvalidData, ""));
                }
            }
            RuntimesCommands::Install(install) => {
                if let Err(err) = install.run(&args.path, sub).await {
                    println!("❌ There was an error installing the runtime from the repository");
                    println!("👉 {err}");
                    run_result = Err(Error::new(ErrorKind::InvalidData, ""));
                }
            }
            RuntimesCommands::Uninstall(uninstall) => {
                if let Err(err) = uninstall.run(&args.path, sub) {
                    println!("❌ There was an error uninstalling the runtime");
                    println!("👉 {err}");
                    run_result = Err(Error::new(ErrorKind::InvalidData, ""));
                }
            }
            RuntimesCommands::Check(check) => {
                if let Err(err) = check.run(&args.path) {
                    println!("❌ There was an error checking the local runtimes");
                    println!("👉 {err}");
                    run_result = Err(Error::new(ErrorKind::InvalidData, ""));
                }
            }
        };

        run_result
    } else {
        // TODO(Angelmmiguel): refactor this into a separate command!

        // Set the final options
        let project_opts = options::build_project_options(&args);

        println!("⚙️  Preparing the project from: {}", &args.path.display());
        let project_path = match prepare_project(&args.path, None, project_opts).await {
            Ok(p) => p,
            Err(err) => {
                eprintln!("❌ There was an error preparing the project: {err}");

                exit(1);
            }
        };

        // Loading the local configuration if available.
        let config = match Config::load(&project_path) {
            Ok(c) => c,
            Err(err) => {
                println!("⚠️  There was an error reading the .wws.toml file. It will be ignored");
                println!("⚠️  Error: {err}");

                Config::default()
            }
        };

        // Check if there're missing runtimes
        if config.is_missing_any_runtime(&project_path) {
            if args.install_runtimes {
                match install_missing_runtimes(&project_path).await {
                    Ok(_) => {}
                    Err(err) => {
                        eprintln!("❌ There was an error installing the missing runtimes: {err}");

                        exit(1);
                    }
                }
            } else {
                println!("⚠️  Required language runtimes are not installed. Some files may not be considered workers");
                println!("⚠️  You can install the missing runtimes adding the --install-runtimes / -i flag");
            }
        }

        println!("⚙️  Loading routes from: {}", &project_path.display());
        let routes = Routes::new(&project_path, &args.prefix, args.ignore, &config);
        for route in routes.routes.iter() {
            println!(
                "    - http://{}:{}{}\n      => {}",
                &args.hostname,
                args.port,
                route.path,
                route.handler.display()
            );
        }

        if args.enable_panel {
            println!(
                "🎛️  The admin panel is available at http://{}:{}/_panel/",
                &args.hostname, args.port
            );
        }

        let server = serve(
            &project_path,
            routes,
            &args.hostname,
            args.port,
            args.enable_panel,
            None,
            args.cors,
        )
        .await
        .map_err(|err| Error::new(ErrorKind::AddrInUse, err))?;

        println!(
            "🚀 Start serving requests at http://{}:{}\n",
            &args.hostname, args.port
        );

        // Run the server
        server.await
    }
}
