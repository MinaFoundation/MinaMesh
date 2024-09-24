use crate::{graphql::GraphQLClient, MinaMesh};
use anyhow::Result;
use clap::Args;
use mesh::models::BlockIdentifier;
use sqlx::PgPool;

#[derive(Debug, Args)]
pub struct MinaMeshConfig {
  #[arg(long, env, default_value_t = mina_proxy_url())]
  pub mina_proxy_url: String,
  #[arg(long, env, default_value_t = database_url())]
  pub database_url: String,
  #[arg(long, env, default_value_t = genesis_block_identifier_height())]
  pub genesis_block_identifier_height: i64,
  #[arg(long, env, default_value_t = genesis_block_identifier_state_hash())]
  pub genesis_block_identifier_state_hash: String,
}

impl MinaMeshConfig {
  pub async fn to_mina_mesh(&self) -> Result<MinaMesh> {
    Ok(MinaMesh {
      graphql_client: GraphQLClient::new(self.mina_proxy_url.to_owned()),
      pg_pool: PgPool::connect(self.database_url.as_str()).await?,
      genesis_block_identifier: BlockIdentifier::new(
        self.genesis_block_identifier_height,
        self.genesis_block_identifier_state_hash.to_owned(),
      ),
    })
  }
}

impl Default for MinaMeshConfig {
  fn default() -> Self {
    Self {
      mina_proxy_url: mina_proxy_url(),
      database_url: database_url(),
      genesis_block_identifier_height: genesis_block_identifier_height(),
      genesis_block_identifier_state_hash: genesis_block_identifier_state_hash(),
    }
  }
}

fn mina_proxy_url() -> String {
  "https://mainnet.minaprotocol.network/graphql".to_string()
}

fn database_url() -> String {
  "postgres://mina:whatever@localhost:5432/archive".to_string()
}

fn genesis_block_identifier_height() -> i64 {
  359605
}

fn genesis_block_identifier_state_hash() -> String {
  "3NK4BpDSekaqsG6tx8Nse2zJchRft2JpnbvMiog55WCr5xJZaKeP".to_string()
}
