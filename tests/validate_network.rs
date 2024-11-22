use anyhow::Result;
use mina_mesh::{models::NetworkIdentifier, MinaMeshConfig, MinaMeshError};

#[tokio::test]
async fn validate_network_ok() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;
  let network = NetworkIdentifier::new("mina".to_string(), "testnet".to_string());
  let result = mina_mesh.validate_network(&network).await;

  assert!(result.is_ok());
  let cache_key = "network_id".to_string();
  if let Some(cached_entry) = mina_mesh.cache.get(&cache_key) {
    let (cached_network_id, _) = &*cached_entry;
    assert_eq!(cached_network_id, "mina:testnet", "Cached network_id does not match");
  } else {
    panic!("Cache was not updated after validate_network");
  }
  Ok(())
}

#[tokio::test]
async fn validate_network_err() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;
  let network = NetworkIdentifier::new("mina".to_string(), "unknown".to_string());
  let result = mina_mesh.validate_network(&network).await;
  assert!(result.is_err());
  if let Err(MinaMeshError::NetworkDne(expected, actual)) = result {
    assert_eq!(expected, "mina:unknown");
    assert_eq!(actual, "mina:testnet");
  } else {
    panic!("Unexpected error type");
  }

  Ok(())
}
