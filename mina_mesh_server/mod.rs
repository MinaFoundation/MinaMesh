use anyhow::Result;
use axum::{
  body::Body,
  extract::State,
  http::StatusCode,
  response::{IntoResponse, Response},
  routing::post,
  routing::get,
  serve as axum_serve, Json, Router,
};
use mina_mesh::MinaMesh;
use paste::paste;
use std::sync::Arc;
use tokio::net::TcpListener;


pub async fn serve() -> Result<()> {
  tracing_subscriber::fmt::init();
  let mina_mesh = Arc::new(MinaMesh::from_env().await?);
  let router = Router::new()
    .route("/network/list", post(handle_network_list))
    .route("/network/status", post(handle_network_status))
    .route("/network/options", post(handle_network_options))
    .route("/block", post(handle_block))
    .route("/", get(|| -> &'static str {
      "Hello"
    }))
    .route("/block/transaction", post(handle_block_transaction))
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
    .with_state(mina_mesh);
  let listener = TcpListener::bind("127.0.0.1:6465").await?;
  tracing::debug!("listening on {}", listener.local_addr()?);
  axum_serve(listener, router).await?;
  Ok(())
}

macro_rules! create_handler {
  ($name:ident, $request_type:ty) => {
    paste! {
      async fn [<handle _ $name>](
        State(server): State<Arc<MinaMesh>>,
        Json(req): Json<mina_mesh::$request_type>,
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
create_handler!(block_transaction, BlockTransactionRequest);
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

fn anyhow_error_as_response(e: anyhow::Error) -> Response {
  Response::builder()
    .status(StatusCode::INTERNAL_SERVER_ERROR)
    .body(Body::from(e.to_string()))
    .unwrap()
}
