use clap::{Parser, Subcommand};
use types::shared::{Blockchain, Net};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub(super) struct Cli {
    #[command(subcommand)]
    pub(super) command: Option<Commands>,
}

#[derive(Subcommand)]
pub(super) enum Commands {
    /// Create a new wallet.
    New {
        name: String,
        net: Option<Net>,
        language_code: Option<String>,
        word_count: Option<i32>,
        passphrase: Option<String>,
    },
    /// Restore a wallet from a seed phrase.
    Restore {
        name: String,
        net: Net,
        language_code: String,
        seed_phrase: String,
        passphrase: Option<String>,
    },
    /// Get a wallet balance.
    Balance {
        blockchain: Blockchain,
        address: String,
        net: Option<Net>,
    },
}
