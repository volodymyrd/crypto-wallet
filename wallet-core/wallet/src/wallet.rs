use keypair::account::Account;
use mnemonic::{Mnemonic, WordCount};
use std::fmt::Formatter;
use std::{error, fmt};
use types::Net;

pub struct Wallet {
    seed: Vec<u8>,
    mnemonic: Mnemonic,
    account: Account,
}

impl fmt::Display for Wallet {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let divider = "=".repeat(self.mnemonic.len());
        // Write mnemonic section
        writeln!(f, "{}\n{}\n{}", divider, self.mnemonic, divider)?;

        // Write addresses section
        writeln!(f, "Addresses")?;
        self.account
            .addresses
            .iter()
            .try_for_each(|(k, v)| writeln!(f, "{}: {}", k, v))?;

        // Write closing divider
        writeln!(f, "{}", divider)
    }
}

impl Wallet {
    pub fn new(
        net: Net,
        word_count: WordCount,
        passphrase: &str,
        language_code: &str,
    ) -> Result<Self, Box<dyn error::Error>> {
        let mnemonic = Mnemonic::new(language_code, word_count);
        let seed = mnemonic.seed(passphrase);
        let account = Account::new(net, &seed)?;
        Ok(Self {
            seed,
            mnemonic,
            account,
        })
    }

    pub fn restore(
        net: Net,
        language_code: &str,
        seed_phrase: &str,
        passphrase: &str,
    ) -> Result<Self, Box<dyn error::Error>> {
        let mnemonic = Mnemonic::new_from_phrase(language_code, seed_phrase)?;
        let seed = mnemonic.seed(passphrase);
        let account = Account::new(net, &seed)?;
        Ok(Self {
            seed,
            mnemonic,
            account,
        })
    }

    pub fn pk(&self) -> Result<(), Box<dyn error::Error>> {
        println!("{:?}", self.account.pk(&self.seed)?);
        Ok(())
    }
}
