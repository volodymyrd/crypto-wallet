use crate::keypair::Keypair;
use bip39::{Language, Mnemonic, MnemonicType, Seed};
use model::account::Pubkey;
use std::fmt::Formatter;
use std::{error, fmt};

pub struct Wallet {
    phrase: String,
    pubkey: Pubkey,
}

impl fmt::Display for Wallet {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let divider = String::from_utf8(vec![b'='; self.phrase.len()]).unwrap();
        f.write_str(&format!(
            "{}\npubkey: {}\n{}\nSave this seed phrase to recover your new keypair:\n{}\n{}",
            &divider, self.pubkey, &divider, self.phrase, &divider
        ))
    }
}

impl Wallet {
    pub fn new(
        word_count: usize,
        passphrase: &str,
        language_code: &str,
    ) -> Result<Self, Box<dyn error::Error>> {
        let mnemonic_type = MnemonicType::for_word_count(word_count)?;
        let mnemonic = Mnemonic::new(
            mnemonic_type,
            Language::from_language_code(language_code).unwrap_or(Language::English),
        );
        let seed = Seed::new(&mnemonic, passphrase);
        let keypair = Keypair::keypair_from_seed(seed.as_bytes())?;

        Ok(Self {
            phrase: String::from(mnemonic.phrase()),
            pubkey: keypair.pubkey(),
        })
    }
}
