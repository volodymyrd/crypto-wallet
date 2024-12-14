use keypair::account::Account;
use mnemonic::{Mnemonic, WordCount};
use std::collections::HashMap;
use std::fmt::Formatter;
use std::{error, fmt};
use types::{Address, Blockchain};

pub struct Wallet {
    mnemonic: Mnemonic,
    addresses: HashMap<Blockchain, Address>,
}

impl fmt::Display for Wallet {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let divider = "=".repeat(self.mnemonic.len());
        // Write mnemonic section
        writeln!(f, "{}\n{}\n{}", divider, self.mnemonic, divider)?;

        // Write addresses section
        writeln!(f, "Addresses")?;
        self.addresses
            .iter()
            .try_for_each(|(k, v)| writeln!(f, "{}: {}", k, v))?;

        // Write closing divider
        writeln!(f, "{}", divider)
    }
}

impl Wallet {
    pub fn new(
        word_count: WordCount,
        passphrase: &str,
        language_code: &str,
    ) -> Result<Self, Box<dyn error::Error>> {
        let mnemonic = Mnemonic::new(language_code, word_count);
        let account = Account::new(&mnemonic.seed(passphrase))?;
        Ok(Self {
            mnemonic,
            addresses: account.addresses,
        })
    }
}
