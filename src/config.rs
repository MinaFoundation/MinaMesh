use anyhow::Result;
use clap::{Args, Parser};
use mesh::models::BlockIdentifier;
use sqlx::PgPool;

use crate::{graphql::GraphQLClient, util::default_mina_proxy_url, MinaMesh};

#[derive(Debug, Args)]
pub struct MinaMeshConfig {
  #[arg(long, env = "MINAMESH_PROXY_URL", default_value_t = default_mina_proxy_url())]
  pub proxy_url: String,
  #[arg(long, env = "MINAMESH_ARCHIVE_DATABASE_URL")]
  pub archive_database_url: String,
  #[arg(long, env = "MINAMESH_GENESIS_BLOCK_IDENTIFIER_HEIGHT")]
  pub genesis_block_identifier_height: i64,
  #[arg(long, env = "MINAMESH_GENESIS_BLOCK_IDENTIFIER_STATE_HASH")]
  pub genesis_block_identifier_state_hash: String,
}

impl MinaMeshConfig {
  pub fn from_env() -> Self {
    dotenv::dotenv().ok();
    return MinaMeshConfigParser::parse().config;

    #[derive(Parser)]
    struct MinaMeshConfigParser {
      #[command(flatten)]
      config: MinaMeshConfig,
    }
  }

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
