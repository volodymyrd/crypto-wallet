use crate::cli::{Cli, Commands};
use api::client::Api;
use clap::Parser;
use std::error;
use types::shared::Net;
use wallet::wallet::Wallet;

mod cli;

fn main() -> Result<(), Box<dyn error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::New {
            name,
            net,
            language_code,
            word_count,
            passphrase,
        }) => {
            let wallet = Wallet::new(
                name,
                net.unwrap_or(Net::Dev),
                &language_code.clone().unwrap_or("en".to_string()),
                word_count.unwrap_or(12),
                &passphrase.clone().unwrap_or("".to_string()),
            )?;
            println!("{wallet}")
        }
        Some(Commands::Restore {
            name,
            net,
            language_code,
            seed_phrase,
            passphrase,
        }) => {
            let wallet = Wallet::restore_from_seed(
                name,
                *net,
                &language_code.clone(),
                &seed_phrase.clone(),
                &passphrase.clone().unwrap_or("".to_string()),
            )?;
            println!("{wallet}")
        }
        Some(Commands::Balance {
            blockchain,
            address,
            net,
        }) => {
            let api = Api::new(*blockchain, net.unwrap_or(Net::Dev));
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
