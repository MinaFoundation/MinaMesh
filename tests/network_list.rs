use anyhow::Result;
use mina_mesh::{MinaMeshConfig, NetworkListResponse};

#[tokio::test]
async fn network_list_test() -> Result<()> {
  // Create a MinaMesh instance using the default configuration
  let mina_mesh = MinaMeshConfig::default().to_mina_mesh().await?;

  // Call the network_list function
  let result: NetworkListResponse = mina_mesh.network_list().await?;

  assert!(!result.network_identifiers.is_empty());

  let network_identifier = &result.network_identifiers[0];
  assert_eq!(network_identifier.blockchain, "mina");
  assert_eq!(network_identifier.network, "mainnet");

  Ok(())
}
