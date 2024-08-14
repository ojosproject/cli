// new_page.rs
// Ojos Project
// 
// Creates a new page for the frontend developers.
use clap::Parser;
mod args;
mod frontend_page;

use args::{Cli, Team};
use crate::frontend_page::create_page;

fn main() {
    let cli = Cli::parse();

    match cli.team {
        Team::Frontend(frontend) => {
            create_page(frontend.filename, frontend.dir, frontend.y)
        },
    }
}