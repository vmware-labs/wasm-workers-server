// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

// Define a common place to store the data associated to the
// user project. wws requires to install runtimes, create
// temporary files with workers metadata, etc.
//
// This struct provide the basics to interact with that folder
// in both Unix and Windows systems.
use std::{
    fs,
    path::{Path, PathBuf},
};

pub mod errors;
use errors::{Result, StoreError};

/// This is a temporary folder in which runtimes can prepare
/// and store certain data. For example, the JS runtime have
/// to mount a folder with the source code. To avoid moun†ing
/// a folder that may include multiple files, it stores in
/// .wws/js/XXX/index.js the worker file.
pub const STORE_FOLDER: &str = ".wws";

/// Struct to initialize, create and interact with files inside
/// the store. All paths are considered &[&str] to ensure we
/// generate the paths properly on Windows and Unix.
pub struct Store {
    /// The base folder for this instance. Every time we initialize
    /// a store, it will ensure all the files are scoped to the given
    /// folder.
    pub folder: PathBuf,
}

impl Store {
    /// Instance a new store. If you want to create the root folder, check [#create].
    /// The root path is used to scope the files inside the STORE_FOLDER folder. Note
    /// other methods may fail if you don't create the folder.
    pub fn new(project_root: &Path, folder: &[&str]) -> Self {
        let folder = Self::build_root_path(project_root, folder);

        Self { folder }
    }

    /// Instance a new store and creates the root folder. The root path is
    /// used to scope the files inside the STORE_FOLDER folder.
    pub fn create(project_root: &Path, folder: &[&str]) -> Result<Self> {
        let folder = Self::build_root_path(project_root, folder);

        // Try to create the directory
        fs::create_dir_all(&folder).map_err(|err| StoreError::CannotCreateDirectory {
            path: folder.clone(),
            error: err,
        })?;

        Ok(Self { folder })
    }

    /// Create the root folder for the current context
    pub fn create_root_folder(&self) -> Result<()> {
        fs::create_dir_all(&self.folder).map_err(|err| StoreError::CannotCreateDirectory {
            path: self.folder.clone(),
            error: err,
        })
    }

    /// Delete the root folder from the current context
    pub fn delete_root_folder(&self) -> Result<()> {
        if self.folder.exists() {
            fs::remove_dir_all(&self.folder).map_err(|err| StoreError::CannotDeleteDirectory {
                path: self.folder.clone(),
                error: err,
            })
        } else {
            Ok(())
        }
    }

    /// Check if the given file path exists in the current context
    pub fn check_file(&self, path: &[&str]) -> bool {
        self.build_folder_path(path).exists()
    }

    /// Write a specific file inside the configured root folder
    pub fn write(&self, path: &[&str], contents: &[u8]) -> Result<()> {
        let file_path = self.build_folder_path(path);
        fs::write(&file_path, contents).map_err(|err| StoreError::CannotWriteFile {
            path: file_path,
            error: err,
        })
    }

    /// Read the file content in the given store
    pub fn read(&self, path: &[&str]) -> Result<Vec<u8>> {
        let file_path = self.build_folder_path(path);
        fs::read(&file_path).map_err(|err| StoreError::CannotReadFile {
            path: file_path,
            error: err,
        })
    }

    /// Copy file inside the configured root folder
    pub fn copy(&self, source: &Path, dest: &[&str]) -> Result<()> {
        let file_path = self.build_folder_path(dest);
        fs::copy(source, &file_path).map_err(|err| StoreError::CannotWriteFile {
            path: file_path,
            error: err,
        })?;
        Ok(())
    }

    /// This method builds a path in the context of the instance folder
    pub fn build_folder_path(&self, source: &[&str]) -> PathBuf {
        source
            .iter()
            .fold(self.folder.clone(), |acc, comp| acc.join(comp))
    }

    /// Generate a file hash based on the blake3 implementation
    pub fn file_hash(path: &Path) -> Result<String> {
        let content = fs::read(path).map_err(|err| StoreError::CannotReadFile {
            path: path.to_path_buf(),
            error: err,
        })?;

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
