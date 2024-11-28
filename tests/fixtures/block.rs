use mina_mesh::models::{BlockRequest, NetworkIdentifier, PartialBlockIdentifier};

use super::CompareGroup;

#[allow(dead_code)]
pub fn block<'a>() -> CompareGroup<'a> {
  ("/block", vec![
    Box::new(BlockRequest {
      network_identifier: Box::new(NetworkIdentifier::new("mina".to_string(), "devnet".to_string())),
      block_identifier: Box::new(PartialBlockIdentifier::new()),
    }),
    Box::new(BlockRequest {
      network_identifier: Box::new(NetworkIdentifier::new("mina".to_string(), "devnet".to_string())),
      block_identifier: Box::new(PartialBlockIdentifier { index: Some(52676), hash: None }),
    }),
  ])
}
