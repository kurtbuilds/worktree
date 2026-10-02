use anyhow::{bail, Context, Result};
use std::fs;
use std::io;
use std::path::Path;
use std::process::{Command, Stdio};

/// Per-repo config directory, looked up in the worktree you run `wt add` from
const CONFIG_DIR: &str = ".worktree-cli";

/// Files copied if they exist, unless the repo has a setup.sh
const DEFAULT_COPY: &[&str] = &[".env", ".cargo/config.toml"];

/// Copy untracked config files into a new worktree and run the repo's setup hook.
///
/// - Copies any paths listed in `.worktree-cli/copy` (one path per line,
///   relative to the repo root, `#` for comments).
/// - Runs `.worktree-cli/setup.sh` (if present) with the new worktree as cwd.
///   Without one, `DEFAULT_COPY` is copied instead.
pub fn run(src_dir: &Path, dest_dir: &Path) -> Result<()> {
    let config_dir = src_dir.join(CONFIG_DIR);
    let setup_script = config_dir.join("setup.sh");
    let has_setup = setup_script.exists();

    let mut paths: Vec<String> = Vec::new();
    if !has_setup {
        paths.extend(DEFAULT_COPY.iter().map(|s| s.to_string()));
    }
    let copy_list = config_dir.join("copy");
    if copy_list.exists() {
        let contents = fs::read_to_string(&copy_list)
            .with_context(|| format!("Failed to read {}", copy_list.display()))?;
        for line in contents.lines() {
            let line = line.trim();
            if !line.is_empty() && !line.starts_with('#') && !paths.iter().any(|p| p == line) {
                paths.push(line.to_string());
            }
        }
    }

    for rel in &paths {
        let src = src_dir.join(rel);
        if !src.exists() {
            continue;
        }
        let dest = dest_dir.join(rel);
        copy_path(&src, &dest).with_context(|| format!("Failed to copy {}", rel))?;
        eprintln!("Copied {} to new worktree", rel);
    }

    if has_setup {
        eprintln!("Running {}/setup.sh", CONFIG_DIR);
        // stdout goes to stderr: the shell wrapper evals our stdout looking for `cd`
        let status = Command::new("sh")
            .arg(&setup_script)
            .current_dir(dest_dir)
            .env("WORKTREE_SOURCE", src_dir)
            .env("WORKTREE_PATH", dest_dir)
            .stdout(Stdio::from(io::stderr()))
            .status()
            .context("Failed to run setup script")?;
        if !status.success() {
            bail!("{}/setup.sh failed with {}", CONFIG_DIR, status);
        }
    }

    Ok(())
}

/// Copy a file or directory (recursively), creating parent directories as needed
fn copy_path(src: &Path, dest: &Path) -> Result<()> {
    if src.is_dir() {
        fs::create_dir_all(dest)?;
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            copy_path(&entry.path(), &dest.join(entry.file_name()))?;
        }
    } else {
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::copy(src, dest)?;
    }
    Ok(())
}
