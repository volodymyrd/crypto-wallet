use crate::cli::{Cli, Commands};
use api::client::{Api};
use clap::Parser;
use mnemonic::WordCount;
use std::error;
use types::Net;
use wallet::wallet::Wallet;

mod cli;

fn main() -> Result<(), Box<dyn error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::New {
            word_count,
            passphrase,
            lang_code,
        }) => {
            let wallet = Wallet::new(
                WordCount::from(word_count.unwrap_or(12)),
                &passphrase.clone().unwrap_or("".to_string()),
                &lang_code.clone().unwrap_or("en".to_string()),
            )?;
            println!("{wallet}")
        }
        Some(Commands::Balance {
            blockchain,
            net,
            address,
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
