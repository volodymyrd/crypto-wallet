use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub(super) struct Cli {
    #[command(subcommand)]
    pub(super) command: Option<Commands>,
}

#[derive(Subcommand)]
pub(super) enum Commands {
    /// Create a new wallet.
    New,
    /// Get a wallet balance.
    Balance { address: String },
}
