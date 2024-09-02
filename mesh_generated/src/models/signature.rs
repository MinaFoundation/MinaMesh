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

/// Signature : Signature contains the payload that was signed, the public keys of the keypairs used to produce the signature, the signature (encoded in hex), and the SignatureType.  PublicKey is often times not known during construction of the signing payloads but may be needed to combine signatures properly.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Signature {
  #[serde(rename = "signing_payload")]
  pub signing_payload: Box<models::SigningPayload>,
  #[serde(rename = "public_key")]
  pub public_key: Box<models::PublicKey>,
  #[serde(rename = "signature_type")]
  pub signature_type: models::SignatureType,
  #[serde(rename = "hex_bytes")]
  pub hex_bytes: String,
}

impl Signature {
  /// Signature contains the payload that was signed, the public keys of the keypairs used to produce the signature, the signature (encoded in hex), and the SignatureType.  PublicKey is often times not known during construction of the signing payloads but may be needed to combine signatures properly.
  pub fn new(
    signing_payload: models::SigningPayload,
    public_key: models::PublicKey,
    signature_type: models::SignatureType,
    hex_bytes: String,
  ) -> Signature {
    Signature {
      signing_payload: Box::new(signing_payload),
      public_key: Box::new(public_key),
      signature_type,
      hex_bytes,
    }
  }
}
