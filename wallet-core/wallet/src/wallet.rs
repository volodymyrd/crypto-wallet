use keypair::account::Account;
use std::fmt::Formatter;
use std::{error, fmt};
use types::constants::{TEXT_STYLE_BOLD, TEXT_STYLE_RESET, TEXT_STYLE_UNDERLINED};
use types::shared::Net;

pub struct Wallet {
    name: String,
    account: Account,
}

impl fmt::Display for Wallet {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "{}{}Wallet{}: {}",
            TEXT_STYLE_BOLD, TEXT_STYLE_UNDERLINED, TEXT_STYLE_RESET, self.name
        )?;
        writeln!(f, "{}", self.account)
    }
}

impl Wallet {
    pub fn new(
        name: &str,
        net: Net,
        language_code: &str,
        word_count: i32,
        passphrase: &str,
    ) -> Result<Self, Box<dyn error::Error>> {
        Ok(Self {
            name: name.to_string(),
            account: Account::new(net, language_code, word_count, passphrase)?,
        })
    }

    pub fn restore_from_seed(
        name: &str,
        net: Net,
        language_code: &str,
        seed_phrase: &str,
        passphrase: &str,
    ) -> Result<Self, Box<dyn error::Error>> {
        Ok(Self {
            name: name.to_string(),
            account: Account::restore_from_seed(net, language_code, seed_phrase, passphrase)?,
        })
    }
}
