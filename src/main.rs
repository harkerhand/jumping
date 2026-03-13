#![deny(clippy::unwrap_used, clippy::expect_used)]

mod app;
mod entry;
mod init;
mod ui;

use clap::Parser;
use std::io;

#[derive(Parser)]
#[command(name = "jumping")]
#[command(about = "Jumping Unlocks Multi-path Precise Instant Navigating Gear", long_about = None)]
struct Cli {
    /// Add a jump function to the shell initialization file
    #[arg(long)]
    init: bool,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    if cli.init {
        init::init();
        Ok(())
    } else {
        ui::run_tui_app()
    }
}
