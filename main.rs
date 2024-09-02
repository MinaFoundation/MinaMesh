// TODO: document workflow regarding fetching and using initial genesis ledger hash.

use clap::Args;
use clap::Parser;
use clap::Subcommand;
use mina_mesh::fetch_genesis_block_identifier;
use mina_mesh::serve;
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
      Commands::Serve => serve().await,
      Commands::FetchGenesisBlockIdentifier(args) => {
        fetch_genesis_block_identifier(args.proxy_node_graphql_endpoint).await
      }
    },
    None => unimplemented!(),
  }
  .expect("TODO");
}
