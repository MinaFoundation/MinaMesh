use anyhow::Result;
use coinbase_mesh::models::{ConstructionPayloadsRequest, ConstructionPayloadsResponse};
use mina_signer::CompressedPubKey;

use crate::{MinaMesh, MinaMeshError, PartialUserCommand, TransactionMetadata};

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

    // Ensure the source public key is valid and decompressible
    let _ = CompressedPubKey::from_address(&partial_user_command.source)
      .map_err(|e| MinaMeshError::MalformedPublicKey(format!("Invalid source public key: {}", e.to_string())))?;

    Ok(ConstructionPayloadsResponse::new("".to_string(), vec![]))
  }
}
