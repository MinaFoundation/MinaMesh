use anyhow::Result;
use insta::assert_debug_snapshot;
use mina_mesh::MinaMeshConfig;

#[tokio::test]
async fn mainnet_test() -> Result<()> {
  // Create a MinaMesh instance using the default configuration
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;

  // Call the network_list function
  let result = mina_mesh.network_list().await?;

  assert!(!result.network_identifiers.is_empty());

  let network_identifier = &result.network_identifiers[0];
  assert_debug_snapshot!(network_identifier);
  Ok(())
}
