use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::{from_utf8, FromStr};

/// The wallet account.
#[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Account {
    pub address: Pubkey,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParsePubkeyError {
    WrongSize,
    Invalid,
}

impl Display for ParsePubkeyError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ParsePubkeyError::WrongSize => f.write_str("String is the wrong size"),
            ParsePubkeyError::Invalid => f.write_str("Invalid Base58 string"),
        }
    }
}

impl std::error::Error for ParsePubkeyError {}

impl FromStr for Account {
    type Err = ParsePubkeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Account {
            address: Pubkey::from_str(s)?,
        })
    }
}

/// The address of a wallet.
#[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Pubkey([u8; 32]);

impl Pubkey {
    pub fn get_key(&self) -> [u8; 32] {
        self.0
    }

    pub fn get_ref_key(&self) -> &[u8; 32] {
        &self.0
    }
}

impl From<[u8; 32]> for Pubkey {
    #[inline]
    fn from(from: [u8; 32]) -> Self {
        Self(from)
    }
}

impl Display for Pubkey {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write_as_base58(f, self)
    }
}

/// Number of bytes in a pubkey
pub const PUBKEY_BYTES: usize = 32;

impl FromStr for Pubkey {
    type Err = ParsePubkeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > MAX_BASE58_LEN {
            return Err(ParsePubkeyError::WrongSize);
        }
        let mut bytes = [0; PUBKEY_BYTES];
        let decoded_size = bs58::decode(s)
            .onto(&mut bytes)
            .map_err(|_| ParsePubkeyError::Invalid)?;
        if decoded_size != size_of::<Pubkey>() {
            Err(ParsePubkeyError::WrongSize)
        } else {
            Ok(Pubkey(bytes))
        }
    }
}

const MAX_BASE58_LEN: usize = 44;

fn write_as_base58(f: &mut fmt::Formatter, p: &Pubkey) -> fmt::Result {
    let mut out = [0u8; MAX_BASE58_LEN];
    let out_slice: &mut [u8] = &mut out;
    // This will never fail because the only possible error is BufferTooSmall,
    // and we will never call it with too small a buffer.
    let len = bs58::encode(p.0).onto(out_slice).unwrap();
    let as_str = from_utf8(&out[..len]).unwrap();
    f.write_str(as_str)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
