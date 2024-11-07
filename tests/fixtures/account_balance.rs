use mina_mesh::{
  models::{AccountBalanceRequest, AccountIdentifier, PartialBlockIdentifier},
  MinaNetwork,
};

use super::CompareGroup;
use crate::make_loc;

pub fn account_balance<'a>() -> CompareGroup<'a> {
  ("/account/balance", vec![
    (
      make_loc!(),
      Box::new(AccountBalanceRequest {
        account_identifier: Box::new(AccountIdentifier::new(
          "B62qmo4nfFemr9hFtvz8F5h4JFSCxikVNsUJmZcfXQ9SGJ4abEC1RtH".to_string(),
        )),
        block_identifier: Some(Box::new(PartialBlockIdentifier { index: Some(100), hash: None })),
        network_identifier: Box::new(MinaNetwork::Devnet.into()),
        currencies: None,
      }),
    ),
    (
      make_loc!(),
      Box::new(AccountBalanceRequest {
        account_identifier: Box::new(AccountIdentifier {
          address: "B62qkYHGYmws5CYa3phYEKoZvrENTegEhUJYMhzHUQe5UZwCdWob8zv".to_string(),
          sub_account: None,
          metadata: None,
        }),
        block_identifier: Some(Box::new(PartialBlockIdentifier { index: Some(6265), hash: None })),
        currencies: None,
        network_identifier: Box::new(MinaNetwork::Devnet.into()),
      }),
    ),
  ])
}
