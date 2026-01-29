use anyhow::{Context, Result, bail};
use std::path::PathBuf;
use std::process::Command;
use crate::utils;

pub fn execute(name: &str) -> Result<()> {
    let worktrees = list_worktrees()?;

    // Find matching worktree by directory name or branch name
    let matching = worktrees.iter().find(|wt| {
        wt.dir_name == name || wt.branch.as_deref() == Some(name)
    });

    match matching {
        Some(wt) => {
            eprintln!("Switching to worktree: {}", wt.path.display());
            utils::print_cd_command(&wt.path);
            Ok(())
        }
        None => {
            bail!("Worktree '{}' not found. Run 'wt list' to see available worktrees.", name);
        }
    }
}

struct Worktree {
    path: PathBuf,
    dir_name: String,
    branch: Option<String>,
}

fn list_worktrees() -> Result<Vec<Worktree>> {
    let output = Command::new("git")
        .args(["worktree", "list"])
        .output()
        .context("Failed to execute git worktree list")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("Failed to list worktrees: {}", stderr);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut worktrees = Vec::new();

    for line in stdout.lines() {
        if line.is_empty() {
            continue;
        }

        // Parse lines like: /path/to/worktree  abc1234 [branch-name]
        // or: /path/to/worktree  abc1234 (bare)
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        let path = PathBuf::from(parts[0]);
        let dir_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();

        // Extract branch name from [branch] if present
        let branch = line
            .find('[')
            .and_then(|start| line.find(']').map(|end| (start, end)))
            .map(|(start, end)| line[start + 1..end].to_string());

        worktrees.push(Worktree {
            path,
            dir_name,
            branch,
        });
    }

    Ok(worktrees)
}
