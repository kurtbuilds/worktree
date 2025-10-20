use anyhow::Result;
use crate::utils;

pub fn execute() -> Result<()> {
    let main_worktree_path = utils::get_main_worktree_path()?;

    eprintln!("Changing to main worktree: {}", main_worktree_path.display());

    // Print the cd command for the shell to execute
    utils::print_cd_command(&main_worktree_path);

    Ok(())
}
