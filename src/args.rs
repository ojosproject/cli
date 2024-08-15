use clap::{Parser, Subcommand};
use crate::args_frontend_page::FrontendCommand;
use crate::args_newsletter::NewsletterCommand;


#[derive(Parser, Debug)]
#[clap(name = "ojos", version, about)]
pub struct Cli {
    #[clap(subcommand)]
    pub category: CategoryType
}

#[derive(Debug, Subcommand)]
pub enum CategoryType {
    /// Tools for the frontend team
    Frontend(FrontendCommand),

    /// Tools to interact with the Mailgun API
    Newsletter(NewsletterCommand)
}
