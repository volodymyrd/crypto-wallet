use crate::solana::Solana;
use model::account::Account;
use std::error::Error;
use std::str::FromStr;

mod solana;

pub type ApiResult<T> = Result<T, Box<dyn Error>>;

pub enum Blockchain {
    // Bitcoin(Net),
    // Ethereum(Net),
    Solana(Net),
}

pub enum Net {
    Main,
    Test,
    Dev,
    Local,
}

pub struct Api {
    client: Box<dyn Client>,
}

impl Api {
    pub fn new(blockchain: Blockchain) -> Self {
        Self {
            client: Api::get_client(blockchain),
        }
    }

    fn get_client(blockchain: Blockchain) -> Box<dyn Client> {
        match blockchain {
            Blockchain::Solana(net) => Box::new(Solana::new(net)),
        }
    }

    pub fn get_balance(&self, account: &str) -> ApiResult<u64> {
        self.client.get_balance(&Account::from_str(account)?)
    }
}

pub(crate) trait Client {
    fn new(net: Net) -> Self
    where
        Self: Sized;

    fn get_balance(&self, account: &Account) -> ApiResult<u64>;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockClient;

    impl Client for MockClient {
        fn new(_: Net) -> Self
        where
            Self: Sized,
        {
            MockClient
        }

        fn get_balance(&self, _: &Account) -> ApiResult<u64> {
            Ok(12)
        }
    }

    #[test]
    fn get_balance() {
        assert_eq!(
            MockClient::new(Net::Test)
                .get_balance(
                    &Account::from_str("u62NqVqrWp2wE47R8STvCFGwN7XxCq6YcWuYcAwsAWo").unwrap()
                )
                .unwrap(),
            12
        );
    }
}
