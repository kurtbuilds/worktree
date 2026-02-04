use clap::{Parser, Subcommand};
use anyhow::Result;

mod add;
mod checkout;
mod list;
mod master;
mod merge;
mod remove;
mod utils;

#[derive(Parser)]
#[command(name = "worktree")]
#[command(about = "A CLI tool for managing git worktrees", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new git worktree
    #[command(visible_aliases = ["new", "init"])]
    Add {
        /// Name of the branch/worktree
        name: String,
    },
    /// Change directory to a worktree by name
    #[command(visible_aliases = ["co"])]
    Checkout {
        /// Name of the worktree (directory name or branch name)
        name: String,
    },
    /// Change directory to the master git repository
    Master,
    /// List all git worktrees
    #[command(visible_aliases = ["ls"])]
    List,
    /// Remove a git worktree
    #[command(visible_aliases = ["rm"])]
    Remove {
        /// Name of the worktree to remove, or '.' for current
        name: String,
    },
    /// Merge the PR for the current worktree and clean up
    Merge {
        /// Name of the worktree to merge (required if on main, optional if inside a worktree)
        name: Option<String>,
        /// Merge strategy: squash, merge, or rebase
        #[arg(short, long, default_value = "squash")]
        strategy: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Add { name }) => add::execute(&name),
        Some(Commands::Checkout { name }) => checkout::execute(&name),
        Some(Commands::Master) => master::execute(),
        Some(Commands::List) | None => list::execute(),
        Some(Commands::Remove { name }) => remove::execute(&name),
        Some(Commands::Merge { name, strategy }) => merge::execute(name.as_deref(), &strategy),
    }
}
