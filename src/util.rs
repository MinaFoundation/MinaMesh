use anyhow::Result;
use axum::{
  extract::Json,
  response::{IntoResponse, Response},
};
use serde::Serialize;

use crate::MinaMeshError;

pub struct Wrapper<T>(pub T);

impl<T: Serialize, E: ToString> IntoResponse for Wrapper<Result<T, E>>
where
  MinaMeshError: From<E>,
{
  fn into_response(self) -> Response {
    match self.0 {
      Ok(v) => Json(v).into_response(),
      Err(err) => {
        let mina_error: MinaMeshError = err.into();
        mina_error.into_response()
      }
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
pub const MINIMUM_USER_COMMAND_FEE: u64 = 1_000_000;

pub fn default_mina_proxy_url() -> String {
  "https://mainnet.minaprotocol.network/graphql".to_string()
}
