use anyhow::Result;
use axum::{
  extract::Json,
  http::StatusCode,
  response::{IntoResponse, Response},
};
use mesh::models::PartialBlockIdentifier;
use serde::Serialize;

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
  pub fn to_token_id(self) -> Result<String, MinaMeshError> {
    match self.0 {
      None => Ok(DEFAULT_TOKEN_ID.to_string()),
      Some(serde_json::Value::Object(map)) => {
        Ok(map.get("token_id").map(|v| v.to_string()).ok_or(MinaMeshError::JsonParse(None))?)
      }
      _ => Err(MinaMeshError::JsonParse(None))?,
    }
  }
}

// cspell:disable-next-line
const DEFAULT_TOKEN_ID: &str = "wSHV2S4qX9jFsLjQo8r1BsMLH2ZRKsZx6EJd1sbozGPieEC4Jf";

#[allow(clippy::to_string_trait_impl)]
impl ToString for Wrapper<&PartialBlockIdentifier> {
  fn to_string(&self) -> String {
    match &self.0.hash {
      Some(hash) => hash.to_owned(),
      None => match self.0.index {
        Some(index) => index.to_string(),
        None => "latest".to_string(),
      },
    }
  }
}
