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

#[cfg(test)]
mod tests {
  use serde_json;

  use super::*;

  #[test]
  fn test_empty_payload_serialization() {
    let payload = EmptyPayload;
    let serialized = serde_json::to_string(&payload).expect("Serialization failed");
    assert_eq!(serialized, "{}", "EmptyPayload did not serialize into an empty JSON object");
  }
}
