use crate::cli::{Cli, Commands};
use api::Net::Dev;
use api::{Api, Blockchain};
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
        Some(Commands::Balance { address }) => {
            let api = Api::new(Blockchain::Solana(Dev));
            match api.get_balance(address) {
                Ok(balance) => {
                    println!("Balance for {address}: {balance}")
                }
                Err(e) => {
                    eprintln!("Error getting balance: {e}")
                }
            }
        }
        None => {}
    }

    Ok(())
}
