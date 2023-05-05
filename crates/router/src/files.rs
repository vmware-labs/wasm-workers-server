// Copyright 2023 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::path::{Path, PathBuf};
use std::str::FromStr;
use wax::{Glob, WalkEntry};
use wws_config::Config;
use wws_runtimes_manager::check_runtime;
use wws_store::STORE_FOLDER;

const IGNORE_PATH_PREFIX: &str = "_";

/// Manages the files associated to a Wasm Workers Run.
/// It uses glob patterns to detect the workers and
/// provide utilities to work with public folders and
/// other related resources.
pub struct Files {
    /// Root path
    root: PathBuf,
    /// Defines pattern for files considered as workers
    include_pattern: String,
    /// Defines patterns to exclude when traversing for workers
    ignore_patterns: Vec<String>,
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

        let include_pattern: String = format!("**/*.{{{}}}", extensions.join(","));

        let default_ignore_patterns = vec![
            "**/public/**".to_string(),
            format!("**/{}/**", STORE_FOLDER),
            format!("**/{}*/**", IGNORE_PATH_PREFIX),
        ];

        Self {
            root: root.to_path_buf(),
            include_pattern,
            ignore_patterns: default_ignore_patterns,
        }
    }

    /// Walk through all the different files associated to this
    /// project using a Glob pattern
    pub fn walk(&self) -> Vec<WalkEntry> {
        let include_pattern = Glob::from_str(self.include_pattern.as_str()).expect(
            "Failed to parse include pattern when processing files in the current directory",
        );

        return include_pattern
            .walk(&self.root)
            .not(self.ignore_patterns.iter().map(|s| s.as_str()))
            .expect("Failed to parse ignore patterns pattern when processing files in the current directory")
            .map(|e| e.unwrap()).collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use path_slash::PathBufExt as _;
    use std::collections::HashSet;

    #[test]
    fn walk_default_ignore() {
        let config = Config::default();
        let files = Files::new(Path::new("tests/data/files"), &config);

        let mut expected = HashSet::new();
        expected.insert(PathBuf::from_slash("tests/data/files/examples.js"));
        expected.insert(PathBuf::from_slash("tests/data/files/index.js"));
        expected.insert(PathBuf::from_slash("tests/data/files/public.js"));
        expected.insert(PathBuf::from_slash("tests/data/files/examples/public.js"));
        expected.insert(PathBuf::from_slash(
            "tests/data/files/examples/index/index.js",
        ));

        let mut actual = HashSet::new();
        for entry in files.walk() {
            actual.insert(PathBuf::from_slash(String::from(
                entry.path().to_string_lossy(),
            )));
        }

        assert_eq!(expected, actual);
    }

    #[test]
    fn walk_default_ignore_subfolder() {
        let config = Config::default();
        let files = Files::new(Path::new("tests/data/files/examples"), &config);

        let mut expected = HashSet::new();
        expected.insert(PathBuf::from_slash("tests/data/files/examples/public.js"));
        expected.insert(PathBuf::from_slash(
            "tests/data/files/examples/index/index.js",
        ));

        let mut actual = HashSet::new();
        for entry in files.walk() {
            actual.insert(PathBuf::from_slash(String::from(
                entry.path().to_string_lossy(),
            )));
        }

        assert_eq!(expected, actual);
    }
}
