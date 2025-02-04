use anyhow::Result;
use coinbase_mesh::models::{ConstructionPayloadsRequest, ConstructionPayloadsResponse};

use crate::{MinaMesh, MinaMeshError, PartialUserCommand, PreprocessMetadata, TransactionMetadata};

/// https://github.com/MinaProtocol/mina/blob/985eda49bdfabc046ef9001d3c406e688bc7ec45/src/app/rosetta/lib/construction.ml#L473
impl MinaMesh {
  pub async fn construction_payloads(
    &self,
    request: ConstructionPayloadsRequest,
  ) -> Result<ConstructionPayloadsResponse> {
    self.validate_network(&request.network_identifier).await?;

    let metadata: TransactionMetadata = request
      .metadata
      .ok_or(MinaMeshError::JsonParse(Some("Metadata is required for payloads request".to_string())))?
      .try_into()?;

    let partial_user_command =
      PartialUserCommand::from_operations(&request.operations, metadata.valid_until, metadata.memo)?;

    Ok(ConstructionPayloadsResponse::new("".to_string(), vec![]))
  }
}
