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

/// NetworkOptionsResponse : NetworkOptionsResponse contains information about the versioning of the node and the allowed operation statuses, operation types, and errors. 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkOptionsResponse {
    #[serde(rename = "version")]
    pub version: Box<models::Version>,
    #[serde(rename = "allow")]
    pub allow: Box<models::Allow>,
}

impl NetworkOptionsResponse {
    /// NetworkOptionsResponse contains information about the versioning of the node and the allowed operation statuses, operation types, and errors. 
    pub fn new(version: models::Version, allow: models::Allow) -> NetworkOptionsResponse {
        NetworkOptionsResponse {
            version: Box::new(version),
            allow: Box::new(allow),
        }
    }
}
