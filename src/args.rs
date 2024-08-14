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

    /// Choose where to generate files
    #[arg(short, long, default_value_t=String::from("src/app/"))]
    pub dir: String,

    /// Assume yes, skips the verification
    #[arg(short, action)]
    pub y: bool
}
