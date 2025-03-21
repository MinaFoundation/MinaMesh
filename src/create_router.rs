use std::sync::Arc;

use axum::{
  debug_handler,
  extract::State,
  response::IntoResponse,
  routing::{get, post},
  Json, Router,
};
use paste::paste;

use crate::{playground::handle_playground, util::Wrapper, MinaMesh, MinaMeshError};

pub fn create_router(mina_mesh: MinaMesh, playground: bool) -> Router {
  let mut router = Router::new()
    .route("/available_endpoints", get(handle_available_endpoints))
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
    .route("/mempool", post(handle_mempool))
    .route("/mempool/transaction", post(handle_mempool_transaction))
    .route("/network/list", post(handle_network_list))
    .route("/network/options", post(handle_network_options))
    .route("/network/status", post(handle_network_status))
    .route("/search/transactions", post(handle_search_transactions))
    .with_state(Arc::new(mina_mesh));
  if playground {
    router = router.route("/", get(handle_playground));
  }
  router
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
async fn handle_available_endpoints() -> impl IntoResponse {
  Json([
    "/account/balance",
    "/construction/derive",
    "/construction/combine",
    "/construction/hash",
    "/construction/parse",
    "/construction/payloads",
    "/construction/preprocess",
    "/construction/metadata",
    "/construction/submit",
    "/block",
    "/mempool",
    "/mempool/transaction",
    "/network/list",
    "/network/options",
    "/network/status",
    "/search/transactions",
  ])
}
