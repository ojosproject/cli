use clap::{Args, Parser, Subcommand};


#[derive(Parser, Debug)]
#[clap(version, about)]
pub struct Cli {
    #[clap(subcommand)]
    pub team: Team
}

#[derive(Debug, Subcommand)]
pub enum Team {
    /// Tools for the Frontend team
    Frontend(FrontendCommand)
}

#[derive(Debug, Args)]
pub struct FrontendCommand{
    /// A file name, to skip the input portion
    #[arg(short, long)]
    pub filename: Option<String>,

    /// Nested mode, will create files in the current directory instead of app/src/
    #[arg(long, action)]
    pub nested: bool,

    /// Assume yes, skips the verification
    #[arg(short, action)]
    pub y: bool
}
