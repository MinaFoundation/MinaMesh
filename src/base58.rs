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
