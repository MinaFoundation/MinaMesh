use anyhow::Result;
use clap::{Args, Parser};
use coinbase_mesh::models::BlockIdentifier;
use reqwest::Client;
use tokio::sync::OnceCell;

use crate::{
  graphql::GraphQLClient,
  util::{default_devnet_proxy_url, default_mainnet_proxy_url},
  MinaMesh,
};

#[derive(Debug, Args)]
pub struct MinaMeshConfig {
  /// The mainnet GraphQL endpoint.
  #[arg(long, env = "MINAMESH_MAINNET_PROXY_URL", default_value_t = default_mainnet_proxy_url())]
  pub mainnet_proxy_url: String,
  /// The devnet GraphQL endpoint.
  #[arg(long, env = "MINAMESH_DEVNET_PROXY_URL", default_value_t = default_devnet_proxy_url())]
  pub devnet_proxy_url: String,

  /// The mainnet archive database connection.
  #[arg(long, env = "MINAMESH_MAINNET_ARCHIVE_DATABASE_URL")]
  pub mainnet_archive_database_url: String,
  /// The mainnet archive database connection.
  #[arg(long, env = "MINAMESH_DEVNET_ARCHIVE_DATABASE_URL")]
  pub devnet_archive_database_url: String,

  /// The mainnet genesis block identifier height.
  #[arg(long, env = "MINAMESH_MAINNET_GENESIS_BLOCK_IDENTIFIER_HEIGHT")]
  pub mainnet_genesis_block_identifier_height: i64,
  /// The devnet genesis block identifier height.
  #[arg(long, env = "MINAMESH_DEVNET_GENESIS_BLOCK_IDENTIFIER_HEIGHT")]
  pub devnet_genesis_block_identifier_height: i64,

  /// The mainnet genesis block identifier state hash.
  #[arg(long, env = "MINAMESH_MAINNET_GENESIS_BLOCK_IDENTIFIER_STATE_HASH")]
  pub mainnet_genesis_block_identifier_state_hash: String,
  /// The devnet genesis block identifier state hash.
  #[arg(long, env = "MINAMESH_DEVNET_GENESIS_BLOCK_IDENTIFIER_STATE_HASH")]
  pub devnet_genesis_block_identifier_state_hash: String,

  /// The maximum number of concurrent connections allowed in the Archive
  /// Database connection pool.
  #[arg(long, env = "MINAMESH_MAX_DB_POOL_SIZE", default_value_t = 128)]
  pub db_pool_max_size: u32,

  /// The duration (in seconds) that an unused connection can remain idle in the
  /// pool before being closed.
  #[arg(long, env = "MINAMESH_DB_POOL_IDLE_TIMEOUT", default_value_t = 1)]
  pub db_pool_idle_timeout: u64,

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

  pub async fn to_mina_mesh(self) -> Result<MinaMesh> {
    Ok(MinaMesh {
      graphql_client: GraphQLClient {
        mainnet_endpoint: self.mainnet_proxy_url.to_owned(),
        devnet_endpoint: self.devnet_proxy_url.to_owned(),
        client: Client::new(),
      },
      mainnet_pg_pool: OnceCell::new(),
      devnet_pg_pool: OnceCell::new(),
      genesis_block_identifier: BlockIdentifier::new(
        self.devnet_genesis_block_identifier_height,
        self.devnet_genesis_block_identifier_state_hash.to_owned(),
      ),
      search_tx_optimized: self.use_search_tx_optimizations,
      mainnet_archive_database_url: self.mainnet_archive_database_url,
      devnet_archive_database_url: self.devnet_archive_database_url,
      db_pool_idle_timeout: self.db_pool_idle_timeout,
      db_pool_max_size: self.db_pool_max_size,
    })
  }
}
