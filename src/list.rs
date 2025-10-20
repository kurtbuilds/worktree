use anyhow::{Context, Result};
use std::process::Command;

pub fn execute() -> Result<()> {
    let output = Command::new("git")
        .args(["worktree", "list"])
        .output()
        .context("Failed to execute git worktree list")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Failed to list worktrees: {}", stderr);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    print!("{}", stdout);

    Ok(())
}
