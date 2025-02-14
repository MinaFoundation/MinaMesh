use anyhow::Result;
use coinbase_mesh::models::{Amount, ConstructionMetadataRequest, ConstructionMetadataResponse};
use cynic::QueryBuilder;
use serde_json::{json, Value};

use crate::{
  create_currency,
  graphql::{Block3, PublicKey, QueryConstructionMetadata, QueryConstructionMetadataVariables, TokenId},
  signer_utils::validate_base58_with_checksum,
  util::{DEFAULT_TOKEN_ID, MINIMUM_USER_COMMAND_FEE},
  MinaMesh, MinaMeshError, TransactionMetadata,
};

/// https://github.com/MinaProtocol/mina/blob/985eda49bdfabc046ef9001d3c406e688bc7ec45/src/app/rosetta/lib/construction.ml#L133
impl MinaMesh {
  pub async fn construction_metadata(
    &self,
    request: ConstructionMetadataRequest,
  ) -> Result<ConstructionMetadataResponse, MinaMeshError> {
    // Validate network
    self.validate_network(&request.network_identifier).await?;

    // Extract sender, receiver, and token_id from options
    let options = request.options.as_ref().ok_or(MinaMeshError::NoOptionsProvided)?;

    let sender = self.get_field_from_options(options, "sender")?;
    validate_base58_with_checksum(sender, None)
      .map_err(|e| MinaMeshError::JsonParse(Some(format!("Sender key not valid: {}", e))))?;

    let receiver = self.get_field_from_options(options, "receiver")?;
    validate_base58_with_checksum(receiver, None)
      .map_err(|e| MinaMeshError::JsonParse(Some(format!("Receiver key not valid: {}", e))))?;

    let token_id = self.get_field_from_options(options, "token_id")?;

    // Send GraphQL query
    let query_variables = QueryConstructionMetadataVariables {
      sender: PublicKey(sender.to_string()),
      // for now, nonce is based on the fee payer's account using the default token ID
      // https://github.com/MinaProtocol/mina/blob/985eda49bdfabc046ef9001d3c406e688bc7ec45/src/app/rosetta/lib/construction.ml#L239
      token_id: Some(TokenId(DEFAULT_TOKEN_ID.to_string())),
      receiver_key: PublicKey(receiver.to_string()),
    };
    let query = QueryConstructionMetadata::build(query_variables);
    let response = self.graphql_client.send(query).await?;

    // Extract inferred nonce from sender
    let inferred_nonce = response
      .sender
      .ok_or(MinaMeshError::AccountNotFound(format!("Sender account not found: {}", sender)))?
      .inferred_nonce
      .map(|n| n.0)
      .unwrap_or("0".to_string()); // Default to 0 if missing;

    // Extract account creation fee
    let account_creation_fee_value = response.genesis_constants.account_creation_fee;

    // Calculate suggested fee from best_chain
    let best_chain = response.best_chain.ok_or(MinaMeshError::ChainInfoMissing)?;
    let suggested_fee = self.suggested_fee(best_chain).unwrap_or(MINIMUM_USER_COMMAND_FEE);

    // Construct metadata
    let account_creation_fee = response.receiver.is_none().then_some(account_creation_fee_value.0);
    let valid_until = options.get("valid_until").and_then(|v| v.as_str());
    let memo = options.get("memo").and_then(|v| v.as_str());
    let metadata =
      TransactionMetadata::new(sender, receiver, inferred_nonce, token_id, account_creation_fee, valid_until, memo);

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

    Ok(ConstructionMetadataResponse { metadata: metadata.to_json(), suggested_fee: Some(vec![suggested_fee_entry]) })
  }

  fn get_field_from_options<'a>(&self, options: &'a Value, field: &'a str) -> Result<&'a str, MinaMeshError> {
    options
      .get(field)
      .and_then(|v| v.as_str())
      .ok_or_else(|| MinaMeshError::JsonParse(format!("Field `{}` missing", field).into()))
  }

  // Calculate suggested fee (median + IQR/2)
  // https://github.com/MinaProtocol/mina/blob/985eda49bdfabc046ef9001d3c406e688bc7ec45/src/app/rosetta/lib/construction.ml#L275
  fn suggested_fee(&self, blocks: Vec<Block3>) -> Option<u64> {
    let mut fees: Vec<u64> = blocks
      .iter()
      .flat_map(|block| block.transactions.user_commands.iter().map(|cmd| cmd.fee.to_u64()))
      .flatten()
      .collect();

    if fees.is_empty() {
      None
    } else {
      fees.sort_unstable();

      let len = fees.len();
      let median = fees[len / 2];
      let q3 = fees[(3 * len) / 4];
      let q1 = fees[len / 4];
      let iqr = q3.saturating_sub(q1); // Ensure no underflow
      let suggested = median + (iqr / 2);

      Some(suggested)
    }
  }
}
