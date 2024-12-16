use crate::bitcoin::BitcoinKeypair;
use crate::solana::SolanaKeypair;
use mnemonic::{Mnemonic, WordCount};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::from_utf8;
use types::shared::{Address, Blockchain, Net};

pub trait Keypair {
    fn address(net: Net, seed: &[u8]) -> Result<Address, Box<dyn Error>>;
    fn pk(net: Net, seed: &[u8]) -> Result<String, Box<dyn Error>>;
}

pub struct Account {
    net: Net,
    mnemonic: Mnemonic,
    addresses: Addresses,
    keys: Keys,
}

impl Account {
    pub fn new(
        net: Net,
        language_code: &str,
        word_count: i32,
        passphrase: &str,
    ) -> Result<Self, Box<dyn Error>> {
        let mnemonic = Mnemonic::new(language_code, WordCount::from(word_count));
        Account::build(net, mnemonic, passphrase)
    }
    pub fn restore_from_seed(
        net: Net,
        language_code: &str,
        seed_phrase: &str,
        passphrase: &str,
    ) -> Result<Self, Box<dyn Error>> {
        let mnemonic = Mnemonic::new_from_phrase(language_code, seed_phrase)?;
        Account::build(net, mnemonic, passphrase)
    }

    fn build(net: Net, mnemonic: Mnemonic, passphrase: &str) -> Result<Self, Box<dyn Error>> {
        let seed = mnemonic.seed(passphrase);
        Ok(Self {
            net,
            mnemonic,
            addresses: Addresses::new(net, &seed)?,
            keys: Keys::new(net, &seed)?,
        })
    }
}

impl Display for Account {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let divider = "=".repeat(self.mnemonic.len());

        // Write opening divider
        writeln!(f, "{}", divider)?;

        // Write addresses section
        writeln!(f, "Network: {}\n", self.net)?;

        // Write addresses section
        writeln!(f, "Addresses:")?;
        write!(f, "{}", self.addresses)?;

        // Write mnemonic section
        writeln!(f, "{}", self.mnemonic)?;

        // Write keys section
        writeln!(f, "Keys:")?;
        write!(f, "{}", self.keys)?;

        // Write closing divider
        writeln!(f, "{}", divider)
    }
}

struct Keys(HashMap<Blockchain, String>);

impl Keys {
    fn new(net: Net, seed: &[u8]) -> Result<Self, Box<dyn Error>> {
        Ok(Self(HashMap::from([
            (Blockchain::Bitcoin, BitcoinKeypair::pk(net, seed)?),
            (Blockchain::Solana, SolanaKeypair::pk(net, seed)?),
        ])))
    }
}

impl Display for Keys {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0
            .iter()
            .collect::<Vec<_>>()
            .iter()
            .try_for_each(|(k, v)| writeln!(f, "{}: {}", k, v))?;
        Ok(())
    }
}

struct Addresses(HashMap<Blockchain, Address>);

impl Addresses {
    fn new(net: Net, seed: &[u8]) -> Result<Self, Box<dyn Error>> {
        Ok(Self(HashMap::from([
            (Blockchain::Bitcoin, BitcoinKeypair::address(net, seed)?),
            (Blockchain::Solana, SolanaKeypair::address(net, seed)?),
        ])))
    }
}

impl Display for Addresses {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0
            .iter()
            .collect::<Vec<_>>()
            .iter()
            .try_for_each(|(k, v)| writeln!(f, "{}: {}", k, v))?;
        Ok(())
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
