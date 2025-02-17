use mina_mesh::{
  models::{SearchTransactionsRequest, TransactionIdentifier},
  test::network_id,
};

use super::CompareGroup;

#[allow(dead_code)]
pub fn search_transactions<'a>() -> CompareGroup<'a> {
  ("/search/transactions", vec![
    Box::new(SearchTransactionsRequest {
      network_identifier: Box::new(network_id()),
      address: Some("B62qkd6yYALkQMq2SFd5B57bJbGBMA2QuGtLPMzRhhnvexRtVRycZWP".to_string()),
      limit: Some(5),
      offset: Some(0),
      ..Default::default()
    }),
    Box::new(SearchTransactionsRequest {
      network_identifier: Box::new(network_id()),
      max_block: Some(44),
      status: Some("failed".to_string()),
      limit: Some(5),
      ..Default::default()
    }),
    Box::new(SearchTransactionsRequest {
      network_identifier: Box::new(network_id()),
      max_block: Some(44),
      transaction_identifier: Some(Box::new(TransactionIdentifier::new(
        // cspell:disable-next-line
        "CkpYcKc2oGs8JUd4tmdGBsZXQCQVkayuyffEjrNWctX5Wuad3vVNe".to_string(),
      ))),
      limit: Some(5),
      ..Default::default()
    }),
    Box::new(SearchTransactionsRequest {
      network_identifier: Box::new(network_id()),
      transaction_identifier: Some(Box::new(TransactionIdentifier::new(
        // cspell:disable-next-line
        "5JuBzSkpBQWVmHoL8bCxPqddrzs8CbhkHG2n5BayCYG11idS8LCs".to_string(),
      ))),
      limit: Some(1),
      ..Default::default()
    }),
  ])
}
