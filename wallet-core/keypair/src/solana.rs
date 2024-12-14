use crate::account::Keypair;
use ed25519_dalek::SecretKey;
use std::error;
use std::str::from_utf8;
use types::Address;

/// A vanilla Ed25519 key pair.
#[derive(Debug)]
pub(super) struct SolanaKeypair(ed25519_dalek::SigningKey);

impl Keypair for SolanaKeypair {
    fn address(seed: &[u8]) -> Result<Address, Box<dyn error::Error>> {
        let keypair = SolanaKeypair::from_seed(seed)?;
        Ok(keypair.address())
    }
}

impl SolanaKeypair {
    fn from_seed(seed: &[u8]) -> Result<SolanaKeypair, Box<dyn error::Error>> {
        if seed.len() < ed25519_dalek::SECRET_KEY_LENGTH {
            return Err("Seed is too short".into());
        }
        let secret_key = SecretKey::try_from(&seed[..ed25519_dalek::SECRET_KEY_LENGTH])?;
        let signing_key = ed25519_dalek::SigningKey::from_bytes(&secret_key);
        Ok(SolanaKeypair(signing_key))
    }

    fn address(&self) -> Address {
        write_as_base58(self.0.verifying_key().to_bytes())
    }
}
const MAX_BASE58_LEN: usize = 44;

fn write_as_base58(key: [u8; 32]) -> String {
    let mut out = [0u8; MAX_BASE58_LEN];
    let out_slice: &mut [u8] = &mut out;
    // This will never fail because the only possible error is BufferTooSmall,
    // and we will never call it with too small a buffer.
    let len = bs58::encode(key).onto(out_slice).unwrap();
    from_utf8(&out[..len]).unwrap().to_string()
}
