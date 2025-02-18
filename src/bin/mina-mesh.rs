// TODO: document workflow regarding fetching and using initial genesis ledger
// hash.

use anyhow::Result;
use clap::Parser;
use mina_mesh::{SearchTxOptimizationsCommand, ServeCommand, SignCommand};
use tokio::{select, signal};

#[derive(Debug, Parser)]
#[command(name = "mina-mesh", version, about = "A Mesh-compliant Server for Mina", propagate_version = true, author)]
enum Command {
  Serve(ServeCommand),
  SearchTxOptimizations(SearchTxOptimizationsCommand),
  Sign(SignCommand),
}

#[tokio::main]
async fn main() -> Result<()> {
  dotenv::dotenv().ok();
  match Command::parse() {
    Command::Serve(cmd) => cmd.run(shutdown_signal()).await,
    Command::SearchTxOptimizations(cmd) => cmd.run().await,
    Command::Sign(cmd) => cmd.run().await,
  }
}

pub async fn shutdown_signal() {
  let windows = async {
    signal::ctrl_c().await.unwrap_or_else(|_| panic!("Error: failed to install windows shutdown handler"));
  };

  #[cfg(unix)]
  let unix = async {
    signal::unix::signal(signal::unix::SignalKind::terminate())
      .unwrap_or_else(|_| panic!("Error: failed to install unix shutdown handler"))
      .recv()
      .await;
  };

  #[cfg(not(unix))]
  let terminate = std::future::pending::<()>();

  select! {
    () = windows => {},
    () = unix => {},
  }

  println!("Signal received - starting graceful shutdown...");
}
