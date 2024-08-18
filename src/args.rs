use clap::{Parser, Subcommand};
use crate::frontend::args::FrontendCommand;
use crate::newsletter::args::NewsletterCommand;


#[derive(Parser, Debug)]
#[clap(name = "ojos", version, about)]
pub struct Cli {
    #[clap(subcommand)]
    pub category: CategoryType
}

#[derive(Debug, Subcommand)]
pub enum CategoryType {
    /// Creates necessary `.tsx`, `.module.css`, and `/components/` files for a new frontend page.
    Frontend(FrontendCommand),

    /// Manages the configuration and publication of our email newsletter.
    Newsletter(NewsletterCommand)
}
