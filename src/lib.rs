mod commands;
mod config;
mod errors;
mod graphql;
pub mod handlers;
mod util;

use anyhow::Result;
use axum::{
  body::Body,
  extract::State,
  http::StatusCode,
  response::{IntoResponse, Response},
  routing::post,
  serve as axum_serve, Json, Router,
};
use clap::Args;
pub use commands::*;
pub use config::*;
pub use errors::*;
pub use handlers::*;
pub use mesh::models::{AccountIdentifier, BlockIdentifier, NetworkIdentifier};
use paste::paste;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::net::TcpListener;
pub use util::*;

#[derive(Debug)]
pub struct MinaMesh {
  pub graphql_client: graphql::GraphQLClient,
  pub pg_pool: PgPool,
  pub genesis_block_identifier: BlockIdentifier,
}

impl MinaMesh {
  pub async fn serve(self, ServeArgs { host, port }: ServeArgs) -> Result<()> {
    let router = Router::new()
      .route("/network/list", post(handle_network_list))
      .route("/network/status", post(handle_network_status))
      .route("/network/options", post(handle_network_options))
      .route("/block", post(handle_block))
      .route("/mempool", post(handle_mempool))
      .route("/mempool/transaction", post(handle_mempool_transaction))
      .route("/account/balance", post(handle_account_balance))
      .route("/construction/derive", post(handle_construction_derive))
      .route("/construction/preprocess", post(handle_construction_preprocess))
      .route("/construction/metadata", post(handle_construction_metadata))
      .route("/construction/payloads", post(handle_construction_payloads))
      .route("/construction/combine", post(handle_construction_combine))
      .route("/construction/parse", post(handle_construction_parse))
      .route("/construction/hash", post(handle_construction_hash))
      .route("/construction/submit", post(handle_construction_submit))
      .route("/call", post(handle_call))
      .with_state(Arc::new(self));
    let listener = TcpListener::bind(format!("{}:{}", host, port)).await?;
    tracing::debug!("listening on {}", listener.local_addr()?);
    axum_serve(listener, router).await?;
    Ok(())
  }
}

macro_rules! create_handler {
  ($name:ident, $request_type:ty) => {
    paste! {
      async fn [<handle _ $name>](
        State(server): State<Arc<MinaMesh>>,
        Json(req): Json<crate::$request_type>,
      ) -> Response {
        match server.$name(req).await {
          Ok(d) => (StatusCode::OK, Json(d)).into_response(),
          Err(e) => anyhow_error_as_response(e),
        }
      }
    }
  };
  ($name:ident) => {
    paste! {
      async fn [<handle _ $name>](State(server): State<Arc<MinaMesh>>) -> Response {
        match server.$name().await {
          Ok(d) => (StatusCode::OK, Json(d)).into_response(),
          Err(e) => anyhow_error_as_response(e),
        }
      }
    }
  };
}

create_handler!(block, BlockRequest);
create_handler!(network_list);
create_handler!(network_status);
create_handler!(network_options);
create_handler!(mempool);
create_handler!(mempool_transaction, MempoolTransactionRequest);
create_handler!(account_balance, AccountBalanceRequest);
create_handler!(construction_derive, ConstructionDeriveRequest);
create_handler!(construction_preprocess, ConstructionPreprocessRequest);
create_handler!(construction_metadata, ConstructionMetadataRequest);
create_handler!(construction_payloads, ConstructionPayloadsRequest);
create_handler!(construction_combine, ConstructionCombineRequest);
create_handler!(construction_parse, ConstructionParseRequest);
create_handler!(construction_hash, ConstructionHashRequest);
create_handler!(construction_submit, ConstructionSubmitRequest);
create_handler!(call, CallRequest);

// TODO
fn anyhow_error_as_response(e: anyhow::Error) -> Response {
  Response::builder()
    .status(StatusCode::INTERNAL_SERVER_ERROR)
    .body(Body::from(e.to_string()))
    .unwrap()
}

#[derive(Debug, Args)]
pub struct ServeArgs {
  #[arg(default_value_t = default_host())]
  host: String,
  #[arg(default_value_t = default_port())]
  port: u16,
}

impl Default for ServeArgs {
  fn default() -> Self {
    Self {
      host: default_host(),
      port: default_port(),
    }
  }
}

fn default_host() -> String {
  "127.0.0.1".to_string()
}

fn default_port() -> u16 {
  3000
}
