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

/// TransactionIdentifierResponse : TransactionIdentifierResponse contains the transaction_identifier of a transaction that was submitted to either `/construction/hash` or `/construction/submit`.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionIdentifierResponse {
  #[serde(rename = "transaction_identifier")]
  pub transaction_identifier: Box<models::TransactionIdentifier>,
  #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
  pub metadata: Option<serde_json::Value>,
}

impl TransactionIdentifierResponse {
  /// TransactionIdentifierResponse contains the transaction_identifier of a transaction that was submitted to either `/construction/hash` or `/construction/submit`.
  pub fn new(transaction_identifier: models::TransactionIdentifier) -> TransactionIdentifierResponse {
    TransactionIdentifierResponse {
      transaction_identifier: Box::new(transaction_identifier),
      metadata: None,
    }
  }
}
