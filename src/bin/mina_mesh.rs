// TODO: document workflow regarding fetching and using initial genesis ledger
// hash.

use anyhow::Result;
use clap::Parser;
use mina_mesh::{FetchGenesisBlockIdentifierCommand, ServeCommand};

#[derive(Debug, Parser)]
#[command(name = "mina-mesh", version, about = "A Mesh-compliant Server for Mina", propagate_version = true, author)]
enum Command {
  Serve(ServeCommand),
  FetchGenesisBlockIdentifier(FetchGenesisBlockIdentifierCommand),
}

#[tokio::main]
async fn main() -> Result<()> {
  dotenv::dotenv().ok();
  match Command::parse() {
    Command::Serve(cmd) => cmd.run().await,
    Command::FetchGenesisBlockIdentifier(cmd) => cmd.run().await,
  }
}
