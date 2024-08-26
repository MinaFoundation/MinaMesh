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

/// BlockTransaction : BlockTransaction contains a populated Transaction and the BlockIdentifier that contains it. 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlockTransaction {
    #[serde(rename = "block_identifier")]
    pub block_identifier: Box<models::BlockIdentifier>,
    #[serde(rename = "transaction")]
    pub transaction: Box<models::Transaction>,
}

impl BlockTransaction {
    /// BlockTransaction contains a populated Transaction and the BlockIdentifier that contains it. 
    pub fn new(block_identifier: models::BlockIdentifier, transaction: models::Transaction) -> BlockTransaction {
        BlockTransaction {
            block_identifier: Box::new(block_identifier),
            transaction: Box::new(transaction),
        }
    }
}
