use git2::{build::CheckoutBuilder, build::RepoBuilder, Repository as GitRepository,
           RepositoryInitOptions};
use quicli::prelude::*;
use remove_dir_all::remove_dir_all;
use std::path::PathBuf;
use Cli;

pub fn create(project_dir: &PathBuf, args: Cli) -> Result<GitRepository> {
    Ok(RepoBuilder::new()
        .bare(false)
        .with_checkout(CheckoutBuilder::new())
        .clone(&args.git, &project_dir)?)
}

pub fn remove_history(project_dir: &PathBuf) -> Result<()> {
    Ok(remove_dir_all(project_dir.join(".git")).context("Error cleaning up cloned template")?)
}

pub fn init(project_dir: &PathBuf) -> Result<GitRepository> {
    Ok(
        GitRepository::init_opts(project_dir, RepositoryInitOptions::new().bare(false))
            .context("Couldn't init new repository")?,
    )
}
