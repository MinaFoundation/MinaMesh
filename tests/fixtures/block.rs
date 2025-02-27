use mina_mesh::{
  models::{BlockRequest, PartialBlockIdentifier},
  test::network_id,
};

use super::CompareGroup;

pub fn block<'a>() -> CompareGroup<'a> {
  let blocks_by_index: Vec<Box<dyn erased_serde::Serialize + 'static>> = (373675 ..= 373705)
    .map(|index| {
      Box::new(BlockRequest {
        network_identifier: Box::new(network_id()),
        block_identifier: Box::new(PartialBlockIdentifier { index: Some(index), hash: None }),
      }) as Box<dyn erased_serde::Serialize>
    })
    .collect();

  let blocks_by_hash: Vec<Box<dyn erased_serde::Serialize + 'static>> = vec![
    Box::new(BlockRequest {
      network_identifier: Box::new(network_id()),
      block_identifier: Box::new(PartialBlockIdentifier {
        index: None,
        // cspell:disable-next-line
        hash: Some("3NLRpBDtzPySnPXGzKjFn4jsnPchoRk6N88NVGV3bexvdwJaptg1".to_string()),
      }),
    }),
    Box::new(BlockRequest {
      network_identifier: Box::new(network_id()),
      block_identifier: Box::new(PartialBlockIdentifier {
        index: Some(73706),
        // cspell:disable-next-line
        hash: Some("3NK8kTsPoTErvXN5PqtZVpztz6ZE8hmCATRAs8wTcdaevUsdALf3".to_string()),
      }),
    }),
  ];

  let blocks = blocks_by_index.into_iter().chain(blocks_by_hash).collect();
  ("/block", blocks)
}

pub fn block_not_found<'a>() -> CompareGroup<'a> {
  ("/block", vec![Box::new(BlockRequest {
    network_identifier: Box::new(network_id()),
    block_identifier: Box::new(PartialBlockIdentifier { index: None, hash: Some("not_found".to_string()) }),
  })])
}
