use std::future::Future;

use anyhow::Result;
use axum::serve;
use clap::Args;
use tokio::net::TcpListener;

use crate::{create_router, MinaMeshConfig};

#[derive(Debug, Args)]
#[command(about = "Start the Mina Mesh Server.")]
pub struct ServeCommand {
  #[command(flatten)]
  config: MinaMeshConfig,
  #[arg(default_value = "0.0.0.0")]
  host: String,
  #[arg(default_value = "3000")]
  port: u16,
  /// Whether to enable the playground.
  #[arg(env = "PLAYGROUND", long)]
  playground: bool,
}

impl ServeCommand {
  pub async fn run<F>(self, signal: F) -> Result<()>
  where
    F: Future<Output = ()> + Send + 'static,
  {
    tracing_subscriber::fmt::init();
    let mina_mesh = self.config.to_mina_mesh().await?;
    let router = create_router(mina_mesh, self.playground);
    let listener = TcpListener::bind(format!("{}:{}", self.host, self.port)).await?;
    tracing::info!("listening on http://{}", listener.local_addr()?);
    serve(listener, router).with_graceful_shutdown(signal).await?;
    Ok(())
  }
}
