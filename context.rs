use anyhow::Context;
use anyhow::Result;
pub use mesh::models::AccountIdentifier;
pub use mesh::models::NetworkIdentifier;
use mina_mesh_graphql::GraphQLClient;
use serde::Deserialize;
use serde::Serialize;
use sqlx::PgPool;
use sqlx::Type;

#[derive(Debug)]
pub struct MinaMesh {
  pub env: MinaMeshEnv,
  pub graphql_client: GraphQLClient,
  pub pool: PgPool,
}

impl MinaMesh {
  pub async fn from_env() -> Result<Self> {
    let config = envy::from_env::<MinaMeshEnv>().context("Failed to parse config from env")?;
    let database_url = config.database_url.to_owned();
    let mina_proxy_url = config.mina_proxy_url.to_owned();
    Ok(Self {
      env: config,
      graphql_client: GraphQLClient::new(mina_proxy_url),
      pool: PgPool::connect(database_url.as_str()).await?,
    })
  }
}

#[derive(Deserialize, Debug, Default)]
pub struct MinaMeshEnv {
  #[serde(default = "default_mina_proxy_url")]
  mina_proxy_url: String,
  #[serde(default = "default_database_url")]
  database_url: String,
  #[serde(default = "default_genesis_block_identifier_height")]
  pub genesis_block_identifier_height: i64,
  #[serde(default = "default_genesis_block_identifier_state_hash")]
  pub genesis_block_identifier_state_hash: String,
}

fn default_mina_proxy_url() -> String {
  "https://mainnet.minaprotocol.network/graphql".to_string()
}

fn default_database_url() -> String {
  "postgres://mina:whatever@localhost:5432/archive".to_string()
}

fn default_genesis_block_identifier_height() -> i64 {
  359605
}

fn default_genesis_block_identifier_state_hash() -> String {
  "3NK4BpDSekaqsG6tx8Nse2zJchRft2JpnbvMiog55WCr5xJZaKeP".to_string()
}

#[derive(Type, Debug, Serialize, Deserialize)]
#[sqlx(type_name = "chain_status_type", rename_all = "lowercase")]
pub enum ChainStatus {
  Canonical,
  Orphaned,
  Pending,
}
