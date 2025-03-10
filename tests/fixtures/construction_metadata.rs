use mina_mesh::{models::ConstructionMetadataRequest, test::network_id};

use super::CompareGroup;

pub fn construction_metadata<'a>() -> CompareGroup<'a> {
  ("/construction/metadata", vec![
    // with account creation fee
    Box::new(ConstructionMetadataRequest {
      network_identifier: network_id().into(),
      options: Some(serde_json::json!({
        // cspell:disable
        "sender": "B62qnuDyj65AQfcZt3MvcwSX7ohLcP1ayNQwu9Zi6YH7JQddJnc8mkz",
        "receiver": "B62qnXy1f75qq8c6HS2Am88Gk6UyvTHK3iSYh4Hb3nD6DS2eS6wZ4or", // receiver is a dummy address
        "token_id": "weihj2SSP7Z96acs56ygP64Te6wauzvWWfAPHKb1gzqem9J4Ne",
        // cspell:enable
        "valid_until": "200000",
        "memo": "test transaction"
      })),
      public_keys: None,
    }),
    // without account creation fee
    Box::new(ConstructionMetadataRequest {
      network_identifier: network_id().into(),
      options: Some(serde_json::json!({
        // cspell:disable
        "sender": "B62qnuDyj65AQfcZt3MvcwSX7ohLcP1ayNQwu9Zi6YH7JQddJnc8mkz",
        "receiver": "B62qjwDWxjf4LtJ4YWJQDdTNPqZ69ZyeCzbpAFKN7EoZzYig5ZRz8JE", // receiver exists
        "token_id": "weihj2SSP7Z96acs56ygP64Te6wauzvWWfAPHKb1gzqem9J4Ne",
        // cspell:enable
        "valid_until": "200000",
        "memo": "test transaction"
      })),
      public_keys: None,
    }),
  ])
}
