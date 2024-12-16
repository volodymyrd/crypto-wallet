use crate::client::{ApiResult, Client};
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use types::shared::{Address, Net};

pub(super) struct Solana {
    client: RpcClient,
}

impl Client for Solana {
    fn new(net: Net) -> Self {
        Solana {
            client: RpcClient::new(match net {
                Net::Main => "https://api.mainnet-beta.solana.com".to_string(),
                Net::Test => "https://api.testnet.solana.com".to_string(),
                Net::Dev => "https://api.devnet.solana.com".to_string(),
                Net::Local => "http://localhost:8899".to_string(),
            }),
        }
    }

    fn get_balance(&self, address: &Address) -> ApiResult<u64> {
        self.client
            .get_balance(&Pubkey::from_str(address)?)
            .map_err(|e| e.into())
    }
}
