use ed25519_dalek::SecretKey;
use std::{error};
use model::account::Pubkey;

/// A vanilla Ed25519 key pair.
#[derive(Debug)]
pub struct Keypair(ed25519_dalek::SigningKey);

impl Keypair {
    pub fn keypair_from_seed(seed: &[u8]) -> Result<Keypair, Box<dyn error::Error>> {
        if seed.len() < ed25519_dalek::SECRET_KEY_LENGTH {
            return Err("Seed is too short".into());
        }
        let secret_key = SecretKey::try_from(&seed[..ed25519_dalek::SECRET_KEY_LENGTH])?;
        let signing_key = ed25519_dalek::SigningKey::from_bytes(&secret_key);
        Ok(Keypair(signing_key))
    }

    #[inline]
    pub fn pubkey(&self) -> Pubkey {
        Pubkey::from(self.0.verifying_key().to_bytes())
    }
}
