use std::fmt::Display;

use anyhow::Result;
use axum::{
  body::{to_bytes, Body},
  http::{Request, StatusCode},
  response::IntoResponse,
  Router,
};
use pretty_assertions::assert_eq;
use reqwest::Client;
use serde_json::{Map, Value};
use tower::ServiceExt;

use crate::{create_router, MinaMesh};

pub struct ResponseComparisonContext {
  pub router: Router,
  pub client: Client,
  pub endpoint: String,
}

impl ResponseComparisonContext {
  pub fn new(mina_mesh: MinaMesh, endpoint: String) -> Self {
    let client = Client::new();
    let router = create_router(mina_mesh, false);
    Self { client, endpoint, router }
  }

  pub async fn assert_responses_eq(&self, subpath: &str, maybe_body_bytes: Option<Vec<u8>>) -> Result<()> {
    let body_bytes = maybe_body_bytes.clone().unwrap_or_default();
    let (a, b) =
      tokio::try_join!(self.mina_mesh_req(subpath, body_bytes.clone()), self.legacy_req(subpath, body_bytes))?;
    assert_eq!(a, b);
    Ok(())
  }

  async fn mina_mesh_req(&self, subpath: &str, body_bytes: Vec<u8>) -> Result<String> {
    let oneshot_req = Request::builder()
      .method("POST")
      .uri(subpath)
      .header(http::header::CONTENT_TYPE, "application/json")
      .body(Body::from(body_bytes))?;
    let response = self.router.clone().oneshot(oneshot_req).await.into_response();
    let status = response.status();
    let body_raw = String::from_utf8(to_bytes(response.into_body(), 100_000).await?.to_vec())?;
    let body = normalize_body(body_raw.as_str())?;
    if status == StatusCode::OK {
      Ok(body)
    } else {
      Ok(ErrorContainer { status: status.to_string(), body }.to_string())
    }
  }

  async fn legacy_req(&self, subpath: &str, body_bytes: Vec<u8>) -> Result<String> {
    let response = self.client.post(format!("{}{subpath}", self.endpoint)).body(body_bytes).send().await?;
    let status = response.status();
    let body = normalize_body(&response.text().await?)?;
    if status == StatusCode::OK {
      Ok(body)
    } else {
      Ok(ErrorContainer { status: status.to_string(), body }.to_string())
    }
  }
}

#[derive(Debug, PartialEq)]
struct ErrorContainer {
  status: String,
  body: String,
}

impl Display for ErrorContainer {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}: {}", self.status, self.body)
  }
}

fn normalize_body(raw: &str) -> Result<String> {
  let mut json_unsorted: Value = serde_json::from_str(raw)?;
  sort_json_value(&mut json_unsorted);
  Ok(serde_json::to_string_pretty(&json_unsorted)?)
}

fn sort_json_value(value: &mut Value) {
  match value {
    Value::Object(map) => {
      let mut keys: Vec<_> = map.keys().cloned().collect();
      keys.sort();
      let mut sorted_map = Map::new();
      for k in keys {
        if let Some(mut v) = map.remove(&k) {
          sort_json_value(&mut v);
          sorted_map.insert(k, v);
        }
      }
      *map = sorted_map
    }
    Value::Array(vec) => {
      for v in vec.iter_mut() {
        sort_json_value(v);
      }
    }
    _ => {}
  }
}
