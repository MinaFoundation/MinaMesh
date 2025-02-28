use std::fmt::Display;

use anyhow::Result;
use axum::{
  body::{to_bytes, Body},
  http::{Request, StatusCode},
  response::IntoResponse,
  Router,
};
use coinbase_mesh::models::{
  AccountIdentifier, Amount, Currency, CurveType, NetworkIdentifier, NetworkRequest, Operation, OperationIdentifier,
  PublicKey, Signature, SignatureType, SigningPayload,
};
use pretty_assertions::assert_eq;
use reqwest::Client;
use serde_json::{json, Map, Value};
use tower::ServiceExt;

use crate::{create_router, MinaMesh, OperationType::*};

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
pub const DEVNET_NETWORK_ID: &str = "devnet";

pub fn network_id() -> NetworkIdentifier {
  NetworkIdentifier::new(DEVNET_BLOCKCHAIN_ID.to_string(), DEVNET_NETWORK_ID.to_string())
}

pub fn network_request() -> NetworkRequest {
  NetworkRequest::new(network_id())
}

pub fn payment_operations(
  (fee_act, fee_amt): (&str, &str),
  (sender_act, sender_amt): (&str, &str),
  (receiver_act, receiver_amt): (&str, &str),
) -> Vec<Operation> {
  vec![
    Operation {
      operation_identifier: OperationIdentifier::new(0).into(),
      related_operations: None,
      r#type: FeePayment.to_string(),
      account: Some(
        AccountIdentifier { address: fee_act.into(), sub_account: None, metadata: json!({ "token_id": "1" }).into() }
          .into(),
      ),
      amount: Some(Box::new(Amount::new(fee_amt.into(), Currency::new("MINA".into(), 9)))),
      coin_change: None,
      metadata: None,
      status: None,
    },
    Operation {
      operation_identifier: OperationIdentifier::new(1).into(),
      related_operations: None,
      r#type: PaymentSourceDec.to_string(),
      account: Some(
        AccountIdentifier {
          address: sender_act.into(),
          sub_account: None,
          metadata: json!({ "token_id": "1" }).into(),
        }
        .into(),
      ),
      amount: Some(Box::new(Amount::new(sender_amt.into(), Currency::new("MINA".into(), 9)))),
      coin_change: None,
      metadata: None,
      status: None,
    },
    Operation {
      operation_identifier: OperationIdentifier::new(2).into(),
      related_operations: vec![OperationIdentifier::new(1)].into(),
      r#type: PaymentReceiverInc.to_string(),
      account: Some(
        AccountIdentifier {
          address: receiver_act.into(),
          sub_account: None,
          metadata: json!({ "token_id": "1" }).into(),
        }
        .into(),
      ),
      amount: Some(Box::new(Amount::new(receiver_amt.into(), Currency::new("MINA".into(), 9)))),
      coin_change: None,
      status: None,
      metadata: None,
    },
  ]
}

pub fn delegation_operations(
  fee_act: &str,
  fee_amt: &str,
  source_act: &str,
  delegate_target_act: &str,
) -> Vec<Operation> {
  vec![
    Operation {
      operation_identifier: OperationIdentifier::new(0).into(),
      related_operations: None,
      r#type: FeePayment.to_string(),
      account: Some(AccountIdentifier::new(fee_act.into()).into()),
      amount: Some(Box::new(Amount::new(fee_amt.into(), Currency::new("MINA".into(), 9)))),
      coin_change: None,
      metadata: None,
      status: None,
    },
    Operation {
      operation_identifier: OperationIdentifier::new(1).into(),
      related_operations: None,
      r#type: DelegateChange.to_string(),
      account: Some(AccountIdentifier::new(source_act.into()).into()),
      amount: None,
      coin_change: None,
      metadata: Some(json!({
          "delegate_change_target": delegate_target_act
      })),
      status: None,
    },
  ]
}

//cspell:disable
pub fn unsigned_transaction_payment() -> String {
  r#"{
      "randomOracleInput": "000000035E6737A0AC0A147918437FC8C21EA57CECFB613E711CA2E4FD328401657C291C5E6737A0AC0A147918437FC8C21EA57CECFB613E711CA2E4FD328401657C291C5E6737A0AC0A147918437FC8C21EA57CECFB613E711CA2E4FD328401657C291C000002570561800000000000800000000000000081F0000001586000401013570767000000000000000000000000000000000000000000000000000000000E0000000000000000013E815200000000",
      "signerInput": {
          "prefix": [
              "5E6737A0AC0A147918437FC8C21EA57CECFB613E711CA2E4FD328401657C291C",
              "5E6737A0AC0A147918437FC8C21EA57CECFB613E711CA2E4FD328401657C291C",
              "5E6737A0AC0A147918437FC8C21EA57CECFB613E711CA2E4FD328401657C291C"
          ],
          "suffix": [
              "0001CDC1D5901004000C350000001F0200000000000000020000000000030D40",
              "0000000003800000000000000000000000000000000000000000000000000000",
              "00000000000000000000000000000000000000000000000009502F9000000000"
          ]
      },
      "payment": {
          "to": "B62qoDWfBZUxKpaoQCoFqr12wkaY84FrhxXNXzgBkMUi2Tz4K8kBDiv",
          "from": "B62qoDWfBZUxKpaoQCoFqr12wkaY84FrhxXNXzgBkMUi2Tz4K8kBDiv",
          "fee": "100000",
          "token": "1",
          "nonce": "1984",
          "memo": "dups",
          "amount": "5000000000",
          "valid_until": "200000"
      },
      "stakeDelegation": null
  }"#.to_string()
}

pub fn unsigned_transaction_delegation() -> String {
  r#"{
      "randomOracleInput": "0000000334411FBC9BF58536335A3B711494DA6EC9916AFC520F389B66D00796DCD9BA7A34411FBC9BF58536335A3B711494DA6EC9916AFC520F389B66D00796DCD9BA7A0131D887E9AE69AF4D40469B25411CB7EB94CDAD60E23B71E608B0A58FBCD4080000025704B85900000000008000000000000000E00000000158600040500B531B1B7B0000000000000000000000000000000000000000000000000000001A00000000000000000000000000000000",
      "signerInput": {
          "prefix": [
              "34411FBC9BF58536335A3B711494DA6EC9916AFC520F389B66D00796DCD9BA7A",
              "34411FBC9BF58536335A3B711494DA6EC9916AFC520F389B66D00796DCD9BA7A",
              "0131D887E9AE69AF4D40469B25411CB7EB94CDAD60E23B71E608B0A58FBCD408"
          ],
          "suffix": [
              "01BDB1B195A01404000C35000000000E00000000000000020000000001343A40",
              "0000000002C00000000000000000000000000000000000000000000000000000",
              "0000000000000000000000000000000000000000000000000000000000000000"
          ]
      },
      "payment": null,
      "stakeDelegation": {
          "delegator": "B62qkXajxfnicuCNtaurdAhQpkFsqjoyPJuw53aeJP848bsa3Ne3RvB",
          "new_delegate": "B62qiburnzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzmp7r7UN6X",
          "fee": "10100000",
          "nonce": "3",
          "memo": "hello",
          "valid_until": "200000"
      }
  }"#.to_string()
}

pub fn signature(sig_hex: &str, signature_type: SignatureType) -> Signature {
  Signature {
    signing_payload: SigningPayload::new("xxx".to_owned()).into(),
    public_key: PublicKey::new("xxx".to_owned(), CurveType::Tweedle).into(),
    signature_type,
    hex_bytes: sig_hex.into(),
  }
}
//cspell:enable
