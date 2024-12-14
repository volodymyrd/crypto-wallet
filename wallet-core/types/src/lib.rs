use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Blockchain {
    Bitcoin,
    Ethereum,
    Solana,
}

impl Blockchain {
    pub fn all_variants() -> &'static [Blockchain] {
        &[
            Blockchain::Bitcoin,
            Blockchain::Ethereum,
            Blockchain::Solana,
        ]
    }
}

impl Display for Blockchain {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Blockchain::Bitcoin => write!(f, "Bitcoin"),
            Blockchain::Ethereum => write!(f, "Ethereum"),
            Blockchain::Solana => write!(f, "Solana"),
        }
    }
}

#[derive(Debug)]
pub struct ParseBlockchainError;

impl Display for ParseBlockchainError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid blockchain name")
    }
}

impl Error for ParseBlockchainError {}

impl FromStr for Blockchain {
    type Err = ParseBlockchainError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_lowercase().as_str() {
            "bitcoin" => Ok(Blockchain::Bitcoin),
            "ethereum" => Ok(Blockchain::Ethereum),
            "solana" => Ok(Blockchain::Solana),
            _ => Err(ParseBlockchainError),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Net {
    Main,
    Test,
    Dev,
    Local,
}

#[derive(Debug)]
pub struct ParseNetError;

impl Display for ParseNetError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid net name")
    }
}

impl Error for ParseNetError {}

impl FromStr for Net {
    type Err = ParseNetError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_lowercase().as_str() {
            "main" => Ok(Net::Main),
            "test" => Ok(Net::Test),
            "dev" => Ok(Net::Dev),
            "local" => Ok(Net::Local),
            _ => Err(ParseNetError),
        }
    }
}

/// The wallet address.
pub type Address = String;
