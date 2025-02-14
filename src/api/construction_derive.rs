use anyhow::Result;
use coinbase_mesh::models::{AccountIdentifier, ConstructionDeriveRequest, ConstructionDeriveResponse};
use serde_json::{json, Value};

use crate::{
  signer_utils::{hex_to_compressed_pub_key, validate_base58_with_checksum},
  util::DEFAULT_TOKEN_ID,
  MinaMesh, MinaMeshError,
};

/// https://github.com/MinaProtocol/mina/blob/985eda49bdfabc046ef9001d3c406e688bc7ec45/src/app/rosetta/lib/construction.ml#L162
impl MinaMesh {
  pub async fn construction_derive(
    &self,
    request: ConstructionDeriveRequest,
  ) -> Result<ConstructionDeriveResponse, MinaMeshError> {
    // Validate the network identifier
    self.validate_network(&request.network_identifier).await?;

    // Decode the hex_bytes payload into an address
    let compressed_pk = hex_to_compressed_pub_key(&request.public_key.hex_bytes)?;
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
  validate_base58_with_checksum(token_id, None)
}
