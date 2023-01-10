// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::ffi::OsStr;
use std::path::{Component, Path, PathBuf};
use wax::{Glob, WalkEntry};

/// Manages the files associated to a Wasm Workers Run.
/// It uses glob patterns to detect the workers and
/// provide utilities to work with public folders and
/// other related resources.
pub struct Files {
    root: PathBuf,
    has_public: bool,
}

impl Files {
    /// Initializes a new files instance. It will detect
    /// relevant resources for WWS like the public folder.
    pub fn new(root: &Path) -> Self {
        Self {
            root: root.to_path_buf(),
            has_public: root.join(Path::new("public")).exists(),
        }
    }

    /// Walk through all the different files associated to this
    /// project using a Glob pattern
    pub fn walk(&self) -> Vec<WalkEntry> {
        let glob =
            Glob::new("**/*.{wasm,js}").expect("Failed to read the files in the current directory");

        glob.walk(&self.root)
            .filter_map(|el| match el {
                Ok(entry) if !self.is_in_public_folder(entry.path()) => Some(entry),
                _ => None,
            })
            .collect()
    }

    /// Checks if the given filepath is inside the "public" folder.
    /// It will return an early false if the project doesn't have
    /// a public folder.
    pub fn is_in_public_folder(&self, path: &Path) -> bool {
        if !self.has_public {
            return false;
        }

        path.components().any(|c| match c {
            Component::Normal(os_str) => os_str == OsStr::new("public"),
            _ => false,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
            assert_eq!(
                Files::new(Path::new("./tests/data")).is_in_public_folder(Path::new(t.0)),
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
            assert_eq!(
                Files::new(Path::new(".\\tests\\data")).is_in_public_folder(Path::new(t.0)),
                t.1
            )
        }
    }
}
