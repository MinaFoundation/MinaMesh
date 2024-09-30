use std::sync::Arc;

use anyhow::Result;
use axum::{
  debug_handler,
  extract::State,
  response::IntoResponse,
  routing::{get, post},
  serve, Json, Router,
};
use clap::Args;
use paste::paste;
use tokio::net::TcpListener;

use crate::{playground::handle_playground, util::Wrapper, MinaMesh, MinaMeshConfig};

#[derive(Debug, Args)]
#[command(about = "Start the Mina Mesh Server.")]
pub struct ServeCommand {
  #[command(flatten)]
  config: MinaMeshConfig,
  #[arg(default_value = "0.0.0.0")]
  host: String,
  #[arg(default_value = "3000")]
  port: u16,
  /// Whether to enable the playground.
  #[arg(env = "PLAYGROUND", long)]
  playground: bool,
}

impl ServeCommand {
  pub async fn run(self) -> Result<()> {
    tracing_subscriber::fmt::init();
    let mina_mesh = self.config.to_mina_mesh().await?;
    let mut router = Router::new()
      .route("/account/balance", post(handle_account_balance))
      .route("/block", post(handle_block))
      .route("/call", post(handle_call))
      .route("/construction/combine", post(handle_construction_combine))
      .route("/construction/derive", post(handle_construction_derive))
      .route("/construction/hash", post(handle_construction_hash))
      .route("/construction/metadata", post(handle_construction_metadata))
      .route("/construction/parse", post(handle_construction_parse))
      .route("/construction/payloads", post(handle_construction_payloads))
      .route("/construction/preprocess", post(handle_construction_preprocess))
      .route("/construction/submit", post(handle_construction_submit))
      .route("/implemented_methods", get(handle_implemented_methods))
      .route("/mempool", post(handle_mempool))
      .route("/mempool/transaction", post(handle_mempool_transaction))
      .route("/network/list", post(handle_network_list))
      .route("/network/options", post(handle_network_options))
      .route("/network/status", post(handle_network_status))
      .route("/search/transactions", post(handle_search_transactions))
      .with_state(Arc::new(mina_mesh));
    if self.playground {
      router = router.route("/", get(handle_playground));
    }
    let listener = TcpListener::bind(format!("{}:{}", self.host, self.port)).await?;
    tracing::info!("listening on http://{}", listener.local_addr()?);
    serve(listener, router).await?;
    Ok(())
  }
}

macro_rules! create_handler {
  ($name:ident, $request_type:ty) => {
    paste! {
      async fn [<handle _ $name>](mina_mesh: State<Arc<MinaMesh>>, Json(req): Json<coinbase_mesh::models::$request_type>) -> impl IntoResponse {
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

create_handler!(account_balance, AccountBalanceRequest);
create_handler!(block, BlockRequest);
create_handler!(call, CallRequest);
create_handler!(construction_combine, ConstructionCombineRequest);
create_handler!(construction_derive, ConstructionDeriveRequest);
create_handler!(construction_hash, ConstructionHashRequest);
create_handler!(construction_metadata, ConstructionMetadataRequest);
create_handler!(construction_parse, ConstructionParseRequest);
create_handler!(construction_payloads, ConstructionPayloadsRequest);
create_handler!(construction_preprocess, ConstructionPreprocessRequest);
create_handler!(construction_submit, ConstructionSubmitRequest);
create_handler!(mempool);
create_handler!(mempool_transaction, MempoolTransactionRequest);
create_handler!(network_list);
create_handler!(network_options);
create_handler!(network_status);
create_handler!(search_transactions, SearchTransactionsRequest);

#[debug_handler]
async fn handle_implemented_methods() -> impl IntoResponse {
  Json([
    "account_balance",
    "block",
    "mempool",
    "mempool_transaction",
    "network_list",
    "network_options",
    "network_status",
  ])
}
