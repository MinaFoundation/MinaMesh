use crate::graphql::GraphQLClient;
use anyhow::Result;
pub use mesh::models::AccountIdentifier;
pub use mesh::models::BlockIdentifier;
pub use mesh::models::NetworkIdentifier;
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Debug)]
pub struct MinaMesh {
  pub graphql_client: GraphQLClient,
  pub pg_pool: PgPool,
  pub genesis_block_identifier: BlockIdentifier,
}

impl MinaMesh {
  pub async fn from_env() -> Result<Self> {
    let env = envy::from_env::<MinaMeshEnv>()?;
    Ok(Self {
      graphql_client: GraphQLClient::new(env.mina_proxy_url),
      pg_pool: PgPool::connect(env.database_url.as_str()).await?,
      genesis_block_identifier: BlockIdentifier::new(
        env.genesis_block_identifier_height,
        env.genesis_block_identifier_state_hash,
      ),
    })
  }
}

#[derive(Deserialize, Debug, Default)]
struct MinaMeshEnv {
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
