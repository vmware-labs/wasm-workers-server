// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

// Define multiple utils functions that runtimes
// will use. This includes methods to create shared
// folders, provide a hashes for the functions, etc.

use anyhow::Result;
use std::{fs, path::Path};

// Common tmp folder
const TMP_FOLDER: &str = ".wws";

/// Struct to deal with temporary files as part of the
/// Runtime process. Some runtimes may require to write
/// temporary files on a folder. Note that Wasm VMs require
/// to mount a folder, not a file. To keep workers isolated
/// between others, we will prepare specific folders with the
/// source code only.
pub struct TempUtils {
    /// The folder inside the main TMP folder.
    pub folder: String,
}

impl TempUtils {
    /// Creates a new temp folder for the given language. This will
    /// allow later on to write files in that folder.
    pub fn new(lang_folder: String, source_path: &Path) -> Result<Self> {
        let hash = Self::file_hash(source_path)?;
        let folder = format!("{}/{}/{}", TMP_FOLDER, lang_folder, hash);

        fs::create_dir_all(&folder)?;

        Ok(Self { folder })
    }

    /// Write a source file into the temp language folder
    pub fn write_source(&self, source_path: &Path) -> Result<u64> {
        let ext = source_path
            .extension()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default();

        fs::copy(source_path, format!("{}/index.{}", self.folder, ext)).map_err(anyhow::Error::msg)
    }

    /// Geenrate a file hash based on the blake3 implementation. This will
    /// allow to have multiple folders that don't collide between them.
    pub fn file_hash(path: &Path) -> Result<String> {
        let content = fs::read(path)?;

        Ok(blake3::hash(&content).to_string())
    }
}
