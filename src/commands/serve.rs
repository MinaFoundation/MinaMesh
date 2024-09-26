use std::sync::Arc;

use anyhow::Result;
use axum::{extract::State, response::IntoResponse, routing::post, serve, Json, Router};
use clap::Args;
use paste::paste;
use tokio::net::TcpListener;

use crate::{util::Wrapper, MinaMesh, MinaMeshConfig};

#[derive(Debug, Args)]
#[command(about = "Start the Mina Mesh Server.")]
pub struct ServeCommand {
  #[command(flatten)]
  config: MinaMeshConfig,
  #[arg(default_value = "0.0.0.0")]
  host: String,
  #[arg(default_value = "3000")]
  port: u16,
}

impl ServeCommand {
  pub async fn run(self) -> Result<()> {
    let mina_mesh = self.config.to_mina_mesh().await?;
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
      .with_state(Arc::new(mina_mesh));
    let listener = TcpListener::bind(format!("{}:{}", self.host, self.port)).await?;
    tracing::debug!("listening on {}", listener.local_addr()?);
    serve(listener, router).await?;
    Ok(())
  }
}

macro_rules! create_handler {
  ($name:ident, $request_type:ty) => {
    paste! {
      async fn [<handle _ $name>](mina_mesh: State<Arc<MinaMesh>>, Json(req): Json<crate::$request_type>) -> impl IntoResponse {
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
