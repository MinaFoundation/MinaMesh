use std::fmt::Display;

use anyhow::Result;
use axum::{
  body::{to_bytes, Body},
  http::{Request, StatusCode},
  response::IntoResponse,
  Router,
};
use coinbase_mesh::models::{NetworkIdentifier, NetworkRequest};
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
    assert_eq!(a, b, "Mismatch for {subpath}; left = mina_mesh, right = rosetta");
    Ok(())
  }

  pub async fn assert_responses_contain(
    &self,
    subpath: &str,
    maybe_body_bytes: Option<Vec<u8>>,
    expected_fragment: &str,
  ) -> Result<()> {
    let body_bytes = maybe_body_bytes.clone().unwrap_or_default();
    let (a, b) =
      tokio::try_join!(self.mina_mesh_req(subpath, body_bytes.clone()), self.legacy_req(subpath, body_bytes))?;

    // Check if the expected fragment is present in both responses
    let a_contains = a.contains(expected_fragment);
    let b_contains = b.contains(expected_fragment);

    assert!(
      a_contains && b_contains,
      "Mismatch for {subpath}; expected fragment `{}` not found in one or both responses; mina_mesh: {}, rosetta: {}",
      expected_fragment,
      a,
      b
    );

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
  remove_empty_tx_fields(&mut json_unsorted);
  sort_transactions(&mut json_unsorted);
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
      *map = sorted_map;
    }
    Value::Array(vec) => {
      for v in vec.iter_mut() {
        sort_json_value(v);
      }
    }
    _ => {}
  }
}

// Remove empty "related_transactions" | "other_transactions" arrays from the
// JSON This is necessary because Rosetta OCaml includes empty arrays in the
// response but mina-mesh does not
// Workaround for https://github.com/MinaFoundation/MinaMesh/issues/48
fn remove_empty_tx_fields(value: &mut Value) {
  match value {
    Value::Object(map) => {
      map.retain(|key, v| {
        if key == "related_transactions" || key == "other_transactions" {
          !matches!(v, Value::Array(arr) if arr.is_empty())
        } else {
          true
        }
      });

      for v in map.values_mut() {
        remove_empty_tx_fields(v);
      }
    }
    Value::Array(vec) => {
      for v in vec.iter_mut() {
        remove_empty_tx_fields(v);
      }
    }
    _ => {}
  }
}

fn sort_transactions(value: &mut Value) {
  if let Some(block) = value.get_mut("block") {
    if let Some(Value::Array(tx_array)) = block.get_mut("transactions") {
      tx_array.sort_by(|a, b| {
        let hash_a =
          a.get("transaction_identifier").and_then(|ti| ti.get("hash")).and_then(|h| h.as_str()).unwrap_or("");
        let hash_b =
          b.get("transaction_identifier").and_then(|ti| ti.get("hash")).and_then(|h| h.as_str()).unwrap_or("");
        hash_a.cmp(hash_b)
      });
    }
  }
}

pub const DEVNET_BLOCKCHAIN_ID: &str = "mina";
pub const DEVNET_NETWORK_ID: &str = "testnet";

pub fn network_id() -> NetworkIdentifier {
  NetworkIdentifier::new(DEVNET_BLOCKCHAIN_ID.to_string(), DEVNET_NETWORK_ID.to_string())
}

pub fn network_request() -> NetworkRequest {
  NetworkRequest::new(network_id())
}
