use crate::solana::Solana;
use std::error::Error;
use types::shared::{Address, Blockchain, Net};

pub type ApiResult<T> = Result<T, Box<dyn Error>>;

pub struct Api {
    client: Box<dyn Client>,
}

impl Api {
    pub fn new(blockchain: Blockchain, net: Net) -> Self {
        Self {
            client: Api::get_client(blockchain, net),
        }
    }

    fn get_client(blockchain: Blockchain, net: Net) -> Box<dyn Client> {
        match blockchain {
            Blockchain::Solana => Box::new(Solana::new(net)),
            Blockchain::Bitcoin => Box::new(Solana::new(net)),
            Blockchain::Ethereum => Box::new(Solana::new(net)),
        }
    }

    pub fn get_balance(&self, address: &Address) -> ApiResult<u64> {
        self.client.get_balance(address)
    }
}

pub(crate) trait Client {
    fn new(net: Net) -> Self
    where
        Self: Sized;

    fn get_balance(&self, address: &Address) -> ApiResult<u64>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use types::Net;

    struct MockClient;

    impl Client for MockClient {
        fn new(_: Net) -> Self
        where
            Self: Sized,
        {
            MockClient
        }

        fn get_balance(&self, _: &Address) -> ApiResult<u64> {
            Ok(12)
        }
    }

    #[test]
    fn get_balance() {
        let address: Address = "u62NqVqrWp2wE47R8STvCFGwN7XxCq6YcWuYcAwsAWo".to_string();
        assert_eq!(
            MockClient::new(Net::Test).get_balance(&address).unwrap(),
            12
        );
    }
}
