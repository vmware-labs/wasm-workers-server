// Copyright 2023 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::options::{GitReference, Options};
use anyhow::{anyhow, bail, Result};
use git2::{Oid, Repository};
use sha256::digest as sha256_digest;
use std::{
    env::temp_dir,
    fs::remove_dir_all,
    path::{Path, PathBuf},
};

// Default remote for git repos
static DEFAULT_REMOTE: &str = "origin";

/// Prepare a project based on a git repository. This method
/// clones the repo locally and returns the path in which it's located.
pub fn prepare_git_project(location: &Path, options: Option<Options>) -> Result<PathBuf> {
    let project_url = location
        .to_str()
        .ok_or(anyhow!("The project URL cannot be retrieved"))?;
    // By default, we use temporary dirs
    let mut dir = temp_dir().join(sha256_digest(project_url));

    if dir.exists() {
        // Clean up a previous download
        remove_dir_all(&dir)?;
    }

    let repo = match Repository::clone(project_url, &dir) {
        Ok(repo) => repo,
        Err(e) => bail!("There was an error cloning the repository: {e}"),
    };

    if let Some(options) = options {
        if let Some(git) = options.git {
            if let Some(git_ref) = git.git_ref {
                match git_ref {
                    GitReference::Commit(commit) => {
                        let oid = Oid::from_str(&commit)?;
                        let commit = repo.find_commit(oid)?;
                        repo.checkout_tree(commit.as_object(), None)?;
                    }
                    GitReference::Tag(tag) => {
                        let mut remote = repo.find_remote(DEFAULT_REMOTE)?;
                        let tag_remote = format!("refs/tags/{tag}:refs/tags/{tag}");
                        remote.fetch(&[&tag_remote], None, None)?;

                        let oid = Oid::from_str(&tag)?;
                        let tag = repo.find_tag(oid)?;
                        repo.checkout_tree(tag.as_object(), None)?;
                    }
                    GitReference::Branch(branch) => {
                        let mut remote = repo.find_remote(DEFAULT_REMOTE)?;
                        let head_remote = format!("refs/heads/{branch}:refs/heads/{branch}");
                        remote.fetch(&[&head_remote], None, None)?;

                        let branch = repo.find_branch(&branch, git2::BranchType::Local)?;
                        let reference = branch.into_reference();
                        repo.checkout_tree(&reference.peel(git2::ObjectType::Tree)?, None)?;
                    }
                }
            }

            if let Some(folder) = git.folder {
                dir = dir.join(folder);
            }
        }
    }

    Ok(dir)
}
