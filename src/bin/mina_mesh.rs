// TODO: document workflow regarding fetching and using initial genesis ledger
// hash.

use anyhow::Result;
use clap::Parser;
use mina_mesh::{FetchGenesisBlockIdentifierCommand, ServeCommand};

#[derive(Debug, Parser)]
#[command(
  name = "mina-mesh",
  version,
  about = "A Mesh-compliant Server for Mina",
  propagate_version = true,
  author
)]
enum Command {
  #[command(about = "Start the Mina Mesh Server.")]
  Serve(ServeCommand),
  #[command(about = "Retrieve the genesis block identifier via a proxy node GraphQL endpoint.")]
  FetchGenesisBlockIdentifier(FetchGenesisBlockIdentifierCommand),
}

#[tokio::main]
async fn main() -> Result<()> {
  match Command::parse() {
    Command::Serve(cmd) => cmd.execute().await,
    Command::FetchGenesisBlockIdentifier(cmd) => cmd.execute().await,
  }
}
