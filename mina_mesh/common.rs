use anyhow::{Context, Result};
pub use mesh::models::{AccountIdentifier, NetworkIdentifier};
use mina_mesh_graphql::GraphQLClient;
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Debug)]
pub struct MinaMesh {
  pub env: MinaMeshEnv,
  pub graphql_client: GraphQLClient,
  pub pool: PgPool,
}

impl MinaMesh {
  pub async fn from_env() -> Result<Self> {
    let config = envy::from_env::<MinaMeshEnv>().context("Failed to parse config from env")?;
    let database_url = config.mina_archive_database_url.to_owned();
    let mina_proxy_url = config.mina_graphql_url.to_owned();
    Ok(Self {
      env: config,
      graphql_client: GraphQLClient::new(mina_proxy_url),
      pool: PgPool::connect(database_url.as_str()).await?,
    })
  }
}

#[derive(Deserialize, Debug, Default)]
pub struct MinaMeshEnv {
  #[serde(default = "default_mina_graphql_url", rename(deserialize = "MINA_GRAPHQL_URL"))]
  mina_graphql_url: String,
  #[serde(
    default = "default_mina_archive_database_url",
    rename(deserialize = "MINA_ARCHIVE_DATABASE_URL")
  )]
  mina_archive_database_url: String,
  #[serde(default = "default_genesis_block_identifier_height")]
  pub genesis_block_identifier_height: i64,
  #[serde(default = "default_genesis_block_identifier_state_hash")]
  pub genesis_block_identifier_state_hash: String,
}

fn default_mina_graphql_url() -> String {
  "https://mainnet.minaprotocol.network/graphql".to_string()
}

fn default_mina_archive_database_url() -> String {
  "postgres://mina:whatever@localhost:5432/archive".to_string()
}

fn default_genesis_block_identifier_height() -> i64 {
  359605
}

fn default_genesis_block_identifier_state_hash() -> String {
  "3NK4BpDSekaqsG6tx8Nse2zJchRft2JpnbvMiog55WCr5xJZaKeP".to_string()
}
