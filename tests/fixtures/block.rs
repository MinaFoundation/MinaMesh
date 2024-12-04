use mina_mesh::{
  models::{BlockRequest, PartialBlockIdentifier},
  test::network_id,
};

use super::CompareGroup;

#[allow(dead_code)]
pub fn block<'a>() -> CompareGroup<'a> {
  ("/block", vec![
    Box::new(BlockRequest {
      network_identifier: Box::new(network_id()),
      block_identifier: Box::new(PartialBlockIdentifier::new()),
    }),
    Box::new(BlockRequest {
      network_identifier: Box::new(network_id()),
      block_identifier: Box::new(PartialBlockIdentifier { index: Some(52676), hash: None }),
    }),
  ])
}
