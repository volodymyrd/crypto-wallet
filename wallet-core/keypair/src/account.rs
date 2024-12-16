use crate::bitcoin::BitcoinKeypair;
use crate::solana::SolanaKeypair;
use crate::Keypair;
use mnemonic::{Mnemonic, WordCount};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use types::shared::{Address, Blockchain, Net};

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
        let key_pairs: HashMap<Blockchain, KeypairType> = HashMap::from([
            (
                Blockchain::Bitcoin,
                KeypairType::Bitcoin(BitcoinKeypair::new(net, &seed)?),
            ),
            (
                Blockchain::Solana,
                KeypairType::Solana(SolanaKeypair::new(net, &seed)?),
            ),
        ]);

        Ok(Self {
            net,
            mnemonic,
            addresses: Addresses::new(&key_pairs)?,
            keys: Keys::new(&key_pairs)?,
        })
    }
}

enum KeypairType {
    Bitcoin(BitcoinKeypair),
    Solana(SolanaKeypair),
}

impl Keypair for KeypairType {
    fn address(&self) -> Result<Address, Box<dyn Error>> {
        match self {
            KeypairType::Bitcoin(keypair) => keypair.address(),
            KeypairType::Solana(keypair) => keypair.address(),
        }
    }

    fn pk(&self) -> Result<String, Box<dyn Error>> {
        match self {
            KeypairType::Bitcoin(keypair) => keypair.pk(),
            KeypairType::Solana(keypair) => keypair.pk(),
        }
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
    fn new(key_pairs: &HashMap<Blockchain, KeypairType>) -> Result<Self, Box<dyn Error>> {
        Ok(Self(
            key_pairs
                .iter()
                .map(|(&k, v)| v.pk().map(|pk| (k, pk)))
                .collect::<Result<_, _>>()?,
        ))
    }
}

impl Display for Keys {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut sorted = self.0.iter().collect::<Vec<_>>();
        sorted.sort_by_key(|(k, _)| *k);
        sorted
            .iter()
            .try_for_each(|(k, v)| writeln!(f, "{}: {}", k, v))?;
        Ok(())
    }
}

struct Addresses(HashMap<Blockchain, Address>);

impl Addresses {
    fn new(key_pairs: &HashMap<Blockchain, KeypairType>) -> Result<Self, Box<dyn Error>> {
        Ok(Self(
            key_pairs
                .iter()
                .map(|(&k, v)| v.address().map(|adr| (k, adr)))
                .collect::<Result<_, _>>()?,
        ))
    }
}

impl Display for Addresses {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut sorted = self.0.iter().collect::<Vec<_>>();
        sorted.sort_by_key(|(k, _)| *k);
        sorted
            .iter()
            .try_for_each(|(k, v)| writeln!(f, "{}: {}", k, v))?;
        Ok(())
    }
}
