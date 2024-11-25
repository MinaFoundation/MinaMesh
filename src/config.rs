use std::time::Duration;

use anyhow::Result;
use clap::{Args, Parser};
use coinbase_mesh::models::BlockIdentifier;
use cynic::QueryBuilder;
use dashmap::DashMap;
use sqlx::postgres::PgPoolOptions;

use crate::{
  graphql::{self, GraphQLClient},
  util::default_mina_proxy_url,
  MinaMesh, MinaMeshError,
};

#[derive(Debug, Args)]
pub struct MinaMeshConfig {
  /// The URL of the Mina GraphQL
  #[arg(long, env = "MINAMESH_PROXY_URL", default_value_t = default_mina_proxy_url())]
  pub proxy_url: String,

  /// The URL of the Archive Database
  #[arg(long, env = "MINAMESH_ARCHIVE_DATABASE_URL")]
  pub archive_database_url: String,

  /// The maximum number of concurrent connections allowed in the Archive
  /// Database connection pool.
  #[arg(long, env = "MINAMESH_MAX_DB_POOL_SIZE", default_value_t = 128)]
  pub max_db_pool_size: u32,

  /// The duration (in seconds) that an unused connection can remain idle in the
  /// pool before being closed.
  #[arg(long, env = "MINAMESH_DB_POOL_IDLE_TIMEOUT", default_value_t = 1)]
  pub db_pool_idle_timeout: u64,

  #[arg(long, env = "MINAMESH_GENESIS_BLOCK_IDENTIFIER_HEIGHT")]
  pub genesis_block_identifier_height: i64,
  #[arg(long, env = "MINAMESH_GENESIS_BLOCK_IDENTIFIER_STATE_HASH")]
  pub genesis_block_identifier_state_hash: String,

  /// Whether to use optimizations for searching transactions. Requires the
  /// optimizations to be enabled via the `mina-mesh search-tx-optimizations`
  /// command.
  #[arg(long, env = "USE_SEARCH_TX_OPTIMIZATIONS", default_value = "false")]
  pub use_search_tx_optimizations: bool,
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

  pub async fn to_mina_mesh(self) -> Result<MinaMesh, MinaMeshError> {
    if self.proxy_url.is_empty() {
      return Err(MinaMeshError::GraphqlUriNotSet);
    }
    tracing::info!("Connecting to Mina GraphQL endpoint at {}", self.proxy_url);
    let graphql_client = GraphQLClient::new(self.proxy_url.to_owned());
    let res = graphql_client.send(graphql::QueryGenesisBlockIdentifier::build(())).await?;
    tracing::debug!("Genesis block identifier: {}", res.genesis_block.protocol_state.consensus_state.block_height.0);
    tracing::debug!("Genesis block state hash: {}", res.genesis_block.state_hash.0);

    Ok(MinaMesh {
      graphql_client,
      pg_pool: PgPoolOptions::new()
        .max_connections(self.max_db_pool_size)
        .min_connections(0)
        .idle_timeout(Duration::from_secs(self.db_pool_idle_timeout))
        .connect(self.archive_database_url.as_str())
        .await?,
      genesis_block_identifier: BlockIdentifier::new(
        res.genesis_block.protocol_state.consensus_state.block_height.0.parse::<i64>().unwrap(),
        res.genesis_block.state_hash.0.clone(),
      ),
      search_tx_optimized: self.use_search_tx_optimizations,
      cache: DashMap::new(),
      cache_ttl: Duration::from_secs(300),
    })
  }
}
