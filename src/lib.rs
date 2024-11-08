mod api;
mod commands;
mod config;
mod create_router;
mod error;
mod graphql;
mod network;
mod operation;
mod playground;
mod sql_to_mesh;
mod types;
pub mod util;

use std::time::Duration;

pub use coinbase_mesh::models;
use coinbase_mesh::models::BlockIdentifier;
pub use commands::*;
pub use config::*;
pub use create_router::create_router;
pub use error::*;
use graphql::GraphQLClient;
pub use network::*;
pub use operation::*;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tokio::sync::OnceCell;
pub use types::*;

#[derive(Debug)]
pub struct MinaMesh {
  pub graphql_client: GraphQLClient,
  pub mainnet_pg_pool: OnceCell<PgPool>,
  pub devnet_pg_pool: OnceCell<PgPool>,
  pub genesis_block_identifier: BlockIdentifier,
  pub search_tx_optimized: bool,
  pub db_pool_max_size: u32,
  pub db_pool_idle_timeout: u64,
  pub mainnet_archive_database_url: String,
  pub devnet_archive_database_url: String,
}

impl MinaMesh {
  async fn pool(&self, network: &MinaNetwork) -> Result<PgPool, MinaMeshError> {
    let pool = match &network {
      MinaNetwork::Mainnet => &self.mainnet_pg_pool,
      MinaNetwork::Devnet => &self.devnet_pg_pool,
    };
    let pool = pool
      .get_or_try_init(|| {
        PgPoolOptions::new()
          .max_connections(self.db_pool_max_size)
          .min_connections(0)
          .idle_timeout(Duration::from_secs(self.db_pool_idle_timeout))
          .connect(
            match &network {
              MinaNetwork::Mainnet => &self.mainnet_archive_database_url,
              MinaNetwork::Devnet => &self.devnet_archive_database_url,
            }
            .as_str(),
          )
      })
      .await?;
    Ok(pool.clone())
  }
}
