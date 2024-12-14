use bip39::{Language, MnemonicType, Seed};
use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Copy, Clone)]
pub enum WordCount {
    W12,
    W15,
    W18,
    W21,
    W24,
}

impl From<WordCount> for MnemonicType {
    fn from(value: WordCount) -> Self {
        match value {
            WordCount::W12 => MnemonicType::Words12,
            WordCount::W15 => MnemonicType::Words15,
            WordCount::W18 => MnemonicType::Words18,
            WordCount::W21 => MnemonicType::Words21,
            WordCount::W24 => MnemonicType::Words24,
        }
    }
}

#[derive(Clone)]
pub struct Mnemonic {
    internal: bip39::Mnemonic,
}

impl fmt::Display for Mnemonic {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let phrase = self.seed_phrase();
        let divider = String::from_utf8(vec![b'='; phrase.len()]).unwrap();
        f.write_str(&format!(
            "{}\nSave this seed phrase to recover your key:\n{}\n{}",
            &divider, phrase, &divider
        ))
    }
}

impl Mnemonic {
    pub fn new_default() -> Self {
        Mnemonic::new("en", WordCount::W12)
    }

    pub fn new(language_code: &str, key_length: WordCount) -> Self {
        Self {
            internal: bip39::Mnemonic::new(key_length.into(), get_language(language_code)),
        }
    }

    pub fn new_from_phrase(language_code: &str, phrase: &str) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            internal: bip39::Mnemonic::from_phrase(phrase, get_language(language_code))?,
        })
    }

    pub fn seed_phrase(&self) -> &str {
        self.internal.phrase()
    }

    pub fn seed(&self, passphrase: &str) -> Vec<u8> {
        Vec::from(Seed::new(&self.internal, passphrase).as_bytes())
    }
}

fn get_language(language_code: &str) -> Language {
    Language::from_language_code(language_code).unwrap_or(Language::English)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_mnemonic() {
        assert_eq!(
            Mnemonic::new_default()
                .seed_phrase()
                .split(" ")
                .collect::<Vec<&str>>()
                .len(),
            12
        );
    }

    #[test]
    fn mnemonic_es_24() {
        println!("{}", Mnemonic::new_default().seed_phrase());
        assert_eq!(
            Mnemonic::new("es", WordCount::W24)
                .seed_phrase()
                .split(" ")
                .collect::<Vec<&str>>()
                .len(),
            24
        );
    }

    #[test]
    fn seed() {
        assert_eq!(
            Mnemonic::new_from_phrase(
                "en",
                "brisk fossil brisk dream dry sing lottery mountain sound void refuse pumpkin"
            )
            .unwrap()
            .seed(""),
            vec![
                73, 42, 249, 78, 22, 229, 13, 110, 113, 82, 248, 136, 14, 35, 138, 222, 48, 91,
                184, 149, 14, 90, 25, 216, 207, 25, 63, 166, 238, 150, 234, 157, 191, 111, 23, 149,
                231, 167, 236, 8, 152, 18, 245, 165, 139, 236, 63, 7, 141, 55, 244, 179, 181, 241,
                175, 96, 74, 95, 118, 95, 60, 128, 33, 56
            ]
        );
    }

    #[test]
    fn seed_with_pass() {
        assert_eq!(
            Mnemonic::new_from_phrase(
                "en",
                "brisk fossil brisk dream dry sing lottery mountain sound void refuse pumpkin"
            )
            .unwrap()
            .seed("paSword!"),
            vec![
                189, 161, 136, 153, 195, 227, 160, 129, 114, 228, 151, 19, 143, 125, 21, 129, 250,
                20, 50, 106, 157, 14, 219, 235, 238, 224, 80, 92, 0, 66, 55, 164, 220, 169, 172,
                78, 119, 128, 250, 217, 51, 48, 7, 152, 192, 97, 106, 73, 60, 138, 213, 211, 23,
                154, 62, 67, 103, 253, 147, 160, 140, 191, 232, 127
            ]
        );
    }
}
