// Copyright 2023 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::path::{Path, PathBuf};
use std::str::FromStr;
use wax::{Glob, WalkEntry};
use wws_store::STORE_FOLDER;

const IGNORE_PATH_PREFIX: &str = "_";

/// Manages the files associated to a Wasm Workers Run.
/// It uses glob patterns to detect the workers and
/// provide utilities to work with public folders and
/// other related resources.
pub struct Files<'t> {
    /// Root path
    root: PathBuf,
    /// Defines pattern for files considered as workers
    include_pattern: Glob<'t>,
    /// Defines patterns to exclude when traversing for workers
    ignore_patterns: Vec<Glob<'t>>,
}

impl<'t> Files<'t> {
    const PUBLIC_ASSETS_FOLDER: &str = "public";
    const DEFAULT_EXTENSIONS: [&str; 2] = ["js", "wasm"];

    /// Initializes a new files instance. It will detect
    /// relevant resources for WWS like the public folder.
    pub fn new(root: &Path, file_extensions: Vec<String>, ignore_patterns: Vec<String>) -> Self {
        Self {
            root: root.to_path_buf(),
            include_pattern: Self::build_include_pattern(file_extensions),
            ignore_patterns: Self::build_ignore_patterns(ignore_patterns),
        }
    }

    /// Walk through all the different files associated to this
    /// project using a Glob pattern
    pub fn walk(&self) -> Vec<WalkEntry> {
        return self
            .include_pattern
            .walk(&self.root)
            .not(self.ignore_patterns.clone())
            .expect("Failed to walk the tree when processing files in the current directory")
            .map(|e| e.unwrap())
            .collect();
    }

    fn build_include_pattern(file_extensions: Vec<String>) -> Glob<'t> {
        let mut file_extensions = file_extensions;
        for default_extension in Self::DEFAULT_EXTENSIONS {
            file_extensions.push(default_extension.to_string());
        }

        let include_pattern = format!("**/*.{{{}}}", file_extensions.join(","));
        Glob::from_str(include_pattern.as_str()).expect("Failed to parse include pattern!")
    }

    fn build_ignore_patterns(ignore_patterns: Vec<String>) -> Vec<Glob<'t>> {
        let default_ignore_patterns = vec![
            format!("**/{}/**", Self::PUBLIC_ASSETS_FOLDER),
            format!("**/{}/**", STORE_FOLDER),
            format!("**/{}*/**", IGNORE_PATH_PREFIX),
        ];

        let mut result = default_ignore_patterns;
        result.extend(ignore_patterns);
        result
            .iter()
            .map(|s| Glob::from_str(s.as_str()).expect("Failed to parse ignore pattern"))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use path_slash::PathBufExt as _;
    use std::collections::HashSet;

    #[test]
    fn walk_default() {
        let files = Files::new(Path::new("tests/data/files"), vec![], vec![]);

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
    fn walk_default_subfolder() {
        let files = Files::new(Path::new("tests/data/files/examples"), vec![], vec![]);

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

    #[test]
    fn walk_extensions() {
        let files = Files::new(
            Path::new("tests/data/files"),
            vec!["ext".to_string()],
            vec![],
        );

        let mut expected = HashSet::new();
        expected.insert(PathBuf::from_slash("tests/data/files/examples.js"));
        expected.insert(PathBuf::from_slash("tests/data/files/home.ext"));
        expected.insert(PathBuf::from_slash("tests/data/files/index.js"));
        expected.insert(PathBuf::from_slash("tests/data/files/public.js"));
        expected.insert(PathBuf::from_slash("tests/data/files/examples/home.ext"));
        expected.insert(PathBuf::from_slash("tests/data/files/examples/public.js"));
        expected.insert(PathBuf::from_slash(
            "tests/data/files/examples/index/home.ext",
        ));
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
    fn walk_ignore() {
        let files = Files::new(
            Path::new("tests/data/files"),
            vec![],
            vec!["**/examples/**".to_string()],
        );

        let mut expected = HashSet::new();
        expected.insert(PathBuf::from_slash("tests/data/files/examples.js"));
        expected.insert(PathBuf::from_slash("tests/data/files/index.js"));
        expected.insert(PathBuf::from_slash("tests/data/files/public.js"));

        let mut actual = HashSet::new();
        for entry in files.walk() {
            actual.insert(PathBuf::from_slash(String::from(
                entry.path().to_string_lossy(),
            )));
        }

        assert_eq!(expected, actual);
    }

    #[test]
    fn walk_ignore_multiple_patterns() {
        let files = Files::new(
            Path::new("tests/data/files"),
            vec!["ext".to_string(), "none".to_string()],
            vec!["**/index/**".to_string(), "*/*pub*".to_string()],
        );

        let mut expected = HashSet::new();
        expected.insert(PathBuf::from_slash("tests/data/files/examples.js"));
        expected.insert(PathBuf::from_slash("tests/data/files/home.ext"));
        expected.insert(PathBuf::from_slash("tests/data/files/index.js"));
        expected.insert(PathBuf::from_slash("tests/data/files/public.js"));
        expected.insert(PathBuf::from_slash("tests/data/files/examples/home.ext"));

        let mut actual = HashSet::new();
        for entry in files.walk() {
            actual.insert(PathBuf::from_slash(String::from(
                entry.path().to_string_lossy(),
            )));
        }

        assert_eq!(expected, actual);
    }
}
