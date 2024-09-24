use std::sync::Arc;

use anyhow::Result;
use axum::{Json, Router, extract::State, response::IntoResponse, routing::post, serve as axum_serve};
use clap::Args;
pub use mesh::models::AccountIdentifier;
use paste::paste;
use tokio::net::TcpListener;

use crate::{MinaMesh, util::Wrapper};

#[derive(Debug, Args)]
pub struct ServeArgs {
  #[arg(default_value_t = default_host())]
  host: String,
  #[arg(default_value_t = default_port())]
  port: u16,
}

impl Default for ServeArgs {
  fn default() -> Self {
    Self { host: default_host(), port: default_port() }
  }
}

fn default_host() -> String {
  "0.0.0.0".to_string()
}

fn default_port() -> u16 {
  3000
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
        mina_mesh: State<Arc<MinaMesh>>,
        Json(req): Json<crate::$request_type>,
      ) -> impl IntoResponse {
        Wrapper(mina_mesh.$name(req).await)
      }
    }
  };
  ($name:ident) => {
    paste! {
      async fn [<handle _ $name>](mina_mesh: State<Arc<MinaMesh>>) -> impl IntoResponse {
        Wrapper(mina_mesh.$name().await)
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
