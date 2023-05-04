// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::ffi::OsStr;
use std::path::{Component, Path, PathBuf};
use wax::{Glob, WalkEntry};
use wws_config::Config;
use wws_runtimes_manager::check_runtime;
use wws_store::STORE_FOLDER;

pub const IGNORE_PATH_PREFIX: &str = "_";

/// Manages the files associated to a Wasm Workers Run.
/// It uses glob patterns to detect the workers and
/// provide utilities to work with public folders and
/// other related resources.
pub struct Files {
    /// Root path
    root: PathBuf,
    /// Available extensions based on the config
    extensions: Vec<String>,
    /// Check if the public folder exists
    has_public: bool,
}

impl Files {
    /// Initializes a new files instance. It will detect
    /// relevant resources for WWS like the public folder.
    pub fn new(root: &Path, config: &Config) -> Self {
        let mut extensions = vec![String::from("js"), String::from("wasm")];

        for repo in &config.repositories {
            for runtime in &repo.runtimes {
                for ext in &runtime.extensions {
                    if check_runtime(root, &repo.name, runtime) && !extensions.contains(ext) {
                        extensions.push(ext.clone());
                    }
                }
            }
        }

        Self {
            root: root.to_path_buf(),
            extensions,
            has_public: root.join(Path::new("public")).exists(),
        }
    }

    /// Walk through all the different files associated to this
    /// project using a Glob pattern
    pub fn walk(&self) -> Vec<WalkEntry> {
        let glob_pattern = format!("**/*.{{{}}}", self.extensions.join(","));
        let glob =
            Glob::new(&glob_pattern).expect("Failed to read the files in the current directory");

        glob.walk(&self.root)
            .filter_map(|el| match el {
                Ok(entry) if !self.should_ignore(entry.path()) => Some(entry),
                _ => None,
            })
            .collect()
    }

    /// Perform multiple checks to confirm if the given file should be ignored.
    /// The current checks are: file is not inside the public or .wws folder, and
    /// any component starts with _.
    fn should_ignore(&self, path: &Path) -> bool {
        path.components().any(|c| match c {
            Component::Normal(os_str) => {
                (self.has_public && os_str == OsStr::new("public"))
                    || os_str == OsStr::new(STORE_FOLDER)
                    || os_str.to_string_lossy().starts_with(IGNORE_PATH_PREFIX)
            }
            _ => false,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashSet;

    #[test]
    fn walk_default_ignore() {
        let config = Config::default();
        let files = Files::new(Path::new("tests/data/files"), &config);

        let mut expected = HashSet::new();
        expected.insert("tests/data/files/examples.js".to_string());
        expected.insert("tests/data/files/index.js".to_string());
        expected.insert("tests/data/files/public.js".to_string());
        expected.insert("tests/data/files/examples/public.js".to_string());
        expected.insert("tests/data/files/examples/index/index.js".to_string());

        let mut actual = HashSet::new();
        for entry in files.walk() {
            actual.insert(String::from(entry.path().to_string_lossy()));
        }

        assert_eq!(expected, actual);
    }

    #[cfg(not(target_os = "windows"))]
    #[test]
    fn unix_is_in_public_folder() {
        let tests = [
            ("public/index.js", true),
            ("examples/public/index.js", true),
            ("examples/public/other.js", true),
            ("public.js", false),
            ("examples/public.js", false),
            ("./examples/public.js", false),
            ("./examples/index.js", false),
        ];

        for t in tests {
            let config = Config::default();
            assert_eq!(
                Files::new(Path::new("../../tests/data"), &config).should_ignore(Path::new(t.0)),
                t.1
            )
        }
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn win_is_in_public_folder() {
        let tests = [
            ("public\\index.js", true),
            ("examples\\public\\index.js", true),
            ("examples\\public\\other.js", true),
            ("public.js", false),
            ("examples\\public.js", false),
            (".\\examples\\public.js", false),
            (".\\examples\\index.js", false),
        ];

        for t in tests {
            let config = Config::default();
            assert_eq!(
                Files::new(Path::new("..\\..\\tests\\data"), &config).should_ignore(Path::new(t.0)),
                t.1
            )
        }
    }
}
