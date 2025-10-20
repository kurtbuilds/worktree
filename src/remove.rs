use anyhow::{bail, Context, Result};
use std::process::Command;
use crate::utils;

pub fn execute(name: &str) -> Result<()> {
    let worktree_path = if name == "." {
        // Check if we're in the main worktree
        if utils::is_main_worktree()? {
            bail!("Cannot remove the main worktree. Use 'rm <worktree-name>' to remove a specific worktree.");
        }

        // Get current directory as the worktree to remove
        let current_dir = utils::get_current_dir()?;
        eprintln!("Removing current worktree: {}", current_dir.display());
        current_dir
    } else {
        // Construct the worktree path from the name
        let root_dir = utils::get_root_dir()?;
        let repo_name = utils::get_repo_name()?;
        let worktree_path = root_dir.join(&repo_name).join(name);
        eprintln!("Removing worktree: {}", worktree_path.display());
        worktree_path
    };

    // Remove the worktree
    let output = Command::new("git")
        .args(["worktree", "remove", worktree_path.to_str().unwrap()])
        .output()
        .context("Failed to execute git worktree remove")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("Failed to remove worktree: {}", stderr);
    }

    eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    eprintln!("Worktree removed successfully");

    // If we removed the current worktree, cd to the main worktree
    if name == "." {
        let main_worktree_path = utils::get_main_worktree_path()?;
        eprintln!("Changing to main worktree: {}", main_worktree_path.display());
        utils::print_cd_command(&main_worktree_path);
    }

    Ok(())
}
