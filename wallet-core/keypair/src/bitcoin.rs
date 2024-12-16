use crate::account::{write_as_base58, Keypair};
use bitcoin::bip32::{DerivationPath, Xpriv};
use bitcoin::hashes::{sha256, Hash};
use bitcoin::{secp256k1, CompressedPublicKey, Network};
use std::error;
use std::error::Error;
use std::str::FromStr;
use types::{Address, Net};

#[derive(Debug)]
pub(super) struct BitcoinKeypair(Network, secp256k1::SecretKey, secp256k1::PublicKey);

impl Keypair for BitcoinKeypair {
    fn address(net: Net, seed: &[u8]) -> Result<Address, Box<dyn Error>> {
        let keypair = BitcoinKeypair::from_seed(convert(net), seed)?;
        Ok(keypair.address())
    }

    fn pk(net: Net, seed: &[u8]) -> Result<String, Box<dyn Error>> {
        let keypair = BitcoinKeypair::from_seed(convert(net), seed)?;
        Ok(keypair.secret_key_to_wif(true))
    }
}

impl BitcoinKeypair {
    fn from_seed(network: Network, seed: &[u8]) -> Result<BitcoinKeypair, Box<dyn error::Error>> {
        let secp = bitcoin::secp256k1::Secp256k1::new();
        let master_key = Xpriv::new_master(network, seed).unwrap();
        let derivation_path = DerivationPath::from_str("m/84'/0'/0'/0/0").unwrap();
        let child_key = master_key.derive_priv(&secp, &derivation_path).unwrap();
        let private_key = child_key.private_key;
        Ok(BitcoinKeypair(
            network,
            private_key,
            private_key.public_key(&secp),
        ))
    }

    fn address(&self) -> Address {
        bitcoin::address::Address::p2wpkh(&CompressedPublicKey(self.2), self.0).to_string()
    }

    fn secret_key_to_wif(&self, is_compressed: bool) -> String {
        // Step 1: Get the raw private key bytes
        let key_bytes = self.1.secret_bytes();

        // Step 2: Add the network prefix
        let mut extended_key = vec![];
        match self.0 {
            Network::Bitcoin => extended_key.push(0x80), // Mainnet prefix
            _ => extended_key.push(0xEF),                // Testnet prefix
        }
        extended_key.extend(&key_bytes);

        // Step 3: Add compression flag (optional)
        if is_compressed {
            extended_key.push(0x01);
        }

        // Step 4: Calculate checksum
        let checksum =
            sha256::Hash::hash(&sha256::Hash::hash(&extended_key).to_byte_array()).to_byte_array();
        extended_key.extend(&checksum[0..4]);

        // Step 5: Encode to Base58
        write_as_base58(extended_key)
    }
}

fn convert(net: Net) -> Network {
    match net {
        Net::Main => Network::Bitcoin,
        Net::Test => Network::Testnet,
        Net::Dev | Net::Local => Network::Regtest,
    }
}
