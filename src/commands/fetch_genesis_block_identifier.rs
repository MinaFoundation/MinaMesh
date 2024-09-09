use crate::graphql::QueryGenesisBlockIdentifier;
use anyhow::{bail, Result};
use clap::Args;
use cynic::{http::ReqwestExt, QueryBuilder};

#[derive(Debug, Args)]
pub struct FetchGenesisBlockIdentifierCommand {
  #[arg(long, short = 'n', default_value = "https://mainnet.minaprotocol.network/graphql")]
  proxy_node_graphql_endpoint: String,
}

impl FetchGenesisBlockIdentifierCommand {
  pub async fn execute(&self) -> Result<()> {
    let client = reqwest::Client::new();
    let result = client
      .post(self.proxy_node_graphql_endpoint.to_owned())
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
}
