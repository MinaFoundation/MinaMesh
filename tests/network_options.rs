use anyhow::Result;
use insta::assert_debug_snapshot;
use mina_mesh::{
  models::{NetworkIdentifier, NetworkRequest},
  MinaMeshConfig,
};

#[tokio::test]
async fn test_network_options() -> Result<()> {
  let req = NetworkRequest::new(NetworkIdentifier::new("mina".to_string(), "testnet".to_string()));
  let response = MinaMeshConfig::from_env().to_mina_mesh().await?.network_options(req).await?;
  assert_debug_snapshot!(&response.allow);
  Ok(())
}
