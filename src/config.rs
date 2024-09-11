use anyhow::Result;
use clap::Args;
use mesh::models::BlockIdentifier;
use sqlx::PgPool;

use crate::{graphql::GraphQLClient, MinaMesh};

#[derive(Debug, Args)]
pub struct MinaMeshConfig {
  #[arg(long, env = "MINA_PROXY_URL", default_value_t = mina_proxy_url())]
  pub proxy_url: String,
  #[arg(long, env = "MINA_ARCHIVE_DATABASE_URL", default_value_t = database_url())]
  pub archive_database_url: String,
  #[arg(long, env = "MINA_GENESIS_BLOCK_IDENTIFIER_HEIGHT", default_value_t = genesis_block_identifier_height())]
  pub genesis_block_identifier_height: i64,
  #[arg(long, env = "MINA_GENESIS_BLOCK_IDENTIFIER_STATE_HASH", default_value_t = genesis_block_identifier_state_hash())]
  pub genesis_block_identifier_state_hash: String,
}

impl MinaMeshConfig {
  pub async fn to_mina_mesh(self) -> Result<MinaMesh> {
    Ok(MinaMesh {
      graphql_client: GraphQLClient::new(self.proxy_url.to_owned()),
      pg_pool: PgPool::connect(self.archive_database_url.as_str()).await?,
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
      proxy_url: mina_proxy_url(),
      archive_database_url: database_url(),
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
