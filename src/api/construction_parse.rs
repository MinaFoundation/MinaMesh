use anyhow::Result;
use coinbase_mesh::models::{ConstructionParseRequest, ConstructionParseResponse};
use serde_json::{json, Value};

use crate::{
  generate_operations_user_command, signer_utils::decode_signature, util::MINIMUM_USER_COMMAND_FEE, MinaMesh,
  MinaMeshError, TransactionSigned, TransactionUnsigned,
};

/// https://github.com/MinaProtocol/mina/blob/985eda49bdfabc046ef9001d3c406e688bc7ec45/src/app/rosetta/lib/construction.ml#L615
impl MinaMesh {
  pub async fn construction_parse(
    &self,
    request: ConstructionParseRequest,
  ) -> Result<ConstructionParseResponse, MinaMeshError> {
    self.validate_network(&request.network_identifier).await?;

    let (mut operations, metadata) = if request.signed {
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
        (generate_operations_user_command(&tx.payment.unwrap()), metadata)
      } else {
        self.check_fee(tx.stake_delegation.as_ref().unwrap().fee)?;
        let metadata = self.make_metadata(
          tx.stake_delegation.as_ref().unwrap().memo.clone(),
          tx.stake_delegation.as_ref().unwrap().valid_until,
        );
        (generate_operations_user_command(&tx.stake_delegation.unwrap()), metadata)
      }
    } else {
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
        (generate_operations_user_command(&tx.payment.unwrap()), metadata)
      } else {
        self.check_fee(tx.stake_delegation.as_ref().unwrap().fee)?;
        let metadata = self.make_metadata(
          tx.stake_delegation.as_ref().unwrap().memo.clone(),
          tx.stake_delegation.as_ref().unwrap().valid_until,
        );
        (generate_operations_user_command(&tx.stake_delegation.unwrap()), metadata)
      }
    };

    // Set status None for all operations
    for operation in operations.iter_mut() {
      operation.status = None;
    }

    Ok(ConstructionParseResponse { operations, signers: None, account_identifier_signers: None, metadata })
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
}
