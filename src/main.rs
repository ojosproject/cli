// new_page.rs
// Ojos Project
// 
// Creates a new page for the frontend developers.
use clap::Parser;
mod args;
mod frontend_page;
mod trust_converter;

use args::{Cli, Team};
use trust_converter::convert;
use crate::frontend_page::create_page;

fn main() {
    let cli = Cli::parse();

    match cli.team {
        Team::Frontend(frontend) => {
            create_page(frontend.name, frontend.dir, frontend.y);
        },
        Team::Trust(trust) => {
            convert(trust.input, trust.output);
        }
    }
}