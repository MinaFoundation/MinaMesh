/*
 * Rosetta
 *
 * Build Once. Integrate Your Blockchain Everywhere.
 *
 * The version of the OpenAPI document: 1.4.13
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::Deserialize;
use serde::Serialize;

/// ConstructionMetadataResponse : The ConstructionMetadataResponse returns network-specific metadata used for transaction construction.  Optionally, the implementer can return the suggested fee associated with the transaction being constructed. The caller may use this info to adjust the intent of the transaction or to create a transaction with a different account that can pay the suggested fee. Suggested fee is an array in case fee payment must occur in multiple currencies.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConstructionMetadataResponse {
  #[serde(rename = "metadata")]
  pub metadata: serde_json::Value,
  #[serde(rename = "suggested_fee", skip_serializing_if = "Option::is_none")]
  pub suggested_fee: Option<Vec<models::Amount>>,
}

impl ConstructionMetadataResponse {
  /// The ConstructionMetadataResponse returns network-specific metadata used for transaction construction.  Optionally, the implementer can return the suggested fee associated with the transaction being constructed. The caller may use this info to adjust the intent of the transaction or to create a transaction with a different account that can pay the suggested fee. Suggested fee is an array in case fee payment must occur in multiple currencies.
  pub fn new(metadata: serde_json::Value) -> ConstructionMetadataResponse {
    ConstructionMetadataResponse {
      metadata,
      suggested_fee: None,
    }
  }
}
