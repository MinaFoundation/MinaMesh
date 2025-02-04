use anyhow::Result;
use coinbase_mesh::models::{ConstructionPreprocessRequest, ConstructionPreprocessResponse};
use serde_json::{json, Map, Value};

use crate::{base58::validate_base58_with_checksum, MinaMesh, MinaMeshError, PartialUserCommand, PreprocessMetadata};

impl MinaMesh {
  pub async fn construction_preprocess(
    &self,
    request: ConstructionPreprocessRequest,
  ) -> Result<ConstructionPreprocessResponse, MinaMeshError> {
    self.validate_network(&request.network_identifier).await?;

    let metadata = PreprocessMetadata::from_json(request.metadata)?.unwrap_or_default();
    let partial_command =
      PartialUserCommand::from_operations(&request.operations, metadata.valid_until, metadata.memo)?;

    validate_base58_public_key(partial_command.fee_payer.as_str())?;
    validate_base58_public_key(partial_command.source.as_str())?;
    validate_base58_public_key(partial_command.receiver.as_str())?;

    Ok(ConstructionPreprocessResponse {
      options: Some(make_response_options(partial_command)),
      required_public_keys: Some(vec![]),
    })
  }
}

fn make_response_options(partial_command: PartialUserCommand) -> Value {
  let mut options = Map::new();

  options.insert("sender".to_string(), json!(partial_command.fee_payer));
  options.insert("receiver".to_string(), json!(partial_command.receiver));
  options.insert("token_id".to_string(), json!(partial_command.token));

  if let Some(valid_until) = partial_command.valid_until {
    options.insert("valid_until".to_string(), json!(valid_until));
  }

  if let Some(memo) = partial_command.memo {
    options.insert("memo".to_string(), json!(memo));
  }

  json!(options)
}

fn validate_base58_public_key(token_id: &str) -> Result<(), MinaMeshError> {
  validate_base58_with_checksum(token_id, None).map_err(|e| MinaMeshError::PublicKeyFormatNotValid(e.to_string()))
}
