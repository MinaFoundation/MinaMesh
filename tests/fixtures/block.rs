use mina_mesh::{
  models::{BlockRequest, PartialBlockIdentifier},
  MinaNetwork,
};

use super::CompareGroup;
use crate::make_loc;

pub fn block<'a>() -> CompareGroup<'a> {
  ("/block", vec![
    (
      make_loc!(),
      Box::new(BlockRequest {
        network_identifier: Box::new(MinaNetwork::Devnet.into()),
        block_identifier: Box::new(PartialBlockIdentifier::new()),
      }),
    ),
    (
      make_loc!(),
      Box::new(BlockRequest {
        network_identifier: Box::new(MinaNetwork::Devnet.into()),
        block_identifier: Box::new(PartialBlockIdentifier { index: Some(52676), hash: None }),
      }),
    ),
  ])
}
