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

/// MempoolTransactionRequest : A MempoolTransactionRequest is utilized to retrieve a transaction from the mempool. 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MempoolTransactionRequest {
    #[serde(rename = "network_identifier")]
    pub network_identifier: Box<models::NetworkIdentifier>,
    #[serde(rename = "transaction_identifier")]
    pub transaction_identifier: Box<models::TransactionIdentifier>,
}

impl MempoolTransactionRequest {
    /// A MempoolTransactionRequest is utilized to retrieve a transaction from the mempool. 
    pub fn new(network_identifier: models::NetworkIdentifier, transaction_identifier: models::TransactionIdentifier) -> MempoolTransactionRequest {
        MempoolTransactionRequest {
            network_identifier: Box::new(network_identifier),
            transaction_identifier: Box::new(transaction_identifier),
        }
    }
}
