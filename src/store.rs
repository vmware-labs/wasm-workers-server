// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

// Define a common place to store the data associated to the
// user project. wws requires to install runtimes, create
// temporary files with workers metadata, etc.
//
// This struct provide the basics to interact with that folder
// in both Unix and Windows systems.

use anyhow::Result;
use std::{
    fs,
    path::{Path, PathBuf},
};

/// This is a temporary folder in which runtimes can prepare
/// and store certain data. For example, the JS runtime have
/// to mount a folder with the source code. To avoid moun†ing
/// a folder that may include multiple files, it stores in
/// .wws/js/XXX/index.js the worker file.
const STORE_FOLDER: &str = ".wws";

/// Struct to initialize, create and interact with files inside
/// the store. All paths are considered &[&str] to ensure we
/// generate the paths properly on Windows and Unix.
pub struct Store {
    /// The base folder for this instance. Every time we initialize
    /// a store, it will ensure all the files are scoped to the given
    /// folder.
    pub folder: PathBuf,
}

// TODO: Remove it when implementing the full logic
#[allow(dead_code)]
impl Store {
    /// Instance a new store and creates the root folder. The root path is
    /// used to scope the files inside the STORE_FOLDER folder.
    pub fn new(project_root: &Path, folder: &[&str]) -> Result<Self> {
        let folder = Self::build_root_path(project_root, folder);

        // Try to create the directory
        fs::create_dir_all(&folder)?;

        Ok(Self { folder })
    }

    /// Write a specific file inside the configured root folder
    pub fn write(&self, path: &[&str], contents: &[u8]) -> Result<()> {
        let file_path = self.build_folder_path(path);
        fs::write(file_path, contents)?;

        Ok(())
    }

    /// Copy file inside the configured root folder
    pub fn copy(&self, source: &Path, dest: &[&str]) -> Result<()> {
        let file_path = self.build_folder_path(dest);
        fs::copy(source, file_path)?;

        Ok(())
    }

    /// This method builds a path in the context of the instance folder
    fn build_folder_path(&self, source: &[&str]) -> PathBuf {
        source
            .iter()
            .fold(self.folder.clone(), |acc, comp| acc.join(comp))
    }

    /// Generate a file hash based on the blake3 implementation
    pub fn file_hash(path: &Path) -> Result<String> {
        let content = fs::read(path)?;

        Ok(blake3::hash(&content).to_string())
    }

    /// Build a valid path for multiple platforms. It takes advantages of the
    /// Path methods
    fn build_root_path(root: &Path, source: &[&str]) -> PathBuf {
        source
            .iter()
            .fold(root.join(STORE_FOLDER), |acc, comp| acc.join(comp))
    }
}
