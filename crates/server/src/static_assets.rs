// Copyright 2023 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::{
    ffi::OsStr,
    io::Error as IoError,
    path::{Path, PathBuf},
};

/// Folder that contains the static assets in a wws project
pub const STATIC_ASSETS_FOLDER: &str = "public";

/// Load and stores the information of static assets
/// in Wasm Workers Server. It enables to manually set
/// the list of routes in actix.
#[derive(Default)]
pub struct StaticAssets {
    /// Static assets folder
    folder: PathBuf,
    /// The initial prefix to mount the static assets
    prefix: String,
    /// List of local paths to set in actix
    pub paths: Vec<String>,
}

impl StaticAssets {
    /// Creates a new instance by looking at the public
    /// folder if exists.
    pub fn new(root_path: &Path, prefix: &str) -> Self {
        Self {
            folder: root_path.join(STATIC_ASSETS_FOLDER),
            prefix: prefix.to_string(),
            paths: Vec::new(),
        }
    }

    /// Load the assets in the public folder.
    pub fn load(&mut self) -> Result<(), IoError> {
        if self.folder.exists() {
            // Set the provided prefix
            let prefix = self.prefix.clone();

            self.load_folder(&self.folder.clone(), &prefix)?;
        }

        Ok(())
    }

    /// Load the assets from a specific folder
    fn load_folder(&mut self, folder: &Path, prefix: &str) -> Result<(), IoError> {
        let paths = folder.read_dir()?;

        for path in paths {
            let path = path?.path();

            if path.is_dir() {
                let folder_path = path
                    .file_stem()
                    .expect("Error reading the file stem from a static file")
                    .to_string_lossy();

                let new_prefix = format!("{}/{}", prefix, folder_path);
                // Recursive
                self.load_folder(&path, &new_prefix)?;
            } else {
                // Save the static file
                match path.extension() {
                    Some(ext) if ext == OsStr::new("html") => {
                        let stem = path
                            .file_stem()
                            .expect("Error reading the file name from a static file")
                            .to_string_lossy();

                        // Add the full file path
                        self.paths.push(format!("{prefix}/{stem}.html"));

                        if stem == "index" {
                            // For index files, mount it on the prefix (folder)
                            self.paths.push(format!("{prefix}/"));
                        } else {
                            // Mount it without the .html (pretty routes)
                            self.paths.push(format!("{prefix}/{stem}"));
                        }
                    }
                    _ => {
                        let name = path
                            .file_name()
                            .expect("Error reading the file name from a static file")
                            .to_string_lossy();

                        self.paths.push(format!("{prefix}/{name}"));
                    }
                }
            }
        }

        Ok(())
    }
}
