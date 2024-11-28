use mina_mesh::models::{NetworkIdentifier, SearchTransactionsRequest, TransactionIdentifier};

use super::CompareGroup;

pub fn search_transactions<'a>() -> CompareGroup<'a> {
  ("/search/transactions", vec![
    Box::new(SearchTransactionsRequest {
      network_identifier: Box::new(NetworkIdentifier::new("mina".to_string(), "devnet".to_string())),
      address: Some("B62qkd6yYALkQMq2SFd5B57bJbGBMA2QuGtLPMzRhhnvexRtVRycZWP".to_string()),
      limit: Some(5),
      offset: Some(0),
      ..Default::default()
    }),
    Box::new(SearchTransactionsRequest {
      network_identifier: Box::new(NetworkIdentifier::new("mina".to_string(), "devnet".to_string())),
      max_block: Some(44),
      status: Some("failed".to_string()),
      limit: Some(5),
      ..Default::default()
    }),
    Box::new(SearchTransactionsRequest {
      network_identifier: Box::new(NetworkIdentifier::new("mina".to_string(), "devnet".to_string())),
      max_block: Some(44),
      transaction_identifier: Some(Box::new(TransactionIdentifier::new(
        // cspell:disable-next-line
        "CkpYcKc2oGs8JUd4tmdGBsZXQCQVkayuyffEjrNWctX5Wuad3vVNe".to_string(),
      ))),
      limit: Some(5),
      ..Default::default()
    }),
    Box::new(SearchTransactionsRequest {
      network_identifier: Box::new(NetworkIdentifier::new("mina".to_string(), "devnet".to_string())),
      transaction_identifier: Some(Box::new(TransactionIdentifier::new(
        // cspell:disable-next-line
        "5JvFoEyvuPu9zmi4bDGbhqsakre2SPQU1KKbeh2Lk5uC9eYrc2h2".to_string(),
      ))),
      limit: Some(1),
      ..Default::default()
    }),
  ])
}
