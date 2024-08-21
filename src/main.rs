// new_page.rs
// Ojos Project
//
// Creates a new page for the frontend developers.
mod args;
mod docs;
mod frontend;
mod newsletter;
mod utils;
use args::{CategoryType, Cli};
use clap::Parser;
use frontend::main as frontend_utils;
use newsletter::{args::NewsletterSubcommand, main as newsletter_utils};

fn main() {
    let cli = Cli::parse();

    match cli.category {
        CategoryType::Frontend(frontend) => {
            frontend_utils::create_page(frontend.name, frontend.dir, frontend.y)
        }
        CategoryType::Newsletter(newsletter) => match newsletter.command {
            NewsletterSubcommand::Config(newsletter_config) => {
                newsletter_utils::setup(
                    newsletter_config.path,
                    newsletter_config.email,
                    newsletter_config.domain,
                    newsletter_config.api_key,
                    newsletter_config.show,
                );
            }
            NewsletterSubcommand::Publish(publish) => {
                newsletter_utils::batch_send(publish.newsletter);
            }
        },
        _ => {}
    }
}
