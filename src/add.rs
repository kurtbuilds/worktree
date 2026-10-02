use crate::{setup, utils};
use anyhow::{bail, Context, Result};
use std::process::Command;

pub fn execute(name: &str) -> Result<()> {
    let root_dir = utils::get_root_dir()?;
    let repo_name = utils::get_repo_name()?;
    let repo_root = utils::get_repo_root()?;

    // Construct the worktree path: {root_dir}/{repo_name}/{branch_name}
    let worktree_path = root_dir.join(&repo_name).join(name);

    eprintln!("Creating worktree at: {}", worktree_path.display());

    // Check if the branch exists
    let branch_exists = Command::new("git")
        .args(["show-ref", "--verify", &format!("refs/heads/{}", name)])
        .output()
        .context("Failed to check if branch exists")?
        .status
        .success();

    if branch_exists {
        bail!("Branch already exists.");
    }

    // Create the worktree
    let mut cmd = Command::new("git");
    cmd.arg("worktree").arg("add");

    cmd.arg(worktree_path.to_str().unwrap());

    let output = cmd.output().context("Failed to execute git worktree add")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Failed to create worktree: {}", stderr);
    }

    eprintln!("{}", String::from_utf8_lossy(&output.stderr));

    // Copy untracked config files and run the repo's setup hook
    setup::run(&repo_root, &worktree_path)?;

    // Print the cd command for the shell to execute
    utils::print_cd_command(&worktree_path);

    Ok(())
}
