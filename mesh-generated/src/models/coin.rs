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

/// Coin : Coin contains its unique identifier and the amount it represents. 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Coin {
    #[serde(rename = "coin_identifier")]
    pub coin_identifier: Box<models::CoinIdentifier>,
    #[serde(rename = "amount")]
    pub amount: Box<models::Amount>,
}

impl Coin {
    /// Coin contains its unique identifier and the amount it represents. 
    pub fn new(coin_identifier: models::CoinIdentifier, amount: models::Amount) -> Coin {
        Coin {
            coin_identifier: Box::new(coin_identifier),
            amount: Box::new(amount),
        }
    }
}

