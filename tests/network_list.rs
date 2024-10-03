use anyhow::Result;
use insta::assert_debug_snapshot;
use mina_mesh::MinaMeshConfig;

#[tokio::test]
async fn mainnet_test() -> Result<()> {
  let response = MinaMeshConfig::from_env().to_mina_mesh().await?.network_list().await?;
  assert_debug_snapshot!(&response.network_identifiers);
  Ok(())
}
