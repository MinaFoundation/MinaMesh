use mina_mesh::models::{NetworkIdentifier, SearchTransactionsRequest};

use super::CompareGroup;

pub fn search_transactions<'a>() -> CompareGroup<'a> {
  ("/search/transactions", vec![Box::new(SearchTransactionsRequest {
    network_identifier: Box::new(NetworkIdentifier::new("mina".to_string(), "devnet".to_string())),
    address: Some("B62qkd6yYALkQMq2SFd5B57bJbGBMA2QuGtLPMzRhhnvexRtVRycZWP".to_string()),
    limit: Some(5),
    offset: Some(0),
    ..Default::default()
  })])
}
