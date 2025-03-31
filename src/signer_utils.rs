use mina_signer::{BaseField, CompressedPubKey, Signature};
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
  if &computed_checksum[..4] != checksum {
    return Err(MinaMeshError::MalformedPublicKey("Checksum mismatch".to_string()));
  }

  Ok(())
}

/// Validates a base58-encoded string.
///
/// Less strict than `validate_base58_with_checksum`.
/// It seems that construction/payloads or construction/preprocess
/// (or other that may use coinbase_mesh::models::Operation ->
/// PartialUserCommand) validate tokens with this (or similar)
/// since it allows for tokens like "1" in payload.
pub fn validate_base58(input: &str) -> Result<(), MinaMeshError> {
  bs58::decode(input)
    .with_alphabet(bs58::Alphabet::BITCOIN)
    .into_vec()
    .map_err(|_| MinaMeshError::MalformedPublicKey("Input not valid base58".to_string()))?;
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
  let mut bits: Vec<bool> = bytes.iter().flat_map(|byte| (0..8).rev().map(move |i| (byte >> i) & 1 == 1)).collect();

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

/// Decodes a hex-encoded signature into a `Signature` struct.
/// https://github.com/MinaProtocol/mina/blob/985eda49bdfabc046ef9001d3c406e688bc7ec45/src/lib/mina_base/signature.ml#L62
pub fn decode_signature(signature_raw: &str) -> Result<Signature, MinaMeshError> {
  let bytes =
    hex::decode(signature_raw).map_err(|e| MinaMeshError::SignatureInvalid(format!("Hex decoding failed: {}", e)))?;

  if bytes.len() != 64 {
    return Err(MinaMeshError::SignatureInvalid(format!(
      "Invalid signature length, expected 64 bytes, got {}",
      bytes.len()
    )));
  }

  let (rx_bytes, s_bytes) = bytes.split_at(32);

  let rx = mina_signer::BaseField::from_bytes(rx_bytes)
    .map_err(|_| MinaMeshError::SignatureInvalid("Failed to parse BaseField".to_string()))?;

  let s = mina_signer::ScalarField::from_bytes(s_bytes)
    .map_err(|_| MinaMeshError::SignatureInvalid("Failed to parse ScalarField".to_string()))?;

  Ok(Signature::new(rx, s))
}

#[cfg(test)]
mod tests {
  use hex;
  use mina_signer::ScalarField;

  use super::*;

  #[test]
  fn test_decode_signature_valid() {
    let matrix = vec![
      (
          "CA5B636101409503297B02AB94DE28FACBD53C91919A77E57CCBEDBF9D6C2D1893FDE9B63BF44618270F6D404B7C4B783BB322B05C9347BEF238FDB841BF3320",
          BaseField::from_bytes(&[
              0xCA, 0x5B, 0x63, 0x61, 0x01, 0x40, 0x95, 0x03,
              0x29, 0x7B, 0x02, 0xAB, 0x94, 0xDE, 0x28, 0xFA,
              0xCB, 0xD5, 0x3C, 0x91, 0x91, 0x9A, 0x77, 0xE5,
              0x7C, 0xCB, 0xED, 0xBF, 0x9D, 0x6C, 0x2D, 0x18
          ]).expect("Valid BaseField"),
          ScalarField::from_bytes(&[
              0x93, 0xFD, 0xE9, 0xB6, 0x3B, 0xF4, 0x46, 0x18,
              0x27, 0x0F, 0x6D, 0x40, 0x4B, 0x7C, 0x4B, 0x78,
              0x3B, 0xB3, 0x22, 0xB0, 0x5C, 0x93, 0x47, 0xBE,
              0xF2, 0x38, 0xFD, 0xB8, 0x41, 0xBF, 0x33, 0x20
          ]).expect("Valid ScalarField"),
      ),
      (
          "470460288CDD45957E650194FA727DDFD96869420DF095CEEF614E933F8EF716330075D9A8F3EE459978A236B80B5303B38D13C143F9141C8931AEE058742107",
          BaseField::from_bytes(&[
              0x47, 0x04, 0x60, 0x28, 0x8C, 0xDD, 0x45, 0x95,
              0x7E, 0x65, 0x01, 0x94, 0xFA, 0x72, 0x7D, 0xDF,
              0xD9, 0x68, 0x69, 0x42, 0x0D, 0xF0, 0x95, 0xCE,
              0xEF, 0x61, 0x4E, 0x93, 0x3F, 0x8E, 0xF7, 0x16
          ]).expect("Valid BaseField"),
          ScalarField::from_bytes(&[
              0x33, 0x00, 0x75, 0xD9, 0xA8, 0xF3, 0xEE, 0x45,
              0x99, 0x78, 0xA2, 0x36, 0xB8, 0x0B, 0x53, 0x03,
              0xB3, 0x8D, 0x13, 0xC1, 0x43, 0xF9, 0x14, 0x1C,
              0x89, 0x31, 0xAE, 0xE0, 0x58, 0x74, 0x21, 0x07
          ]).expect("Valid ScalarField"),
      ),
      (
          "006A2A571F3FCCB085410C1B23CC7D254664D0BA697D78A26A203E946BE951048633C79AC64E653FE2B5A404FA5BEC324B1AFA0D174F7DC29D609C058872FD15",
          BaseField::from_bytes(&[
              0x00, 0x6A, 0x2A, 0x57, 0x1F, 0x3F, 0xCC, 0xB0,
              0x85, 0x41, 0x0C, 0x1B, 0x23, 0xCC, 0x7D, 0x25,
              0x46, 0x64, 0xD0, 0xBA, 0x69, 0x7D, 0x78, 0xA2,
              0x6A, 0x20, 0x3E, 0x94, 0x6B, 0xE9, 0x51, 0x04
          ]).expect("Valid BaseField"),
          ScalarField::from_bytes(&[
              0x86, 0x33, 0xC7, 0x9A, 0xC6, 0x4E, 0x65, 0x3F,
              0xE2, 0xB5, 0xA4, 0x04, 0xFA, 0x5B, 0xEC, 0x32,
              0x4B, 0x1A, 0xFA, 0x0D, 0x17, 0x4F, 0x7D, 0xC2,
              0x9D, 0x60, 0x9C, 0x05, 0x88, 0x72, 0xFD, 0x15
          ]).expect("Valid ScalarField"),
      ),
  ];

    for (signature_hex, expected_rx, expected_s) in matrix {
      let result = decode_signature(signature_hex);
      assert!(result.is_ok(), "Expected valid signature decoding for: {}", signature_hex);

      let signature = result.unwrap();
      assert_eq!(
        signature.rx.to_bytes(),
        expected_rx.to_bytes(),
        "Decoded BaseField should match for: {}",
        signature_hex
      );
      assert_eq!(
        signature.s.to_bytes(),
        expected_s.to_bytes(),
        "Decoded ScalarField should match for: {}",
        signature_hex
      );
    }
  }

  #[test]
  fn test_decode_signature_invalid_length() {
    let invalid_hex = "deadbeef"; // Too short
    let result = decode_signature(invalid_hex);
    assert!(matches!(result, Err(MinaMeshError::SignatureInvalid(_))), "Expected invalid signature length error");
  }

  #[test]
  fn test_decode_signature_invalid_length2() {
    let invalid_hex = "AA".repeat(65); // Too long
    let result = decode_signature(&invalid_hex);
    assert!(matches!(result, Err(MinaMeshError::SignatureInvalid(_))), "Expected invalid signature length error");
  }

  #[test]
  fn test_decode_signature_non_hex() {
    let invalid_hex = "ZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZ";
    let result = decode_signature(invalid_hex);
    assert!(matches!(result, Err(MinaMeshError::SignatureInvalid(_))), "Expected hex decoding failure");
  }

  #[test]
  fn test_decode_signature_invalid_field_scalar() {
    let invalid_bytes = vec![0xFF; 64]; // Invalid field values
    let invalid_hex = hex::encode(&invalid_bytes);

    let result = decode_signature(&invalid_hex);
    assert!(matches!(result, Err(MinaMeshError::SignatureInvalid(_))), "Expected invalid field/scalar parsing error");
  }
}
