use std::error::Error;
use std::str::from_utf8;
use types::shared::Address;

pub mod account;
mod bitcoin;
mod solana;

pub(crate) trait Keypair {
    fn address(&self) -> Result<Address, Box<dyn Error>>;
    fn pk(&self) -> Result<String, Box<dyn Error>>;
}

const MAX_BASE58_LEN: usize = 64;

pub(crate) fn write_as_base58(key: Vec<u8>) -> String {
    let mut out = [0u8; MAX_BASE58_LEN];
    let out_slice: &mut [u8] = &mut out;
    // This will never fail because the only possible error is BufferTooSmall,
    // and we will never call it with too small a buffer.
    let len = bs58::encode(key).onto(out_slice).unwrap();
    from_utf8(&out[..len]).unwrap().to_string()
}
