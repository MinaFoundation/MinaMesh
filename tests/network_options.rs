use anyhow::Result;
use insta::assert_debug_snapshot;
use mina_mesh::MinaMeshConfig;

#[tokio::test]
async fn test_network_options() -> Result<()> {
  let response = MinaMeshConfig::from_env().to_mina_mesh().await?.network_options().await?;
  assert_debug_snapshot!(&response.allow);
  Ok(())
}
