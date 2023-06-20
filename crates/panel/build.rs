use std::{io::ErrorKind, process::Command};

// Build the client admin panel.
fn main() {
    // First check if NPM is available in the system
    match Command::new("npm").spawn() {
        Ok(_) => {
            Command::new("npm")
                .current_dir("client")
                .arg("install")
                .status()
                .expect("failed to execute process");

            Command::new("npm")
                .current_dir("client")
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
    println!("cargo:rerun-if-changed=client/src/*");
    println!("cargo:rerun-if-changed=client/public/*");
    println!("cargo:rerun-if-changed=client/index.html");
    println!("cargo:rerun-if-changed=client/vite.config.js");
}
