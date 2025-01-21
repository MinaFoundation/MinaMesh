use anyhow::Result;
use coinbase_mesh::models::{Amount, ConstructionMetadataRequest, ConstructionMetadataResponse};
use cynic::QueryBuilder;
use serde_json::{json, Map};

use crate::{
  create_currency,
  graphql::{PublicKey, QueryConstructionMetadata, QueryConstructionMetadataVariables, TokenId},
  util::MINIMUM_USER_COMMAND_FEE,
  MinaMesh, MinaMeshError,
};

impl MinaMesh {
  pub async fn construction_metadata(
    &self,
    request: ConstructionMetadataRequest,
  ) -> Result<ConstructionMetadataResponse, MinaMeshError> {
    // Validate network
    self.validate_network(&request.network_identifier).await?;

    // Extract sender, receiver, and token_id from options
    let options =
      request.options.as_ref().ok_or(MinaMeshError::JsonParse("Field `options` missing".to_string().into()))?;

    let sender = options
      .get("sender")
      .and_then(|v| v.as_str())
      .ok_or(MinaMeshError::JsonParse("Field `sender` missing".to_string().into()))?;

    let receiver = options
      .get("receiver")
      .and_then(|v| v.as_str())
      .ok_or(MinaMeshError::JsonParse("Field `receiver` missing".to_string().into()))?;

    let token_id = options
      .get("token_id")
      .and_then(|v| v.as_str())
      .ok_or(MinaMeshError::JsonParse("Field `token_id` missing".to_string().into()))?;

    // Send GraphQL query
    let query_variables = QueryConstructionMetadataVariables {
      sender: PublicKey(sender.to_string()),
      token_id: Some(TokenId(token_id.to_string())),
      receiver_key: PublicKey(receiver.to_string()),
    };
    let query = QueryConstructionMetadata::build(query_variables);
    let response = self.graphql_client.send(query).await?;

    // Extract inferred nonce
    let inferred_nonce = response.sender.and_then(|acc| acc.inferred_nonce.map(|n| n.0));

    // Calculate suggested fee (convert `Fee` to u64)
    let suggested_fee = response
      .best_chain
      .and_then(|blocks| {
        let mut fees: Vec<u64> = blocks
          .iter()
          .flat_map(|block| block.transactions.user_commands.iter().map(|cmd| cmd.fee.to_u64()))
          .flatten()
          .collect();

        if fees.is_empty() {
          None
        } else {
          fees.sort_unstable();
          Some(fees[fees.len() / 2]) // Median fee
        }
      })
      .unwrap_or(MINIMUM_USER_COMMAND_FEE);

    // Construct metadata
    let mut metadata_map = Map::new();
    metadata_map.insert("sender".to_string(), json!(sender));
    metadata_map.insert("nonce".to_string(), json!(inferred_nonce));
    metadata_map.insert("token_id".to_string(), json!(token_id));
    metadata_map.insert("receiver".to_string(), json!(receiver));

    if let Some(valid_until) = options.get("valid_until").and_then(|v| v.as_str()) {
      metadata_map.insert("valid_until".to_string(), json!(valid_until));
    }

    if let Some(memo) = options.get("memo").and_then(|v| v.as_str()) {
      metadata_map.insert("memo".to_string(), json!(memo));
    }

    let metadata = json!(metadata_map);

    // Construct suggested fee
    let suggested_fee_entry = Amount {
      value: suggested_fee.to_string(),
      currency: Box::new(create_currency(None)),
      metadata: Some(json!({
          "minimum_fee": {
              "value": MINIMUM_USER_COMMAND_FEE.to_string(),
              "currency": {
                  "symbol": "MINA",
                  "decimals": 9
              }
          }
      })),
    };

    Ok(ConstructionMetadataResponse { metadata, suggested_fee: Some(vec![suggested_fee_entry]) })
  }
}
