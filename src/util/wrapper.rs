use anyhow::Result;
use axum::{
  extract::Json,
  http::StatusCode,
  response::{IntoResponse, Response},
};
use serde::Serialize;

pub struct Wrapper<T>(pub T);

impl<T: Serialize, E: ToString> IntoResponse for Wrapper<Result<T, E>> {
  fn into_response(self) -> Response {
    match self.0 {
      Ok(v) => Json(v).into_response(),
      Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
  }
}
