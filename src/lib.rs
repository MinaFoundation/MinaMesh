mod api;
mod commands;
mod config;
mod error;
mod graphql;
mod playground;
mod types;
mod util;

pub use commands::*;
pub use config::*;
pub use error::*;
use graphql::GraphQLClient;
pub use mesh::models;
use mesh::models::BlockIdentifier;
use sqlx::PgPool;
pub use types::*;

#[derive(Debug)]
pub struct MinaMesh {
  pub graphql_client: GraphQLClient,
  pub pg_pool: PgPool,
  pub genesis_block_identifier: BlockIdentifier,
}
