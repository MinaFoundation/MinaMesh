pub mod docker;
mod shutdown_signal;

use anyhow::Result;
use axum::{
  extract::Json,
  http::StatusCode,
  response::{IntoResponse, Response},
};
use serde::Serialize;
pub use shutdown_signal::shutdown_signal;

use crate::MinaMeshError;

pub struct Wrapper<T>(pub T);

impl<T: Serialize, E: ToString> IntoResponse for Wrapper<Result<T, E>> {
  fn into_response(self) -> Response {
    match self.0 {
      Ok(v) => Json(v).into_response(),
      Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
  }
}

impl Wrapper<Option<serde_json::Value>> {
  pub fn to_token_id(&self) -> Result<String, MinaMeshError> {
    match &self.0 {
      None => Ok(DEFAULT_TOKEN_ID.to_string()),
      Some(serde_json::Value::Object(map)) => {
        Ok(map.get("token_id").map(|v| v.to_string()).ok_or(MinaMeshError::JsonParse(None))?)
      }
      _ => Err(MinaMeshError::JsonParse(None))?,
    }
  }
}

// cspell:disable-next-line
pub const DEFAULT_TOKEN_ID: &str = "wSHV2S4qX9jFsLjQo8r1BsMLH2ZRKsZx6EJd1sbozGPieEC4Jf";

pub fn default_mina_proxy_url() -> String {
  "https://mainnet.minaprotocol.network/graphql".to_string()
}