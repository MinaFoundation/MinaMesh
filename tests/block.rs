use std::sync::OnceLock;

use anyhow::Result;
use futures::{stream::FuturesUnordered, StreamExt};
use mina_mesh::{BlockMetadata, MinaMeshConfig, PartialBlockIdentifier};

#[tokio::test]
async fn specified() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;
  let mut metadata_futures =
    specified_identifiers().iter().map(|item| mina_mesh.block_metadata(item)).collect::<FuturesUnordered<_>>();
  let mut maybe_prev: Option<Option<BlockMetadata>> = None;
  while let Some(Ok(resolved)) = metadata_futures.next().await {
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

#[tokio::test]
async fn unspecified() -> Result<()> {
  let response = MinaMeshConfig::from_env()
    .to_mina_mesh()
    .await?
    .block_metadata(&PartialBlockIdentifier { hash: None, index: None })
    .await;
  assert!(response.is_ok());
  Ok(())
}
