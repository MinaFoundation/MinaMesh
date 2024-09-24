use anyhow::Result;
use futures::{StreamExt, stream::FuturesUnordered};
use mina_mesh::{BlockMetadata, MinaMeshConfig, PartialBlockIdentifier};

#[tokio::test]
async fn specified() -> Result<()> {
  let mina_mesh = MinaMeshConfig::default().to_mina_mesh().await?;
  let mut metadata_futures = FuturesUnordered::new();
  let partial_block_identifiers = vec![
    PartialBlockIdentifier { hash: None, index: Some(375991) },
    PartialBlockIdentifier {
      hash: Some("3NKAyx2FuWx3jqtkpigndDRoyydQSUozPYix3hw3FWhZc8iUWwTP".to_string()),
      index: None,
    },
    PartialBlockIdentifier {
      hash: Some("3NKAyx2FuWx3jqtkpigndDRoyydQSUozPYix3hw3FWhZc8iUWwTP".to_string()),
      index: Some(375991),
    },
  ];
  for p in partial_block_identifiers {
    metadata_futures.push(mina_mesh.block_metadata(p));
  }
  let mut maybe_prev: Option<BlockMetadata> = None;
  while let Some(Ok(resolved)) = metadata_futures.next().await {
    if let Some(prev) = maybe_prev {
      assert_eq!(prev, resolved);
    }
    maybe_prev = Some(resolved);
  }
  Ok(())
}

#[tokio::test]
async fn unspecified() -> Result<()> {
  let mina_mesh = MinaMeshConfig::default().to_mina_mesh().await?;
  let result = mina_mesh.block_metadata(PartialBlockIdentifier { hash: None, index: None }).await;
  assert!(result.is_ok());
  Ok(())
}
