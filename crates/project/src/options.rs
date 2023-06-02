// Copyright 2023 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

/// Defines the different options to configure the project.
/// Every type has their own options.
#[derive(Default)]
pub struct Options {
    /// Options for Git repositories
    pub git: Option<GitOptions>,
    /// Options for local repositories
    pub local: Option<LocalOptions>,
}

/// For now, we don't have any particular option for this type.
/// I'm keeping it as a placeholder
#[derive(Default)]
pub struct LocalOptions {}

/// The different git options you can configure.
#[derive(Default)]
pub struct GitOptions {
    /// Use a specific commit
    pub commit: Option<String>,
    /// Use a specific tag
    pub tag: Option<String>,
    /// Use a specific git branch
    pub branch: Option<String>,
    /// Change the directory to run the workers
    pub folder: Option<String>,
}
