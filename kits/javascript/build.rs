// Copyright 2023 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::{io::ErrorKind, process::Command};

// Build the client admin panel.
fn main() {
    // First check if NPM is available in the system
    match Command::new("npm").spawn() {
        Ok(_) => {
            Command::new("npm")
                .current_dir("shims")
                .arg("install")
                .status()
                .expect("failed to execute process");

            Command::new("npm")
                .current_dir("shims")
                .args(["run", "build"])
                .status()
                .expect("failed to execute process");
        }
        Err(e) => {
            if let ErrorKind::NotFound = e.kind() {
                eprintln!("`npm` was not found in your system. Please, install NodeJS / NPM to build the admin panel.");
                eprintln!("See: https://nodejs.dev/en/download/");
            } else {
                eprintln!(
                    "There was an error when building the admin panel with NodeJS / NPM: {e}"
                );
            }
        }
    }

    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=shims/*.js");
    println!("cargo:rerun-if-changed=shims/package.json");
    println!("cargo:rerun-if-changed=shims/types/*.js");
}
