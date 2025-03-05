use anyhow::Result;
use coinbase_mesh::models::{AccountIdentifier, ConstructionParseRequest, ConstructionParseResponse, Operation};
use serde_json::{json, Value};

use crate::{
  generate_operations_user_command,
  signer_utils::decode_signature,
  util::{DEFAULT_TOKEN_ID, MINIMUM_USER_COMMAND_FEE},
  HasPaymentAndDelegation, MinaMesh, MinaMeshError, PartialUserCommand, TransactionSigned, TransactionUnsigned,
  UserCommandPayload,
};

/// https://github.com/MinaProtocol/mina/blob/985eda49bdfabc046ef9001d3c406e688bc7ec45/src/app/rosetta/lib/construction.ml#L615
impl MinaMesh {
  pub async fn construction_parse(
    &self,
    request: ConstructionParseRequest,
  ) -> Result<ConstructionParseResponse, MinaMeshError> {
    self.validate_network(&request.network_identifier).await?;

    let (mut operations, metadata, account_identifier) = if request.signed {
      // Parse signed transaction
      let tx = TransactionSigned::from_json_string(&request.transaction)?;
      if tx.payment.is_some() && tx.stake_delegation.is_some() {
        return Err(MinaMeshError::JsonParse(Some(
          "Signed transaction must have one of: payment, stake_delegation".to_string(),
        )));
      }

      decode_signature(&tx.signature)?;

      if tx.payment.is_some() {
        self.check_fee(tx.payment.as_ref().unwrap().fee)?;
        let metadata =
          self.make_metadata(tx.payment.as_ref().unwrap().memo.clone(), tx.payment.as_ref().unwrap().valid_until);
        let account_identifier = self.make_account_identifier(
          tx.payment.as_ref().unwrap().from.clone(),
          tx.payment.as_ref().unwrap().token.clone(),
        );
        let operations = generate_operations_user_command(tx.payment.as_ref().unwrap());
        self.validate_operations(&tx, &operations, &metadata)?;
        (operations, metadata, Some(account_identifier))
      } else {
        self.check_fee(tx.stake_delegation.as_ref().unwrap().fee)?;
        let metadata = self.make_metadata(
          tx.stake_delegation.as_ref().unwrap().memo.clone(),
          tx.stake_delegation.as_ref().unwrap().valid_until,
        );
        let account_identifier = self.make_account_identifier(
          tx.stake_delegation.as_ref().unwrap().delegator.clone(),
          DEFAULT_TOKEN_ID.to_string(),
        );
        let operations = generate_operations_user_command(tx.stake_delegation.as_ref().unwrap());
        self.validate_operations(&tx, &operations, &metadata)?;
        (operations, metadata, Some(account_identifier))
      }
    } else {
      // Parse unsigned transaction
      let tx = TransactionUnsigned::from_json_string(&request.transaction)?;
      if tx.payment.is_some() && tx.stake_delegation.is_some() {
        return Err(MinaMeshError::JsonParse(Some(
          "Unsigned transaction must have one of: payment, stake_delegation".to_string(),
        )));
      }
      if tx.payment.is_some() {
        self.check_fee(tx.payment.as_ref().unwrap().fee)?;
        let metadata =
          self.make_metadata(tx.payment.as_ref().unwrap().memo.clone(), tx.payment.as_ref().unwrap().valid_until);
        let operations = generate_operations_user_command(tx.payment.as_ref().unwrap());
        self.validate_unsigned_transaction(&tx, &operations, &metadata)?;
        (operations, metadata, None)
      } else {
        self.check_fee(tx.stake_delegation.as_ref().unwrap().fee)?;
        let metadata = self.make_metadata(
          tx.stake_delegation.as_ref().unwrap().memo.clone(),
          tx.stake_delegation.as_ref().unwrap().valid_until,
        );
        let operations = generate_operations_user_command(tx.stake_delegation.as_ref().unwrap());
        self.validate_unsigned_transaction(&tx, &operations, &metadata)?;
        (operations, metadata, None)
      }
    };

    // Set status None for all operations
    for operation in operations.iter_mut() {
      operation.status = None;
    }

    Ok(ConstructionParseResponse {
      operations,
      signers: None,
      account_identifier_signers: if account_identifier.is_some() {
        Some(vec![account_identifier.unwrap()])
      } else {
        None
      },
      metadata,
    })
  }

  fn check_fee(&self, fee: u64) -> Result<(), MinaMeshError> {
    if fee < MINIMUM_USER_COMMAND_FEE {
      return Err(MinaMeshError::TransactionSubmitFeeSmall("Fee must be at least 0.001".to_string()));
    }
    Ok(())
  }

  fn make_metadata(&self, memo: Option<String>, valid_until: Option<u32>) -> Option<Value> {
    if memo.is_none() && valid_until.is_none() {
      return None;
    }

    let mut metadata = json!({});
    if let Some(memo) = memo {
      metadata["memo"] = json!(memo);
    }
    if let Some(valid_until) = valid_until {
      metadata["valid_until"] = json!(valid_until.to_string());
    }
    Some(metadata)
  }

  fn make_account_identifier(&self, address: String, token_id: String) -> AccountIdentifier {
    AccountIdentifier { address, sub_account: None, metadata: Some(json!({ "token_id": token_id })) }
  }

  fn validate_unsigned_transaction(
    &self,
    tx: &TransactionUnsigned,
    operations: &[Operation],
    metadata: &Option<Value>,
  ) -> Result<(), MinaMeshError> {
    let request_tx = tx.clone();

    let user_command_payload = self.validate_operations(tx, operations, metadata)?;
    let unsigned_transaction: TransactionUnsigned = (&user_command_payload).into();
    if request_tx != unsigned_transaction {
      return Err(MinaMeshError::JsonParse(Some(
        "Unsigned transaction does not match operations, randomOracleInput or signerInput".to_string(),
      )));
    }
    Ok(())
  }

  fn validate_operations<T: HasPaymentAndDelegation>(
    &self,
    tx: &T,
    operations: &[Operation],
    metadata: &Option<Value>,
  ) -> Result<UserCommandPayload, MinaMeshError> {
    let (valid_until, memo) = match metadata {
      Some(metadata) => {
        let valid_until = metadata.get("valid_until").and_then(|v| v.as_str()).map(|s| s.to_string());
        let memo = metadata.get("memo").and_then(|v| v.as_str()).map(|s| s.to_string());
        (valid_until, memo)
      }
      None => (None, None),
    };
    let partial_user_command = PartialUserCommand::from_operations(operations, valid_until, memo)?;
    let nonce = self.get_nonce(tx)?;
    partial_user_command.to_user_command_payload(nonce)
  }

  fn get_nonce<T: HasPaymentAndDelegation>(&self, tx: &T) -> Result<u32, MinaMeshError> {
    if tx.payment().is_some() {
      return Ok(tx.payment().as_ref().unwrap().nonce);
    } else if tx.stake_delegation().is_some() {
      return Ok(tx.stake_delegation().as_ref().unwrap().nonce);
    } else {
      return Err(MinaMeshError::TransactionSubmitBadNonce("Nonce is invalid or missing".to_string()));
    }
  }
}
