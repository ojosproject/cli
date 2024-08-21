use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct DocsCommand {
    #[clap(subcommand)]
    pub command: DocsSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum DocsSubcommand {
    /// Copy a /docs/ folder to the current directory from another. Useful for publishing docs on the website.
    Copy(Copy),
}

#[derive(Debug, Args)]
pub struct Copy {
    /// The path to search for /docs/.
    #[arg(short, long)]
    pub input: String,

    /// Include the README at ../
    #[arg(short, long)]
    pub readme: bool,
}
