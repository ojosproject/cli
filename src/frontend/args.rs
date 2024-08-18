use clap::Args;

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