use crate::cli::{Cli, Commands};
use clap::Parser;
use std::error;
use wallet::wallet::Wallet;

mod cli;

fn main() -> Result<(), Box<dyn error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::New) => {
            let wallet = Wallet::new(12, "", "en")?;
            println!("{wallet}")
        }
        None => {}
    }

    Ok(())
}
