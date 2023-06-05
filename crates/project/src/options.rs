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

/// Defines a different reference when cloning the repository
pub enum GitReference {
    /// Use a specific commit
    Commit(String),
    /// Use a specific tag
    Tag(String),
    /// Use a specific git branch
    Branch(String),
}

/// The different git options you can configure.
#[derive(Default)]
pub struct GitOptions {
    pub git_ref: Option<GitReference>,
    /// Change the directory to run the workers
    pub folder: Option<String>,
}
