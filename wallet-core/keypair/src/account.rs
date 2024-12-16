use crate::bitcoin::BitcoinKeypair;
use crate::solana::SolanaKeypair;
use std::collections::HashMap;
use std::error;
use std::str::from_utf8;
use types::{Address, Blockchain, Net};

pub trait Keypair {
    fn address(net: Net, seed: &[u8]) -> Result<Address, Box<dyn error::Error>>;
    fn pk(net: Net, seed: &[u8]) -> Result<String, Box<dyn error::Error>>;
}

pub struct Account {
    net: Net,
    pub addresses: HashMap<Blockchain, Address>,
}

impl Account {
    pub fn new(net: Net, seed: &[u8]) -> Result<Self, Box<dyn error::Error>> {
        Ok(Self {
            net,
            addresses: HashMap::from([
                (Blockchain::Bitcoin, BitcoinKeypair::address(net, seed)?),
                (Blockchain::Solana, SolanaKeypair::address(net, seed)?),
            ]),
        })
    }

    pub fn pk(&self, seed: &[u8]) -> Result<HashMap<Blockchain, String>, Box<dyn error::Error>> {
        Ok(HashMap::from([
            (Blockchain::Bitcoin, BitcoinKeypair::pk(self.net, seed)?),
            (Blockchain::Solana, SolanaKeypair::pk(self.net, seed)?),
        ]))
    }
}

const MAX_BASE58_LEN: usize = 64;

pub(crate) fn write_as_base58(key: Vec<u8>) -> String {
    let mut out = [0u8; MAX_BASE58_LEN];
    let out_slice: &mut [u8] = &mut out;
    // This will never fail because the only possible error is BufferTooSmall,
    // and we will never call it with too small a buffer.
    let len = bs58::encode(key).onto(out_slice).unwrap();
    from_utf8(&out[..len]).unwrap().to_string()
}
