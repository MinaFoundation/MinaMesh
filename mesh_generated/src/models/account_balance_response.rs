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
use serde::{Deserialize, Serialize};

/// AccountBalanceResponse : An AccountBalanceResponse is returned on the /account/balance endpoint. If an account has a balance for each AccountIdentifier describing it (ex: an ERC-20 token balance on a few smart contracts), an account balance request must be made with each AccountIdentifier.  The `coins` field was removed and replaced by by `/account/coins` in `v1.4.7`. 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountBalanceResponse {
    #[serde(rename = "block_identifier")]
    pub block_identifier: Box<models::BlockIdentifier>,
    /// A single account may have a balance in multiple currencies. 
    #[serde(rename = "balances")]
    pub balances: Vec<models::Amount>,
    /// Account-based blockchains that utilize a nonce or sequence number should include that number in the metadata. This number could be unique to the identifier or global across the account address. 
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

impl AccountBalanceResponse {
    /// An AccountBalanceResponse is returned on the /account/balance endpoint. If an account has a balance for each AccountIdentifier describing it (ex: an ERC-20 token balance on a few smart contracts), an account balance request must be made with each AccountIdentifier.  The `coins` field was removed and replaced by by `/account/coins` in `v1.4.7`. 
    pub fn new(block_identifier: models::BlockIdentifier, balances: Vec<models::Amount>) -> AccountBalanceResponse {
        AccountBalanceResponse {
            block_identifier: Box::new(block_identifier),
            balances,
            metadata: None,
        }
    }
}
