use crate::solana::SolanaKeypair;
use std::collections::HashMap;
use std::error;
use types::{Address, Blockchain};

pub trait Keypair {
    fn address(seed: &[u8]) -> Result<Address, Box<dyn error::Error>>;
}

pub struct Account {
    pub addresses: HashMap<Blockchain, Address>,
}

impl Account {
    pub fn new(seed: &[u8]) -> Result<Self, Box<dyn error::Error>> {
        Ok(Self {
            addresses: HashMap::from([(Blockchain::Solana, SolanaKeypair::address(seed)?)]),
        })
    }
}
