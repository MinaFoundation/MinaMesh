use anyhow::Result;
use coinbase_mesh::models::{
  AccountIdentifier, ConstructionPayloadsRequest, ConstructionPayloadsResponse, SignatureType, SigningPayload,
};
use mina_signer::CompressedPubKey;
use serde_json::json;

use crate::{MinaMesh, MinaMeshError, PartialUserCommand, TransactionMetadata, TransactionUnsigned};

/// https://github.com/MinaProtocol/mina/blob/985eda49bdfabc046ef9001d3c406e688bc7ec45/src/app/rosetta/lib/construction.ml#L473
impl MinaMesh {
  pub async fn construction_payloads(
    &self,
    request: ConstructionPayloadsRequest,
  ) -> Result<ConstructionPayloadsResponse, MinaMeshError> {
    self.validate_network(&request.network_identifier).await?;

    // Extract metadata from the request
    let metadata: TransactionMetadata = request
      .metadata
      .ok_or(MinaMeshError::JsonParse(Some("Metadata is required for payloads request".to_string())))?
      .try_into()?;

    // Create a partial user command from the operations
    let partial_user_command =
      PartialUserCommand::from_operations(&request.operations, metadata.valid_until, metadata.memo)?;

    // Ensure the source public key is valid
    let _ = CompressedPubKey::from_address(&partial_user_command.source)
      .map_err(|e| MinaMeshError::MalformedPublicKey(format!("Invalid source public key: {}", e)))?;

    let nonce_u32 = metadata
      .nonce
      .parse::<u32>()
      .map_err(|_| MinaMeshError::JsonParse(Some(format!("Invalid nonce: {}", metadata.nonce))))?;
    let user_command_payload = partial_user_command.to_user_command_payload(nonce_u32)?;

    // Convert the user command payload to an unsigned transaction
    let unsigned_transaction: TransactionUnsigned = (&user_command_payload).into();

    // Serialize to JSON string
    let unsigned_transaction_json = unsigned_transaction.as_json_string()?;

    // Construct the signing payload
    let signing_payload = SigningPayload {
      account_identifier: Some(
        AccountIdentifier {
          address: partial_user_command.source.clone(),
          metadata: Some(json!({ "token_id": partial_user_command.token.clone() })),
          sub_account: None,
        }
        .into(),
      ),
      hex_bytes: hex::encode(unsigned_transaction_json.clone()).to_uppercase(),
      signature_type: Some(SignatureType::SchnorrPoseidon),
      address: None,
    };

    Ok(ConstructionPayloadsResponse::new(unsigned_transaction_json, vec![signing_payload]))
  }
}
