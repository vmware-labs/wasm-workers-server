// Copyright 2023 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::errors::{self, Result};
use crate::options::{GitReference, Options};
use git2::{build::CheckoutBuilder, FetchOptions, Oid, Repository};
use sha256::digest as sha256_digest;
use std::{
    env::temp_dir,
    path::{Path, PathBuf},
};

// Default remote for git repos
static DEFAULT_REMOTE: &str = "origin";

/// Prepare a project based on a git repository. This method
/// clones the repo locally and returns the path in which it's located.
pub fn prepare_git_project(location: &Path, options: Options) -> Result<PathBuf> {
    let project_url = location
        .to_str()
        .ok_or(errors::FetchError::InvalidRepository)?;
    let (folder, git_ref) = parse_options(options);
    // By default, we use temporary dirs
    let mut dir = temp_dir().join(sha256_digest(project_url));

    let repo = if dir.exists() {
        // Reuse the same repository.
        Repository::open(&dir).map_err(|_| errors::FetchError::InvalidReusedRepository)?
    } else {
        // clone it
        Repository::clone(project_url, &dir).map_err(|_| errors::FetchError::InvalidRepository)?
    };

    if let Some(git_ref) = git_ref.as_ref() {
        match git_ref {
            GitReference::Commit(commit) => {
                pull_default_branch(&repo)?;

                let oid = Oid::from_str(commit)?;
                repo.set_head_detached(oid)?;
                repo.checkout_head(Some(&mut default_checkout()))?;
            }
            GitReference::Tag(tag) => {
                let mut remote = repo.find_remote(DEFAULT_REMOTE)?;
                let tag_remote = format!("refs/tags/{tag}:refs/tags/{tag}");
                remote.fetch(&[&tag_remote], None, None)?;

                repo.set_head(&format!("refs/tags/{tag}"))?;
                repo.checkout_head(Some(&mut default_checkout()))?;
            }
            GitReference::Branch(branch) => {
                let mut remote = repo.find_remote(DEFAULT_REMOTE)?;
                let head_remote = format!("refs/heads/{branch}:refs/heads/{branch}");
                remote.fetch(&[&head_remote], None, None)?;

                repo.set_head(&format!("refs/heads/{branch}"))?;
                repo.checkout_head(Some(&mut default_checkout()))?;
            }
        }
    } else {
        pull_default_branch(&repo)?;
    }

    if let Some(folder) = folder {
        dir = dir.join(folder);
    }

    Ok(dir)
}

/// Generates a default configuration to checkout the git repository
fn default_checkout<'cb>() -> CheckoutBuilder<'cb> {
    let mut checkout_builder = CheckoutBuilder::default();

    checkout_builder
        .allow_conflicts(true)
        .conflict_style_merge(true)
        .force();

    checkout_builder
}

/// Parse the different configuration parameters from the given Options
fn parse_options(options: Options) -> (Option<String>, Option<GitReference>) {
    if let Some(git) = options.git {
        (git.folder, git.git_ref)
    } else {
        (None, None)
    }
}

/// Pull the changes from the default branch
fn pull_default_branch(repo: &Repository) -> Result<()> {
    let branch = detect_main_branch(repo)?;
    pull_repository(repo, branch)
}

/// Detech the main branch of this repository
fn detect_main_branch(repo: &Repository) -> Result<&str> {
    // For now, we only distinguish between the two most common branch names.
    // Ask the user to set the branch in any other case.
    if repo.find_branch("main", git2::BranchType::Local).is_ok() {
        Ok("main")
    } else if repo.find_branch("master", git2::BranchType::Local).is_ok() {
        Ok("master")
    } else {
        Err(errors::FetchError::DefaultBranchMissing)
    }
}

/// Fetch the latest references from a repository and pull all mising
/// objects. This method ensures an existing repo is not stale
fn pull_repository(repo: &Repository, branch: &str) -> Result<()> {
    let mut remote = repo.find_remote(DEFAULT_REMOTE)?;
    let mut fo = FetchOptions::new();

    remote.fetch(&[branch], Some(&mut fo), None)?;

    let fetch_head = repo.find_reference("FETCH_HEAD")?;
    let fetch_commit = fetch_head.peel_to_commit()?;

    // Follow a fast-forward merge by default. These repositories shouldn't be
    // modified. In any other case, it will fail.
    let refname = format!("refs/heads/{}", branch);

    match repo.find_reference(&refname) {
        Ok(mut reference) => {
            // Get the reference name
            let name = match reference.name() {
                Some(s) => s.to_string(),
                None => String::from_utf8_lossy(reference.name_bytes()).to_string(),
            };

            // Perform the pull
            reference.set_target(fetch_commit.id(), "")?;
            repo.set_head(&name)?;
            repo.checkout_head(Some(&mut default_checkout()))?;
        }
        Err(_) => {
            // The branch doesn't exist
            repo.reference(&refname, fetch_commit.id(), true, "")?;
            repo.set_head(&refname)?;
            repo.checkout_head(Some(&mut default_checkout()))?;
        }
    };

    Ok(())
}
