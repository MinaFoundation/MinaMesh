use super::QueryGenesisBlockIdentifier;
use anyhow::bail;
use anyhow::Result;
use cynic::http::ReqwestExt;
use cynic::QueryBuilder;

pub async fn fetch_genesis_block_identifier(proxy_node_graphql_endpoint: String) -> Result<()> {
  let client = reqwest::Client::new();
  let result = client
    .post(proxy_node_graphql_endpoint)
    .run_graphql(QueryGenesisBlockIdentifier::build(()))
    .await?;
  if let Some(inner) = result.data {
    let genesis_block_hash = inner.genesis_block.state_hash.0;
    let genesis_block_index = inner.genesis_block.protocol_state.consensus_state.block_height.0;
    println!("MINAMESH_GENESIS_BLOCK_HASH = {}", genesis_block_hash);
    println!("MINAMESH_GENESIS_BLOCK_INDEX = {}", genesis_block_index);
  } else {
    bail!("No genesis block identifier found in the response");
  }
  Ok(())
}
