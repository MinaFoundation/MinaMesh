mod api;
mod commands;
mod config;
mod error;
mod graphql;
mod operation;
mod playground;
mod sql_to_mesh;
mod types;
mod util;

use std::time::{Duration, Instant};

pub use coinbase_mesh::models;
use coinbase_mesh::models::BlockIdentifier;
pub use commands::*;
pub use config::*;
use dashmap::DashMap;
pub use error::*;
use graphql::GraphQLClient;
pub use operation::*;
use sqlx::PgPool;
pub use types::*;
#[derive(Debug)]
pub struct MinaMesh {
  pub graphql_client: GraphQLClient,
  pub pg_pool: PgPool,
  pub genesis_block_identifier: BlockIdentifier,
  pub search_tx_optimized: bool,
  pub cache: DashMap<String, (String, Instant)>, // Cache for network_id or other reusable data
  pub cache_ttl: Duration,
}
