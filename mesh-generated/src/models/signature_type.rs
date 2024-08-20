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

/// SignatureType : SignatureType is the type of a cryptographic signature.  * ecdsa: `r (32-bytes) || s (32-bytes)` - `64 bytes` * ecdsa_recovery: `r (32-bytes) || s (32-bytes) || v (1-byte)` - `65 bytes` * ed25519: `R (32-byte) || s (32-bytes)` - `64 bytes` * schnorr_1: `r (32-bytes) || s (32-bytes)` - `64 bytes`  (schnorr signature implemented by Zilliqa where both `r` and `s` are scalars encoded as `32-bytes` values, most significant byte first.) * schnorr_bip340: `r (32-bytes) || s (32-bytes)` - `64 bytes`  (sig = (bytes(R) || bytes((k + ed) mod n) where `r` is the `X` coordinate of a point `R` whose `Y` coordinate is even, most significant bytes first.) * schnorr_poseidon: `r (32-bytes) || s (32-bytes)` where s = Hash(1st pk || 2nd pk || r) - `64 bytes`  (schnorr signature w/ Poseidon hash function implemented by O(1) Labs where both `r` and `s` are scalars encoded as `32-bytes` values, least significant byte first. https://github.com/CodaProtocol/signer-reference/blob/master/schnorr.ml ) 
/// SignatureType is the type of a cryptographic signature.  * ecdsa: `r (32-bytes) || s (32-bytes)` - `64 bytes` * ecdsa_recovery: `r (32-bytes) || s (32-bytes) || v (1-byte)` - `65 bytes` * ed25519: `R (32-byte) || s (32-bytes)` - `64 bytes` * schnorr_1: `r (32-bytes) || s (32-bytes)` - `64 bytes`  (schnorr signature implemented by Zilliqa where both `r` and `s` are scalars encoded as `32-bytes` values, most significant byte first.) * schnorr_bip340: `r (32-bytes) || s (32-bytes)` - `64 bytes`  (sig = (bytes(R) || bytes((k + ed) mod n) where `r` is the `X` coordinate of a point `R` whose `Y` coordinate is even, most significant bytes first.) * schnorr_poseidon: `r (32-bytes) || s (32-bytes)` where s = Hash(1st pk || 2nd pk || r) - `64 bytes`  (schnorr signature w/ Poseidon hash function implemented by O(1) Labs where both `r` and `s` are scalars encoded as `32-bytes` values, least significant byte first. https://github.com/CodaProtocol/signer-reference/blob/master/schnorr.ml ) 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SignatureType {
    #[serde(rename = "ecdsa")]
    Ecdsa,
    #[serde(rename = "ecdsa_recovery")]
    EcdsaRecovery,
    #[serde(rename = "ed25519")]
    Ed25519,
    #[serde(rename = "schnorr_1")]
    Schnorr1,
    #[serde(rename = "schnorr_bip340")]
    SchnorrBip340,
    #[serde(rename = "schnorr_poseidon")]
    SchnorrPoseidon,

}

impl std::fmt::Display for SignatureType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Ecdsa => write!(f, "ecdsa"),
            Self::EcdsaRecovery => write!(f, "ecdsa_recovery"),
            Self::Ed25519 => write!(f, "ed25519"),
            Self::Schnorr1 => write!(f, "schnorr_1"),
            Self::SchnorrBip340 => write!(f, "schnorr_bip340"),
            Self::SchnorrPoseidon => write!(f, "schnorr_poseidon"),
        }
    }
}

impl Default for SignatureType {
    fn default() -> SignatureType {
        Self::Ecdsa
    }
}
