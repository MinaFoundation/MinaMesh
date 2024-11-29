use mina_mesh::models::{NetworkIdentifier, NetworkRequest};
use serde::{ser::SerializeStruct, Serialize, Serializer};

use super::CompareGroup;

struct EmptyPayload;

impl Serialize for EmptyPayload {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    // Serialize the empty struct as an empty JSON object
    serializer.serialize_struct("EmptyPayload", 0)?.end()
  }
}

pub fn network_list<'a>() -> CompareGroup<'a> {
  ("/network/list", vec![Box::new(EmptyPayload)])
}

pub fn network_options<'a>() -> CompareGroup<'a> {
  ("/network/options", vec![Box::new(network_request())])
}

pub fn network_status<'a>() -> CompareGroup<'a> {
  ("/network/status", vec![Box::new(network_request())])
}

fn network_request() -> NetworkRequest {
  NetworkRequest::new(NetworkIdentifier::new("mina".to_string(), "devnet".to_string()))
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn test_empty_payload_serialization() {
    let payload = EmptyPayload;
    let serialized = serde_json::to_string(&payload).expect("Serialization failed");
    assert_eq!(serialized, "{}", "EmptyPayload did not serialize into an empty JSON object");
  }
}
