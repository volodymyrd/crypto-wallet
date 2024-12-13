use crate::{ApiResult, Client, Net};
use model::account::Account;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;

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

    fn get_balance(&self, account: &Account) -> ApiResult<u64> {
        self.client
            .get_balance(&Pubkey::from(account.address.get_key()))
            .map_err(|e| e.into())
    }
}
