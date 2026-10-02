use anyhow::{Context, Result, bail};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::env;

/// Get the root directory from WORKTREE_ROOT_DIR env var
pub fn get_root_dir() -> Result<PathBuf> {
    let root = env::var("WORKTREE_ROOT_DIR")
        .context("WORKTREE_ROOT_DIR environment variable not set")?;
    Ok(PathBuf::from(root))
}

/// Get the top-level directory of the current worktree
pub fn get_repo_root() -> Result<PathBuf> {
    let output = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .context("Failed to execute git command")?;

    if !output.status.success() {
        bail!("Not in a git repository");
    }

    let repo_path = String::from_utf8(output.stdout)
        .context("Invalid UTF-8 in git output")?
        .trim()
        .to_string();

    Ok(PathBuf::from(repo_path))
}

/// Get the current git repository name
pub fn get_repo_name() -> Result<String> {
    let repo_path = get_repo_root()?;

    let repo_name = repo_path
        .file_name()
        .context("Could not determine repository name")?
        .to_str()
        .context("Invalid repository name")?
        .to_string();

    Ok(repo_name)
}

/// Check if we're in the main worktree
pub fn is_main_worktree() -> Result<bool> {
    let output = Command::new("git")
        .args(["rev-parse", "--git-dir"])
        .output()
        .context("Failed to execute git command")?;

    if !output.status.success() {
        bail!("Not in a git repository");
    }

    let git_dir = String::from_utf8(output.stdout)
        .context("Invalid UTF-8 in git output")?
        .trim()
        .to_string();

    // In the main worktree, git-dir is ".git"
    // In a linked worktree, it's a full path to .git/worktrees/<name>
    Ok(git_dir == ".git")
}

/// Get the current directory
pub fn get_current_dir() -> Result<PathBuf> {
    env::current_dir().context("Failed to get current directory")
}

/// Print a shell command that changes directory
pub fn print_cd_command(path: &Path) {
    println!("cd \"{}\"", path.display());
}

/// Get the path to the main worktree
pub fn get_main_worktree_path() -> Result<PathBuf> {
    let output = Command::new("git")
        .args(["rev-parse", "--git-common-dir"])
        .output()
        .context("Failed to execute git command")?;

    if !output.status.success() {
        bail!("Not in a git repository");
    }

    let git_common_dir = String::from_utf8(output.stdout)
        .context("Invalid UTF-8 in git output")?
        .trim()
        .to_string();

    let main_worktree = Path::new(&git_common_dir)
        .parent()
        .context("Could not determine main worktree path")?
        .to_path_buf();

    Ok(main_worktree)
}
