use crate::account::{write_as_base58, Keypair};
use ed25519_dalek::SecretKey;
use std::error::Error;
use types::shared::{Address, Net};

/// A vanilla Ed25519 key pair.
#[derive(Debug)]
pub(super) struct SolanaKeypair(ed25519_dalek::SigningKey);

impl Keypair for SolanaKeypair {
    fn address(_: Net, seed: &[u8]) -> Result<Address, Box<dyn Error>> {
        let keypair = SolanaKeypair::from_seed(seed)?;
        Ok(keypair.address())
    }

    fn pk(_: Net, seed: &[u8]) -> Result<String, Box<dyn Error>> {
        let keypair = SolanaKeypair::from_seed(seed)?;
        Ok(keypair.pk())
    }
}

impl SolanaKeypair {
    fn from_seed(seed: &[u8]) -> Result<SolanaKeypair, Box<dyn Error>> {
        if seed.len() < ed25519_dalek::SECRET_KEY_LENGTH {
            return Err("Seed is too short".into());
        }
        let secret_key = SecretKey::try_from(&seed[..ed25519_dalek::SECRET_KEY_LENGTH])?;
        let signing_key = ed25519_dalek::SigningKey::from_bytes(&secret_key);
        Ok(SolanaKeypair(signing_key))
    }

    fn address(&self) -> Address {
        write_as_base58(self.0.verifying_key().to_bytes().to_vec())
    }

    fn pk(&self) -> String {
        write_as_base58(self.0.to_bytes().to_vec())
    }
}
