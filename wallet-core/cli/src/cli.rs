use clap::{Parser, Subcommand};
use types::{Blockchain, Net};

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
        word_count: Option<i32>,
        passphrase: Option<String>,
        lang_code: Option<String>,
    },
    /// Get a wallet balance.
    Balance {
        blockchain: Blockchain,
        net: Option<Net>,
        address: String,
    },
}
