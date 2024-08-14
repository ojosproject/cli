use clap::{Args, Parser, Subcommand};


#[derive(Parser, Debug)]
#[clap(name = "ojos", version, about)]
pub struct Cli {
    #[clap(subcommand)]
    pub team: Team
}

#[derive(Debug, Subcommand)]
pub enum Team {
    /// Tools for the Frontend team
    Frontend(FrontendCommand),
    Trust(TrustCommand)
}

#[derive(Debug, Args)]
pub struct FrontendCommand{
    /// A page name, to skip the input portion
    #[arg(short, long)]
    pub name: Option<String>,

    /// Choose where to generate files
    #[arg(short, long, default_value_t=String::from("src/app/"))]
    pub dir: String,

    /// Assume yes, skips the verification
    #[arg(short, action)]
    pub y: bool
}

/// T(ypeScript)Rust converter. Convert Rust structs to TypeScript types!
#[derive(Debug, Args)]
pub struct TrustCommand {
    /// Where the input is located.
    #[arg(short, long, default_value_t=String::from("src-tauri/src/structs.rs"))]
    pub input: String,

    /// Where to dump the output.
    #[arg(short, long, default_value_t=String::from("src/types.ts"))]
    pub output: String
}
