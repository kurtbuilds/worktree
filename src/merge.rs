use anyhow::{bail, Context, Result};
use std::process::Command;
use crate::utils;

/// Get the branch name checked out in a worktree
fn get_worktree_branch(worktree_path: &std::path::Path) -> Result<String> {
    let output = Command::new("git")
        .current_dir(worktree_path)
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .context("Failed to get branch name")?;

    if !output.status.success() {
        bail!("Failed to determine branch name for worktree");
    }

    Ok(String::from_utf8(output.stdout)
        .context("Invalid UTF-8 in branch name")?
        .trim()
        .to_string())
}

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

    // Get the branch name before we do anything (need it for cleanup later)
    let branch_name = get_worktree_branch(&worktree_path)?;

    eprintln!("Merging and cleaning up worktree: {}", worktree_path.display());
    eprintln!("Branch: {}", branch_name);

    // Build the strategy flag
    let strategy_flag = match strategy {
        "squash" => "--squash",
        "merge" => "--merge",
        "rebase" => "--rebase",
        _ => bail!("Invalid merge strategy: {}. Use 'squash', 'merge', or 'rebase'", strategy),
    };

    // Step 1: Merge the PR (without --delete-branch to avoid the worktree conflict)
    // We handle branch deletion ourselves after removing the worktree
    eprintln!("Running: gh pr merge {}", strategy_flag);
    let output = Command::new("gh")
        .current_dir(&worktree_path)
        .args(["pr", "merge", strategy_flag])
        .output()
        .context("Failed to execute gh pr merge. Is the GitHub CLI installed?")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        if !stdout.is_empty() {
            eprintln!("{}", stdout);
        }
        if stderr.contains("no pull requests found") {
            eprintln!("No PR found for branch \"{}\", skipping merge. Cleaning up worktree and branch.", branch_name);
        } else {
            bail!("Failed to merge PR: {}", stderr);
        }
    } else {
        // Print gh output
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        if !stdout.is_empty() {
            eprintln!("{}", stdout);
        }
        if !stderr.is_empty() {
            eprintln!("{}", stderr);
        }
    }

    // Step 2: Remove the worktree (this unlocks the branch for deletion)
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

    // Step 3: Delete the local branch (now possible since worktree is gone)
    eprintln!("Deleting local branch: {}", branch_name);
    let output = Command::new("git")
        .current_dir(&main_worktree_path)
        .args(["branch", "-D", &branch_name])
        .output()
        .context("Failed to execute git branch -D")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Warning: Failed to delete local branch: {}", stderr);
        // Don't fail here - the branch might already be deleted or not exist
    } else {
        eprintln!("Local branch deleted");
    }

    // Step 4: Delete the remote branch (may already be deleted by GitHub's auto-delete setting)
    eprintln!("Deleting remote branch: origin/{}", branch_name);
    let output = Command::new("git")
        .current_dir(&main_worktree_path)
        .args(["push", "origin", "--delete", &branch_name])
        .output()
        .context("Failed to execute git push --delete")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        // This is expected if GitHub already auto-deleted the branch
        if stderr.contains("remote ref does not exist") {
            eprintln!("Remote branch already deleted (likely by GitHub auto-delete)");
        } else {
            eprintln!("Warning: Failed to delete remote branch: {}", stderr);
        }
    } else {
        eprintln!("Remote branch deleted");
    }

    // If we were in the worktree being removed, cd to main
    if need_cd {
        eprintln!("Changing to main worktree: {}", main_worktree_path.display());
        utils::print_cd_command(&main_worktree_path);
    }

    eprintln!("Merge complete!");

    Ok(())
}
