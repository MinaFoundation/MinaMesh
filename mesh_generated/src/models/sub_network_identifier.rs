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

/// SubNetworkIdentifier : In blockchains with sharded state, the SubNetworkIdentifier is required to query some object on a specific shard. This identifier is optional for all non-sharded blockchains.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubNetworkIdentifier {
  #[serde(rename = "network")]
  pub network: String,
  #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
  pub metadata: Option<serde_json::Value>,
}

impl SubNetworkIdentifier {
  /// In blockchains with sharded state, the SubNetworkIdentifier is required to query some object on a specific shard. This identifier is optional for all non-sharded blockchains.
  pub fn new(network: String) -> SubNetworkIdentifier {
    SubNetworkIdentifier {
      network,
      metadata: None,
    }
  }
}
