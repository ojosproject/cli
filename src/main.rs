// new_page.rs
// Ojos Project
// 
// Creates a new page for the frontend developers.
mod tools;
mod args;
mod frontend_page;
mod newsletter;
mod args_frontend_page;
mod args_newsletter;

use args_newsletter::NewsletterSubcommand;
use clap::Parser;
use args::{Cli, CategoryType};

fn main() {
    let cli = Cli::parse();

    match cli.category {
        CategoryType::Frontend(frontend) => {
            frontend_page::create_page(frontend.name, frontend.dir, frontend.y)
        },
        CategoryType::Newsletter(newsletter) => {
            match newsletter.command {
                NewsletterSubcommand::Config(newsletter_config) => {
                    newsletter::setup(
                        newsletter_config.path,
                        newsletter_config.email,
                        newsletter_config.domain,
                        newsletter_config.api_key,
                        newsletter_config.show);
                },
                NewsletterSubcommand::Publish(publish) => {
                    newsletter::batch_send(publish.newsletter);
                }
            }
        }
    }
}
