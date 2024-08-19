use crate::utils::get_home;
use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct NewsletterCommand {
    #[clap(subcommand)]
    pub command: NewsletterSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum NewsletterSubcommand {
    /// Manage your configuration to interact with the Mailgun API.
    Config(Config),

    /// Publish to the newsletter.
    Publish(Publish),
}

#[derive(Debug, Args)]
pub struct Config {
    /// Where to save the `Newsletter` folder
    #[arg(short, long, default_value_t=String::from(get_home()))]
    pub path: String,

    /// Domain of the email
    #[arg(short, long)]
    pub domain: Option<String>,

    /// Our Mailgun API key
    #[arg(short, long)]
    pub api_key: Option<String>,

    /// The email to send information
    #[arg(short, long)]
    pub email: Option<String>,

    #[arg(short, long)]
    pub show: bool,
}

#[derive(Debug, Args)]
pub struct Publish {
    /// Which newsletter are you sending this to?
    pub newsletter: String,
}
