pub mod account;
mod block;
pub mod construction;
mod mempool;
pub mod network;

use anyhow::{Context as AnyhowContext, Result};
use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
pub struct Config {
  #[serde(default = "default_mina_proxy_url")]
  mina_proxy_url: String,
  #[serde(default = "default_archive_url")]
  archive_url: String,
}

fn default_mina_proxy_url() -> String {
  "https://api.minascan.io/node/devnet/v1/graphql".to_string()
}

fn default_archive_url() -> String {
  "https://api.minascan.io/archive/devnet/v1/graphql".to_string()
}

pub struct Context {
  config: Config,
  client: Client,
}

impl Context {
  fn from_env() -> Result<Self> {
    let config = envy::from_env::<Config>().with_context(|| "Failed to parse config from env")?;
    Ok(Self {
      config,
      client: Client::new(),
    })
  }
}
