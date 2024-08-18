// new_page.rs
// Ojos Project
// 
// Creates a new page for the frontend developers.
mod tools;
mod args;
mod frontend;
mod newsletter;
use clap::Parser;
use args::{Cli, CategoryType};
use newsletter::{main as newsletter_utils, args::NewsletterSubcommand};
use frontend::main as frontend_utils;

fn main() {
    let cli = Cli::parse();

    match cli.category {
        CategoryType::Frontend(frontend) => {
            frontend_utils::create_page(frontend.name, frontend.dir, frontend.y)
        },
        CategoryType::Newsletter(newsletter) => {
            match newsletter.command {
                NewsletterSubcommand::Config(newsletter_config) => {
                    newsletter_utils::setup(
                        newsletter_config.path,
                        newsletter_config.email,
                        newsletter_config.domain,
                        newsletter_config.api_key,
                        newsletter_config.show);
                },
                NewsletterSubcommand::Publish(publish) => {
                    newsletter_utils::batch_send(publish.newsletter);
                }
            }
        }
    }
}
