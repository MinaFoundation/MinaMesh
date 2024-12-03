use mina_mesh::models::{AccountBalanceRequest, AccountIdentifier, PartialBlockIdentifier};

use super::{network_id, CompareGroup};

pub fn account_balance<'a>() -> CompareGroup<'a> {
  ("/account/balance", vec![
    // historical lookups
    Box::new(AccountBalanceRequest {
      account_identifier: Box::new(AccountIdentifier::new(
        "B62qmo4nfFemr9hFtvz8F5h4JFSCxikVNsUJmZcfXQ9SGJ4abEC1RtH".to_string(),
      )),
      block_identifier: Some(Box::new(PartialBlockIdentifier { index: Some(100), hash: None })),
      network_identifier: Box::new(network_id()),
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
      network_identifier: Box::new(network_id()),
    }),
    // current lookups
    Box::new(AccountBalanceRequest::new(
      network_id(),
      AccountIdentifier::new("B62qkYHGYmws5CYa3phYEKoZvrENTegEhUJYMhzHUQe5UZwCdWob8zv".to_string()),
    )),
  ])
}

pub fn account_balance_not_exists<'a>() -> CompareGroup<'a> {
  ("/account/balance", vec![Box::new(AccountBalanceRequest::new(
    network_id(),
    AccountIdentifier::new("B62qiW9Qwv9UnKfNKdBm6hRLNDobv46rVhX1trGdB35YCNT33CSCVt5".to_string()),
  ))])
}
