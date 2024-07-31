mod graphql_generated;

use clap::{Parser, Subcommand};
use tokio;

#[derive(Debug, Parser)]
#[command(name = "mina-mesh", version, about = "A Mesh-compliant Server for Mina", propagate_version = true, author)]
struct MinaMeshArgs {
  #[arg(long, short = 'v', default_value = "false", global = true)]
  verbose: bool,
  #[command(subcommand)]
  command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
  #[command(about = "Start the Mina Mesh Server")]
  Start {
    #[arg(long, short = 'p', default_value = "8686")]
    port: u16,
    #[arg(long, default_value = "https://api.minascan.io/node/devnet/v1/graphql")]
    mina_proxy_url: String,
    #[arg(long, default_value = "https://api.minascan.io/archive/devnet/v1/graphql")]
    archive_url: String,
  },
}

#[tokio::main]
async fn main() {
  let args = MinaMeshArgs::parse();
  println!("Hello, Mina Mesh!, {:?}", args);
}
