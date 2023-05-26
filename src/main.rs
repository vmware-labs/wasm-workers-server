// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

mod commands;

use clap::Parser;
use commands::main::Main;
use commands::runtimes::RuntimesCommands;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use wws_config::Config;
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

    /// Folder to read WebAssembly modules from
    #[arg(value_parser, default_value = ".")]
    path: PathBuf,

    /// Prepend the given path to all URLs
    #[arg(long, default_value = "")]
    prefix: String,

    /// Patterns to ignore when looking for worker files
    #[arg(long, default_value = "")]
    ignore: Vec<String>,

    /// Manage language runtimes in your project
    #[command(subcommand)]
    commands: Option<Main>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // Check the given subcommand
    if let Some(Main::Runtimes(sub)) = &args.commands {
        let mut run_result = Ok(());

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
        // Initialize the routes

        // Loading the local configuration if available.
        let config = match Config::load(&args.path) {
            Ok(c) => c,
            Err(err) => {
                println!("⚠️  There was an error reading the .wws.toml file. It will be ignored");
                println!("⚠️  Error: {err}");

                Config::default()
            }
        };

        // Check if there're missing runtimes
        if config.is_missing_any_runtime(&args.path) {
            println!("⚠️  Required language runtimes are not installed. Some files may not be considered workers");
            println!("⚠️  You can install the missing runtimes with: wws runtimes install");
        }

        println!("⚙️  Loading routes from: {}", &args.path.display());
        let routes = Routes::new(&args.path, &args.prefix, args.ignore, &config);
        for route in routes.routes.iter() {
            println!(
                "    - http://{}:{}{}\n      => {}",
                &args.hostname,
                args.port,
                route.path,
                route.handler.display()
            );
        }

        let server = serve(&args.path, routes, &args.hostname, args.port, None)
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
