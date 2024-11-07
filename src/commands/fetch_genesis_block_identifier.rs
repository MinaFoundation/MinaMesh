use anyhow::{bail, Result};
use clap::Args;
use cynic::{http::ReqwestExt, QueryBuilder};

use crate::{
  graphql::QueryGenesisBlockIdentifier,
  util::{default_devnet_proxy_url, default_mainnet_proxy_url},
};

#[derive(Debug, Args)]
#[command(about = "Retrieve the genesis block identifier via a proxy node GraphQL endpoint.")]
pub struct FetchGenesisBlockIdentifierCommand {
  #[arg(long, env = "MINAMESH_MAINNET_PROXY_URL", default_value_t = default_mainnet_proxy_url())]
  mainnet_proxy_url: String,
  #[arg(long, env = "MINAMESH_DEVNET_PROXY_URL", default_value_t = default_devnet_proxy_url())]
  devnet_proxy_url: String,
}

impl FetchGenesisBlockIdentifierCommand {
  pub async fn run(&self) -> Result<()> {
    let client = reqwest::Client::new();
    let (a, b) = tokio::try_join!(
      client.post(&self.mainnet_proxy_url).run_graphql(QueryGenesisBlockIdentifier::build(())),
      client.post(&self.devnet_proxy_url).run_graphql(QueryGenesisBlockIdentifier::build(()))
    )?;
    if let (Some(mainnet_inner), Some(devnet_inner)) = (a.data, b.data) {
      println!("MINAMESH_GENESIS_BLOCK_IDENTIFIER_STATE_HASH = {}", mainnet_inner.genesis_block.state_hash.0);
      println!(
        "MINAMESH_GENESIS_BLOCK_IDENTIFIER_HEIGHT = {}",
        mainnet_inner.genesis_block.protocol_state.consensus_state.block_height.0
      );
      println!("MINAMESH_GENESIS_BLOCK_IDENTIFIER_STATE_HASH = {}", devnet_inner.genesis_block.state_hash.0);
      println!(
        "MINAMESH_GENESIS_BLOCK_IDENTIFIER_HEIGHT = {}",
        devnet_inner.genesis_block.protocol_state.consensus_state.block_height.0
      );
    } else {
      // TODO: be more specific about error.
      bail!("Genesis block identifiers not found in the response.");
    }
    Ok(())
  }
}
