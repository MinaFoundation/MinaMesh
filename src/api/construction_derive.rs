use anyhow::Result;
use coinbase_mesh::models::{AccountIdentifier, ConstructionDeriveRequest, ConstructionDeriveResponse};
use mina_signer::{BaseField, CompressedPubKey};
use o1_utils::FieldHelpers;
use serde_json::{json, Value};
use sha2::Digest;

use crate::{util::DEFAULT_TOKEN_ID, MinaMesh, MinaMeshError};

/// https://github.com/MinaProtocol/mina/blob/985eda49bdfabc046ef9001d3c406e688bc7ec45/src/app/rosetta/lib/construction.ml#L162
impl MinaMesh {
  pub async fn construction_derive(
    &self,
    request: ConstructionDeriveRequest,
  ) -> Result<ConstructionDeriveResponse, MinaMeshError> {
    // Validate the network identifier
    self.validate_network(&request.network_identifier).await?;

    // Decode the hex_bytes payload into an address
    let compressed_pk = to_public_key_compressed(&request.public_key.hex_bytes)?;
    let address = compressed_pk.into_address();

    // Decode the token ID from metadata (if present)
    let token_id = decode_token_id(request.metadata)?;

    // Construct the account identifier
    let account_identifier = AccountIdentifier {
      address: address.clone(),
      sub_account: None,
      metadata: Some(json!({ "token_id": token_id })),
    };

    // Build the response
    Ok(ConstructionDeriveResponse {
      address: None,
      account_identifier: Some(Box::new(account_identifier)),
      metadata: None,
    })
  }
}

/// Converts a hex string into a compressed public key
/// https://github.com/MinaProtocol/mina/blob/985eda49bdfabc046ef9001d3c406e688bc7ec45/src/lib/rosetta_coding/coding.ml#L128
fn to_public_key_compressed(hex: &str) -> Result<CompressedPubKey, MinaMeshError> {
  if hex.len() != 64 {
    return Err(MinaMeshError::MalformedPublicKey("Invalid length for hex".to_string()));
  }

  // Decode the hex string
  let mut bytes =
    hex::decode(hex).map_err(|_| MinaMeshError::MalformedPublicKey("Invalid hex encoding".to_string()))?;
  // Reverse the bytes
  bytes.reverse();

  // Convert bytes to bits
  let mut bits: Vec<bool> = bytes.iter().flat_map(|byte| (0 .. 8).rev().map(move |i| (byte >> i) & 1 == 1)).collect();

  // Extract the `is_odd` bit
  let is_odd = bits.remove(0);

  // Reverse the remaining bits
  bits.reverse();

  // Create the x-coordinate as a BaseField element
  let x = BaseField::from_bits(&bits)
    .or_else(|_| Err(MinaMeshError::MalformedPublicKey("Invalid x-coordinate".to_string())))?;

  // Construct the compressed public key
  Ok(CompressedPubKey { x, is_odd })
}

/// Decodes the token ID from metadata, or returns a default value if not
/// present
fn decode_token_id(metadata: Option<Value>) -> Result<String, MinaMeshError> {
  if let Some(meta) = metadata {
    if let Some(token_id) = meta.get("token_id") {
      let token_id_str =
        token_id.as_str().ok_or_else(|| MinaMeshError::MalformedPublicKey("Invalid token_id format".to_string()))?;

      // Validate the token ID format (e.g., base58)
      validate_base58_token_id(token_id_str)?;

      return Ok(token_id_str.to_string());
    }
  }
  // Default token ID if not present
  Ok(DEFAULT_TOKEN_ID.to_string())
}

/// Validates the token ID format (base58 and checksum)
fn validate_base58_token_id(token_id: &str) -> Result<(), MinaMeshError> {
  // Decode the token ID using base58
  let bytes = bs58::decode(token_id)
    .with_alphabet(bs58::Alphabet::BITCOIN)
    .into_vec()
    .map_err(|_| MinaMeshError::MalformedPublicKey("Token_id not valid base58".to_string()))?;

  // Check the length (e.g., must include version and checksum)
  if bytes.len() < 5 {
    return Err(MinaMeshError::MalformedPublicKey("Token_id too short".to_string()));
  }

  // Split into payload and checksum
  let (payload, checksum) = bytes.split_at(bytes.len() - 4);

  // Recompute checksum
  let computed_checksum = sha2::Sha256::digest(&sha2::Sha256::digest(payload));
  if &computed_checksum[.. 4] != checksum {
    return Err(MinaMeshError::MalformedPublicKey("Token_id checksum mismatch".to_string()));
  }

  Ok(())
}
