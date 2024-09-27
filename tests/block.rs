use std::sync::OnceLock;

use anyhow::Result;
use futures::{stream::FuturesUnordered, StreamExt};
use mina_mesh::{
  BlockRequest, BlockResponse, MinaMeshConfig, MinaMeshError, NetworkIdentifier, PartialBlockIdentifier,
};

#[tokio::test]
async fn specified() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;
  let mut futures = specified_identifiers()
    .iter()
    .map(|item| mina_mesh.block(BlockRequest::new(network_identifier().to_owned(), item.to_owned())))
    .collect::<FuturesUnordered<_>>();
  let mut maybe_prev: Option<Result<BlockResponse, MinaMeshError>> = None;
  while let Some(resolved) = futures.next().await {
    if let Some(prev) = maybe_prev {
      assert_eq!(prev, resolved);
    }
    maybe_prev = Some(resolved);
  }
  Ok(())
}

fn specified_identifiers() -> &'static [PartialBlockIdentifier; 3] {
  static IDENTIFIERS: OnceLock<[PartialBlockIdentifier; 3]> = OnceLock::new();
  IDENTIFIERS.get_or_init(|| {
    [
      PartialBlockIdentifier { hash: None, index: Some(375991) },
      PartialBlockIdentifier {
        hash: Some("3NKAyx2FuWx3jqtkpigndDRoyydQSUozPYix3hw3FWhZc8iUWwTP".to_string()),
        index: None,
      },
      PartialBlockIdentifier {
        hash: Some("3NKAyx2FuWx3jqtkpigndDRoyydQSUozPYix3hw3FWhZc8iUWwTP".to_string()),
        index: Some(375991),
      },
    ]
  })
}

fn network_identifier() -> &'static NetworkIdentifier {
  static NETWORK_IDENTIFIER: OnceLock<NetworkIdentifier> = OnceLock::new();
  NETWORK_IDENTIFIER.get_or_init(|| NetworkIdentifier::new("mina".to_string(), "mainnet".to_string()))
}

#[tokio::test]
async fn unspecified() -> Result<()> {
  let response = MinaMeshConfig::from_env()
    .to_mina_mesh()
    .await?
    .block(BlockRequest::new(network_identifier().to_owned(), PartialBlockIdentifier::new()))
    .await;
  assert!(response.is_ok());
  Ok(())
}
