// Copyright 2023 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::Args;
use wws_project::options::{GitOptions, GitReference, Options};

/// Create the project options from the CLI arguments
pub fn build_project_options(args: &Args) -> Options {
    Options {
        local: None,
        git: Some(build_git_options(args)),
    }
}

/// Create the Git options from the CLI arguments
pub fn build_git_options(args: &Args) -> GitOptions {
    let mut git_opts = GitOptions::default();

    // This conditional is prioritized: commit > tag > branch
    if let Some(commit) = args.git_commit.as_ref() {
        git_opts.git_ref = Some(GitReference::Commit(commit.clone()));
    } else if let Some(tag) = args.git_tag.as_ref() {
        git_opts.git_ref = Some(GitReference::Tag(tag.clone()));
    } else if let Some(branch) = args.git_branch.as_ref() {
        git_opts.git_ref = Some(GitReference::Branch(branch.clone()));
    }

    if let Some(folder) = args.git_folder.as_ref() {
        git_opts.folder = Some(folder.clone());
    }

    git_opts
}
