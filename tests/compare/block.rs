use erased_serde::Serialize as ErasedSerialize;
use mina_mesh::models::{BlockRequest, NetworkIdentifier, PartialBlockIdentifier};

pub fn requests() -> Vec<Box<dyn ErasedSerialize>> {
  vec![
    Box::new(BlockRequest {
      network_identifier: Box::new(NetworkIdentifier::new("mina".to_string(), "debug".to_string())),
      block_identifier: Box::new(PartialBlockIdentifier::new()),
    }),
    Box::new(BlockRequest {
      network_identifier: Box::new(NetworkIdentifier::new("mina".to_string(), "debug".to_string())),
      block_identifier: Box::new(PartialBlockIdentifier { index: Some(52676), hash: None }),
    }),
  ]
}
