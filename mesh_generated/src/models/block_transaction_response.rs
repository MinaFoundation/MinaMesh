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

/// BlockTransactionResponse : A BlockTransactionResponse contains information about a block transaction.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlockTransactionResponse {
  #[serde(rename = "transaction")]
  pub transaction: Box<models::Transaction>,
}

impl BlockTransactionResponse {
  /// A BlockTransactionResponse contains information about a block transaction.
  pub fn new(transaction: models::Transaction) -> BlockTransactionResponse {
    BlockTransactionResponse {
      transaction: Box::new(transaction),
    }
  }
}
