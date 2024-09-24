use anyhow::Result;
use clap::Args;

use crate::{MinaMeshConfig, ServeArgs};

#[derive(Debug, Args)]
pub struct ServeCommand {
  #[command(flatten)]
  config: MinaMeshConfig,
  #[command(flatten)]
  serve_args: ServeArgs,
}

impl ServeCommand {
  pub async fn execute(self) -> Result<()> {
    self.config.to_mina_mesh().await?.serve(self.serve_args).await
  }
}
