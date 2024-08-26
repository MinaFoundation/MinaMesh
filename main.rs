pub mod graphql_generated;

use anyhow::{bail, Result};
use clap::{Args, Parser, Subcommand};
use cynic::{http::ReqwestExt, QueryBuilder};
use tokio;

#[derive(Debug, Parser)]
#[command(
  name = "mina-mesh",
  version,
  about = "A Mesh-compliant Server for Mina",
  propagate_version = true,
  author
)]
struct MinaMeshArgs {
  #[arg(long, short = 'v', default_value = "false", global = true)]
  verbose: bool,
  #[command(subcommand)]
  maybe_command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
  #[command(name = "serve", about = "Start the Mina Mesh Server.")]
  Serve,
  #[command(
    name = "fetch-genesis-block-identifier",
    about = "Retrieve the genesis block identifier via a proxy node GraphQL endpoint."
  )]
  FetchGenesisBlockIdentifier(FetchGenesisBlockIdentifierArgs),
}

#[derive(Debug, Args)]
struct FetchGenesisBlockIdentifierArgs {
  #[arg(long, short = 'n', default_value = "https://mainnet.minaprotocol.network/graphql")]
  proxy_node_graphql_endpoint: String,
}

#[tokio::main]
async fn main() {
  let args = MinaMeshArgs::parse();
  let MinaMeshArgs { maybe_command, .. } = args;
  match maybe_command {
    Some(command) => match command {
      Commands::Serve => unimplemented!(),
      Commands::FetchGenesisBlockIdentifier(args) => fetch_genesis_block_identifier(args).await,
    },
    None => unimplemented!(),
  }
  .expect("TODO");
}

// Initial genesis ledger got too big because / includes every existing account up until hardfork / when we call this endpoint, it takes a long time to return the identifier
async fn fetch_genesis_block_identifier(
  FetchGenesisBlockIdentifierArgs {
    proxy_node_graphql_endpoint,
  }: FetchGenesisBlockIdentifierArgs,
) -> Result<()> {
  let client = reqwest::Client::new();
  let result = client
    .post(proxy_node_graphql_endpoint)
    .run_graphql(graphql_generated::mina::QueryGenesisBlockIdentifier::build(()))
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
