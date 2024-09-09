use anyhow::Result;
use futures::{stream::FuturesUnordered, StreamExt};
use mesh::models::partial_block_identifier;
use mina_mesh::{BlockMetadata, BlockRequest, BlockResponse, NetworkIdentifier, PartialBlockIdentifier, ServeCommand};

async fn setup(partial_block_identifier: PartialBlockIdentifier) -> Result<BlockMetadata> {
  let mina_mesh = ServeCommand::default().to_mina_mesh().await?;
  mina_mesh.block_metadata(partial_block_identifier).await
}

#[tokio::test]
async fn eq_when_specified() -> Result<()> {
  let mina_mesh = ServeCommand::default().to_mina_mesh().await?;
  let mut metadata_futures = FuturesUnordered::new();
  let partial_block_identifiers = vec![
    PartialBlockIdentifier {
      hash: None,
      index: Some(375991),
    },
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
async fn with_neither() -> Result<()> {
  let result = setup(PartialBlockIdentifier {
    hash: None,
    index: None,
  })
  .await;
  println!("{:?}", result);
  Ok(())
}
