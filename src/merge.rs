use anyhow::{bail, Context, Result};
use std::process::Command;
use crate::utils;

pub fn execute(strategy: Option<&str>) -> Result<()> {
    // Check if we're in the main worktree
    if utils::is_main_worktree()? {
        bail!("Cannot merge from the main worktree. Run this command from within a feature worktree.");
    }

    let current_dir = utils::get_current_dir()?;
    let main_worktree_path = utils::get_main_worktree_path()?;

    eprintln!("Merging PR and cleaning up worktree: {}", current_dir.display());

    // Build the gh pr merge command args
    let mut args = vec!["pr", "merge", "--delete-branch"];

    // Add strategy flag only if explicitly provided
    if let Some(s) = strategy {
        let strategy_flag = match s {
            "squash" => "--squash",
            "merge" => "--merge",
            "rebase" => "--rebase",
            _ => bail!("Invalid merge strategy: {}. Use 'squash', 'merge', or 'rebase'", s),
        };
        args.push(strategy_flag);
    }

    // Merge the PR
    eprintln!("Running: gh {}", args.join(" "));
    let output = Command::new("gh")
        .args(&args)
        .output()
        .context("Failed to execute gh pr merge. Is the GitHub CLI installed?")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        if !stdout.is_empty() {
            eprintln!("{}", stdout);
        }
        bail!("Failed to merge PR: {}", stderr);
    }

    // Print gh output
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    if !stdout.is_empty() {
        eprintln!("{}", stdout);
    }
    if !stderr.is_empty() {
        eprintln!("{}", stderr);
    }

    // Remove the worktree
    eprintln!("Removing worktree: {}", current_dir.display());
    let output = Command::new("git")
        .current_dir(&main_worktree_path)
        .args(["worktree", "remove", current_dir.to_str().unwrap()])
        .output()
        .context("Failed to execute git worktree remove")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("Failed to remove worktree: {}", stderr);
    }

    eprintln!("Worktree removed successfully");

    // Change to main worktree
    eprintln!("Changing to main worktree: {}", main_worktree_path.display());
    utils::print_cd_command(&main_worktree_path);

    Ok(())
}
