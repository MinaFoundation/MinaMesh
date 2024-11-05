use mina_mesh::models::{AccountBalanceRequest, AccountIdentifier, NetworkIdentifier, PartialBlockIdentifier};

use super::CompareGroup;

pub fn account_balance<'a>() -> CompareGroup<'a> {
  ("/account/balance", vec![
    Box::new(AccountBalanceRequest {
      account_identifier: Box::new(AccountIdentifier::new(
        "B62qmo4nfFemr9hFtvz8F5h4JFSCxikVNsUJmZcfXQ9SGJ4abEC1RtH".to_string(),
      )),
      block_identifier: Some(Box::new(PartialBlockIdentifier { index: Some(100), hash: None })),
      network_identifier: Box::new(NetworkIdentifier::new("mina".to_string(), "devnet".to_string())),
      currencies: None,
    }),
    Box::new(AccountBalanceRequest {
      account_identifier: Box::new(AccountIdentifier {
        address: "B62qkYHGYmws5CYa3phYEKoZvrENTegEhUJYMhzHUQe5UZwCdWob8zv".to_string(),
        sub_account: None,
        metadata: None,
      }),
      block_identifier: Some(Box::new(PartialBlockIdentifier { index: Some(6265), hash: None })),
      currencies: None,
      network_identifier: Box::new(NetworkIdentifier {
        blockchain: "mina".into(),
        network: "testnet".into(),
        sub_network_identifier: None,
      }),
    }),
  ])
}
