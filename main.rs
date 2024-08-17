pub mod graphql_generated;
mod handlers;

use clap::{Parser, Subcommand};
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
  command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
  #[command(name = "serve", about = "Start the Mina Mesh Server.")]
  Serve,
}

#[tokio::main]
async fn main() {
  let args = MinaMeshArgs::parse();
  let MinaMeshArgs { command, .. } = args;
  match command {
    Some(Commands::Serve) => unimplemented!(),
    None => unimplemented!(),
  };
}
