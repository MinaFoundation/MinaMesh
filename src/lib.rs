mod api;
mod commands;
mod config;
mod create_router;
mod error;
mod graphql;
pub mod memo;
mod playground;
mod roinput;
pub mod signer_utils;
pub mod test;
mod transaction_operations;
mod types;
pub mod util;

use std::time::{Duration, Instant};

pub use coinbase_mesh::models;
use coinbase_mesh::models::BlockIdentifier;
pub use commands::*;
pub use config::*;
pub use create_router::create_router;
use dashmap::DashMap;
pub use error::*;
use graphql::GraphQLClient;
pub(crate) use roinput::*;
use sqlx::PgPool;
pub use transaction_operations::*;
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
