use mina_signer::{BaseField, CompressedPubKey};
use o1_utils::FieldHelpers;
use sha2::Digest;

use crate::MinaMeshError;

/// https://github.com/MinaProtocol/mina/blob/985eda49bdfabc046ef9001d3c406e688bc7ec45/src/lib/base58_check/base58_check.ml
///
/// Validates a base58-encoded string with checksum and optional version byte.
///
/// # Arguments
/// * `input` - The base58-encoded string to validate.
/// * `expected_version` - An optional expected version byte for validation.
///
/// # Returns
/// * `Ok(())` if the input is valid.
/// * `Err(MinaMeshError)` if the input is invalid.
pub fn validate_base58_with_checksum(input: &str, expected_version: Option<u8>) -> Result<(), MinaMeshError> {
  // Decode the input using base58
  let bytes = bs58::decode(input)
    .with_alphabet(bs58::Alphabet::BITCOIN)
    .into_vec()
    .map_err(|_| MinaMeshError::MalformedPublicKey("Input not valid base58".to_string()))?;

  // Check the length (must include at least version and checksum)
  if bytes.len() < 5 {
    return Err(MinaMeshError::MalformedPublicKey("Input too short".to_string()));
  }

  // Split into version, payload, and checksum
  let (version, rest) = bytes.split_at(1);
  let (payload, checksum) = rest.split_at(rest.len() - 4);

  // Validate version byte if specified
  if let Some(expected) = expected_version {
    if version[0] != expected {
      return Err(MinaMeshError::MalformedPublicKey(format!(
        "Unexpected version byte: expected {}, got {}",
        expected, version[0]
      )));
    }
  }

  // Recompute checksum
  let computed_checksum = sha2::Sha256::digest(sha2::Sha256::digest([version, payload].concat()));
  if &computed_checksum[.. 4] != checksum {
    return Err(MinaMeshError::MalformedPublicKey("Checksum mismatch".to_string()));
  }

  Ok(())
}

/// Converts a hex string into a compressed public key
/// https://github.com/MinaProtocol/mina/blob/985eda49bdfabc046ef9001d3c406e688bc7ec45/src/lib/rosetta_coding/coding.ml#L128
pub fn hex_to_compressed_pub_key(hex: &str) -> Result<CompressedPubKey, MinaMeshError> {
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
  let x =
    BaseField::from_bits(&bits).map_err(|_| MinaMeshError::MalformedPublicKey("Invalid x-coordinate".to_string()))?;

  // Construct the compressed public key
  Ok(CompressedPubKey { x, is_odd })
}

pub fn address_to_compressed_pub_key(context: &str, address: &str) -> Result<CompressedPubKey, MinaMeshError> {
  CompressedPubKey::from_address(address).map_err(|_| MinaMeshError::MalformedPublicKey(context.to_string()))
}
