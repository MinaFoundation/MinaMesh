pub mod api;
mod commands;
mod config;
mod error;
mod graphql;
mod util;

pub use api::*;
pub use commands::*;
pub use config::*;
pub use error::*;
pub use mesh::models::{AccountIdentifier, BlockIdentifier, NetworkIdentifier};
use sqlx::PgPool;
pub use util::Wrapper;

#[derive(Debug)]
pub struct MinaMesh {
  pub graphql_client: graphql::GraphQLClient,
  pub pg_pool: PgPool,
  pub genesis_block_identifier: BlockIdentifier,
}
