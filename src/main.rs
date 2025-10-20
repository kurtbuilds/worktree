use clap::{Parser, Subcommand};
use anyhow::Result;

mod add;
mod list;
mod master;
mod remove;
mod utils;

#[derive(Parser)]
#[command(name = "worktree")]
#[command(about = "A CLI tool for managing git worktrees", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new git worktree
    #[command(visible_aliases = ["new", "init"])]
    Add {
        /// Name of the branch/worktree
        name: String,
    },
    /// Change directory to the master git repository
    Master,
    /// List all git worktrees
    List,
    /// Remove a git worktree
    #[command(visible_aliases = ["rm"])]
    Remove {
        /// Name of the worktree to remove, or '.' for current
        name: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { name } => add::execute(&name),
        Commands::Master => master::execute(),
        Commands::List => list::execute(),
        Commands::Remove { name } => remove::execute(&name),
    }
}
