use anyhow::{bail, Context, Result};
use std::process::Command;
use crate::utils;

pub fn execute(name: Option<&str>, strategy: &str) -> Result<()> {
    let is_main = utils::is_main_worktree()?;
    let main_worktree_path = utils::get_main_worktree_path()?;

    // Determine the worktree path to merge
    let (worktree_path, need_cd) = match (is_main, name) {
        // On main with a name: merge that worktree
        (true, Some(n)) => {
            let root_dir = utils::get_root_dir()?;
            let repo_name = utils::get_repo_name()?;
            let path = root_dir.join(&repo_name).join(n);
            (path, false)
        }
        // On main without a name: error
        (true, None) => {
            bail!("Must specify a worktree name when running from the main worktree.\nUsage: wt merge <worktree-name>");
        }
        // In a worktree without a name: merge current
        (false, None) => {
            let path = utils::get_current_dir()?;
            (path, true)
        }
        // In a worktree with a name: merge that worktree (not current)
        (false, Some(n)) => {
            let root_dir = utils::get_root_dir()?;
            let repo_name = utils::get_repo_name()?;
            let path = root_dir.join(&repo_name).join(n);
            (path, false)
        }
    };

    // Verify the worktree exists
    if !worktree_path.exists() {
        bail!("Worktree not found: {}", worktree_path.display());
    }

    eprintln!("Merging PR and cleaning up worktree: {}", worktree_path.display());

    // Build the strategy flag
    let strategy_flag = match strategy {
        "squash" => "--squash",
        "merge" => "--merge",
        "rebase" => "--rebase",
        _ => bail!("Invalid merge strategy: {}. Use 'squash', 'merge', or 'rebase'", strategy),
    };

    // Merge the PR (run from the worktree directory so gh knows which PR)
    eprintln!("Running: gh pr merge {} --delete-branch", strategy_flag);
    let output = Command::new("gh")
        .current_dir(&worktree_path)
        .args(["pr", "merge", strategy_flag, "--delete-branch"])
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
    eprintln!("Removing worktree: {}", worktree_path.display());
    let output = Command::new("git")
        .current_dir(&main_worktree_path)
        .args(["worktree", "remove", worktree_path.to_str().unwrap()])
        .output()
        .context("Failed to execute git worktree remove")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("Failed to remove worktree: {}", stderr);
    }

    eprintln!("Worktree removed successfully");

    // If we were in the worktree being removed, cd to main
    if need_cd {
        eprintln!("Changing to main worktree: {}", main_worktree_path.display());
        utils::print_cd_command(&main_worktree_path);
    }

    Ok(())
}
