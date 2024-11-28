use anyhow::Result;
use mina_mesh::{models::NetworkIdentifier, CacheKey::NetworkId, MinaMeshConfig, MinaMeshError};

#[tokio::test]
async fn genesis_block_identifier() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;
  assert_eq!(mina_mesh.genesis_block_identifier.index, 296372, "Devnet genesis block index does not match");
  assert_eq!(
    mina_mesh.genesis_block_identifier.hash, "3NL93SipJfAMNDBRfQ8Uo8LPovC74mnJZfZYB5SK7mTtkL72dsPx",
    "Devnet genesis block hash does not match"
  );
  Ok(())
}

#[tokio::test]
async fn validate_network_ok() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;
  let network = NetworkIdentifier::new("mina".to_string(), "devnet".to_string());

  assert!(mina_mesh.get_from_cache(NetworkId).is_none(), "Cache should be empty");
  let result = mina_mesh.validate_network(&network).await;
  assert!(result.is_ok(), "validate_network failed");
  if let Some(cached_network_id) = mina_mesh.get_from_cache(NetworkId) {
    assert_eq!(cached_network_id, "mina:devnet", "Cached network_id does not match");
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
  assert!(result.is_err(), "validate_network should have failed");
  if let Err(MinaMeshError::NetworkDne(expected, actual)) = result {
    assert_eq!(expected, "mina:unknown");
    assert_eq!(actual, "mina:devnet");
  } else {
    panic!("Unexpected error type");
  }

  Ok(())
}
