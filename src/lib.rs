mod api;
mod commands;
mod config;
mod create_router;
mod error;
mod graphql;
mod operation;
mod playground;
mod sql_to_mesh;
pub mod test;
mod types;
pub mod util;

pub use coinbase_mesh::models;
use coinbase_mesh::models::BlockIdentifier;
pub use commands::*;
pub use config::*;
pub use create_router::create_router;
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
}
