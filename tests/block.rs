use std::sync::OnceLock;

use anyhow::Result;
use futures::{stream::FuturesUnordered, StreamExt};
use insta::assert_debug_snapshot;
use mina_mesh::{
  models::{BlockRequest, BlockResponse, PartialBlockIdentifier},
  test::network_id,
  MinaMeshConfig, MinaMeshError,
};
use pretty_assertions::assert_eq;

#[tokio::test]
async fn specified() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;
  let mut futures = specified_identifiers()
    .iter()
    .map(|item| mina_mesh.block(BlockRequest::new(network_id(), item.to_owned())))
    .collect::<FuturesUnordered<_>>();
  let mut maybe_prev: Option<Result<BlockResponse, MinaMeshError>> = None;
  while let Some(resolved) = futures.next().await {
    if let Some(prev) = maybe_prev {
      assert_eq!(prev, resolved);
    }
    maybe_prev = Some(resolved);
  }
  assert_debug_snapshot!(maybe_prev);
  Ok(())
}

fn specified_identifiers() -> &'static [PartialBlockIdentifier; 3] {
  static IDENTIFIERS: OnceLock<[PartialBlockIdentifier; 3]> = OnceLock::new();
  IDENTIFIERS.get_or_init(|| {
    [
      PartialBlockIdentifier { hash: None, index: Some(355393) },
      PartialBlockIdentifier {
        hash: Some("3NLrvv2mG7qmheEzgwCJjYbbEjLq51iCsBPJztL4JbHGFRCo9488".to_string()),
        index: None,
      },
      PartialBlockIdentifier {
        hash: Some("3NLrvv2mG7qmheEzgwCJjYbbEjLq51iCsBPJztL4JbHGFRCo9488".to_string()),
        index: Some(355393),
      },
    ]
  })
}

#[tokio::test]
async fn unspecified() -> Result<()> {
  let response = MinaMeshConfig::from_env()
    .to_mina_mesh()
    .await?
    .block(BlockRequest::new(network_id(), PartialBlockIdentifier::new()))
    .await;
  assert!(response.is_ok());
  Ok(())
}
