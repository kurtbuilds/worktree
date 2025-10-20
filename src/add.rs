use anyhow::{Context, Result};
use std::process::Command;
use crate::utils;

pub fn execute(name: &str) -> Result<()> {
    let root_dir = utils::get_root_dir()?;
    let repo_name = utils::get_repo_name()?;
    let current_dir = utils::get_current_dir()?;

    // Construct the worktree path: {root_dir}/{repo_name}/{branch_name}
    let worktree_path = root_dir.join(&repo_name).join(name);

    eprintln!("Creating worktree at: {}", worktree_path.display());

    // Create the worktree
    let output = Command::new("git")
        .args(["worktree", "add", worktree_path.to_str().unwrap(), name])
        .output()
        .context("Failed to execute git worktree add")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Failed to create worktree: {}", stderr);
    }

    eprintln!("{}", String::from_utf8_lossy(&output.stderr));

    // Copy .env file from current directory to the new worktree
    utils::copy_env_file(&current_dir, &worktree_path)?;

    // Print the cd command for the shell to execute
    utils::print_cd_command(&worktree_path);

    Ok(())
}
