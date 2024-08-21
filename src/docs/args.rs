use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct DocsCommand {
    #[clap(subcommand)]
    pub command: DocsSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum DocsSubcommand {
    /// Copy a /docs/ folder to the current directory from another. Useful for publishing docs on the website.
    Move(Move),
}

#[derive(Debug, Args)]
pub struct Move {
    /// The path to search for /docs/.
    #[arg(short, long)]
    pub path: String,
}
