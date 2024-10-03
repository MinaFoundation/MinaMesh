use anyhow::{bail, Result};
use clap::Args;
use cynic::{http::ReqwestExt, QueryBuilder};

use crate::{graphql::QueryGenesisBlockIdentifier, util::default_mina_proxy_url};

#[derive(Debug, Args)]
#[command(about = "Retrieve the genesis block identifier via a proxy node GraphQL endpoint.")]
pub struct FetchGenesisBlockIdentifierCommand {
  #[arg(long, env = "MINAMESH_PROXY_URL", default_value_t = default_mina_proxy_url())]
  proxy_url: String,
}

impl FetchGenesisBlockIdentifierCommand {
  pub async fn run(&self) -> Result<()> {
    let client = reqwest::Client::new();
    let result = client.post(self.proxy_url.to_owned()).run_graphql(QueryGenesisBlockIdentifier::build(())).await?;
    if let Some(inner) = result.data {
      let genesis_block_hash = inner.genesis_block.state_hash.0;
      let genesis_block_index = inner.genesis_block.protocol_state.consensus_state.block_height.0;
      println!("MINAMESH_GENESIS_BLOCK_IDENTIFIER_STATE_HASH = {genesis_block_hash}");
      println!("MINAMESH_GENESIS_BLOCK_IDENTIFIER_HEIGHT = {genesis_block_index}");
    } else {
      bail!("No genesis block identifier found in the response");
    }
    Ok(())
  }
}
