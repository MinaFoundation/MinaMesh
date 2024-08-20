pub mod account;
mod block;
pub mod construction;
mod mempool;
pub mod network;

use anyhow::{Context as AnyhowContext, Result};
use reqwest::Client;
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize, Debug, Default)]
pub struct Config {
  #[serde(default = "default_mina_proxy_url")]
  mina_proxy_url: String,
  #[serde(default = "default_archive_url")]
  archive_url: String,
  #[serde(default = "default_database_url")]
  database_url: String,
}

fn default_mina_proxy_url() -> String {
  "https://mainnet.minaprotocol.network/graphql".to_string()
}

fn default_archive_url() -> String {
  "https://api.minascan.io/archive/devnet/v1/graphql".to_string()
}

fn default_database_url() -> String {
  "postgres://mina:whatever@localhost:5432/archive".to_string()
}

pub struct Context {
  config: Config,
  client: Client,
  pool: PgPool,
}

impl Context {
  async fn from_env() -> Result<Self> {
    let config = envy::from_env::<Config>().with_context(|| "Failed to parse config from env")?;
    let database_url = config.database_url.clone();
    Ok(Self {
      config,
      client: Client::new(),
      pool: PgPool::connect(database_url.as_str()).await?,
    })
  }
}