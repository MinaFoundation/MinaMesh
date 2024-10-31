use std::future::Future;

use anyhow::Result;
use axum::serve;
use clap::Args;
use tokio::net::TcpListener;

use crate::{create_router, playground::handle_playground, util::Wrapper, MinaMesh, MinaMeshConfig, MinaMeshError};

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
  pub async fn run<F>(self, signal: F) -> Result<()>
  where
    F: Future<Output = ()> + Send + 'static,
  {
    tracing_subscriber::fmt::init();
    let mina_mesh = self.config.to_mina_mesh().await?;
    let router = create_router(mina_mesh, self.playground);
    let listener = TcpListener::bind(format!("{}:{}", self.host, self.port)).await?;
    tracing::info!("listening on http://{}", listener.local_addr()?);
    serve(listener, router).with_graceful_shutdown(signal).await?;
    Ok(())
  }
}

macro_rules! create_handler {
  ($name:ident, $request_type:ty) => {
      paste! {
          async fn [<handle _ $name>](mina_mesh: State<Arc<MinaMesh>>, req: Result<Json<coinbase_mesh::models::$request_type>, axum::extract::rejection::JsonRejection>) -> impl IntoResponse {
              match req {
                  Ok(Json(req)) => Wrapper(mina_mesh.$name(req).await.map_err(MinaMeshError::from)), // Normalize errors to MinaMeshError
                  Err(err) => Wrapper(Err(MinaMeshError::from(err))), // Convert JsonRejection to MinaMeshError
              }
          }
      }
  };
  ($name:ident) => {
      paste! {
          async fn [<handle _ $name>](mina_mesh: State<Arc<MinaMesh>>) -> impl IntoResponse {
              Wrapper(mina_mesh.$name().await.map_err(MinaMeshError::from)) // Normalize errors to MinaMeshError
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
create_handler!(mempool, NetworkRequest);
create_handler!(mempool_transaction, MempoolTransactionRequest);
create_handler!(network_list);
create_handler!(network_options, NetworkRequest);
create_handler!(network_status, NetworkRequest);
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
    "search_transactions",
  ])
}
