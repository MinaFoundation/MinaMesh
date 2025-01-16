use anyhow::Result;
use coinbase_mesh::models::{ConstructionPreprocessRequest, ConstructionPreprocessResponse, Operation};
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};

use crate::{
  base58::validate_base58_with_checksum,
  util::DEFAULT_TOKEN_ID,
  MinaMesh, MinaMeshError,
  OperationType::{self, *},
  PartialReason, UserCommandType,
};

impl MinaMesh {
  pub async fn construction_preprocess(
    &self,
    request: ConstructionPreprocessRequest,
  ) -> Result<ConstructionPreprocessResponse, MinaMeshError> {
    self.validate_network(&request.network_identifier).await?;

    let metadata = PreprocessMetadata::from_json(request.metadata)?;
    let partial_command = PartialUserCommand::from_operations(&request.operations, metadata)?;

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

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PreprocessMetadata {
  valid_until: Option<String>,
  memo: Option<String>,
}

impl PreprocessMetadata {
  fn from_json(metadata: Option<Value>) -> Result<Option<Self>, MinaMeshError> {
    if let Some(meta) = metadata {
      serde_json::from_value(meta)
        .map(Some)
        .map_err(|e| MinaMeshError::JsonParse(Some(format!("Failed to parse metadata: {}", e))))
    } else {
      Ok(None)
    }
  }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct PartialUserCommand {
  pub kind: UserCommandType,
  pub fee_payer: String,
  pub source: String,
  pub receiver: String,
  pub fee_token: String,
  pub token: String,
  pub fee: i64,
  pub amount: Option<String>,
  pub valid_until: Option<String>,
  pub memo: Option<String>,
}

impl PartialUserCommand {
  pub fn from_operations(
    operations: &[Operation],
    metadata: Option<PreprocessMetadata>,
  ) -> Result<Self, MinaMeshError> {
    let mut errors = Vec::new();
    let valid_until;
    let memo;

    if let Some(metadata) = metadata {
      valid_until = metadata.valid_until;
      memo = metadata.memo;
    } else {
      valid_until = None;
      memo = None;
    }

    if operations.len() == 3 {
      match Self::parse_payment_operations(operations, valid_until, memo) {
        Ok(cmd) => return Ok(cmd),
        Err(MinaMeshError::OperationsNotValid(reasons)) => {
          errors.extend(reasons);
        }
        _ => {}
      }
    } else if operations.len() == 2 {
      match Self::parse_delegation_operations(operations, valid_until, memo) {
        Ok(cmd) => return Ok(cmd),
        Err(MinaMeshError::OperationsNotValid(reasons)) => {
          errors.extend(reasons);
        }
        _ => {}
      }
    } else {
      errors.push(PartialReason::LengthMismatch(format!(
        "Expected 2 operations for delegation or 3 operations for payment, got {}",
        operations.len()
      )));
    }

    if !errors.is_empty() {
      Err(MinaMeshError::OperationsNotValid(errors))
    } else {
      Err(MinaMeshError::OperationsNotValid(vec![PartialReason::CanNotFindKind("Unknown".to_string())]))
    }
  }

  fn parse_payment_operations(
    operations: &[Operation],
    valid_until: Option<String>,
    memo: Option<String>,
  ) -> Result<Self, MinaMeshError> {
    let mut errors = Vec::new();

    let fee_payment = Self::find_operation(operations, FeePayment).map_err(|e| {
      errors.push(PartialReason::CanNotFindKind(FeePayment.to_string()));
      e
    });

    let source_dec = Self::find_operation(operations, PaymentSourceDec).map_err(|e| {
      errors.push(PartialReason::CanNotFindKind(PaymentSourceDec.to_string()));
      e
    });

    let receiver_inc = Self::find_operation(operations, PaymentReceiverInc).map_err(|e| {
      errors.push(PartialReason::CanNotFindKind(PaymentReceiverInc.to_string()));
      e
    });

    if errors.len() > 0 {
      return Err(MinaMeshError::OperationsNotValid(errors));
    }

    let fee_payment = fee_payment.unwrap();
    let source_dec = source_dec.unwrap();
    let receiver_inc = receiver_inc.unwrap();

    let fee_token = Self::token_id_from_operation(fee_payment);
    let token = Self::token_id_from_operation(source_dec);

    if fee_payment.account != source_dec.account {
      errors.push(PartialReason::FeePayerAndSourceMismatch);
    }

    let mut fee = 0;
    if let Some(amount) = &fee_payment.amount {
      if let Ok(value) = amount.value.parse::<i64>() {
        if value >= 0 {
          errors.push(PartialReason::FeeNotNegative);
        }
        fee = value
      } else {
        errors.push(PartialReason::AmountNotValid);
      }
    } else {
      errors.push(PartialReason::AmountNotSome);
    }

    if !errors.is_empty() {
      return Err(MinaMeshError::OperationsNotValid(errors));
    }

    Ok(PartialUserCommand {
      kind: UserCommandType::Payment,
      fee_payer: Self::address_from_operation(fee_payment),
      source: Self::address_from_operation(source_dec),
      receiver: Self::address_from_operation(receiver_inc),
      fee_token,
      token,
      fee,
      amount: Self::amount_from_operation(receiver_inc),
      valid_until,
      memo,
    })
  }

  fn parse_delegation_operations(
    operations: &[Operation],
    valid_until: Option<String>,
    memo: Option<String>,
  ) -> Result<Self, MinaMeshError> {
    let mut errors = Vec::new();

    let fee_payment = Self::find_operation(operations, FeePayment).map_err(|e| {
      errors.push(PartialReason::CanNotFindKind(FeePayment.to_string()));
      e
    });

    let delegate_change = Self::find_operation(operations, DelegateChange).map_err(|e| {
      errors.push(PartialReason::CanNotFindKind(DelegateChange.to_string()));
      e
    });

    if errors.len() > 0 {
      return Err(MinaMeshError::OperationsNotValid(errors));
    }

    let fee_payment = fee_payment.unwrap();
    let delegate_change = delegate_change.unwrap();

    let fee_token = Self::token_id_from_operation(fee_payment);
    let token = Self::token_id_from_operation(delegate_change);

    if fee_payment.account != delegate_change.account {
      errors.push(PartialReason::FeePayerAndSourceMismatch);
    }

    let mut fee = 0;
    if let Some(amount) = &fee_payment.amount {
      if let Ok(value) = amount.value.parse::<i64>() {
        if value >= 0 {
          errors.push(PartialReason::FeeNotNegative);
        }
        fee = value;
      } else {
        errors.push(PartialReason::AmountNotValid);
      }
    } else {
      errors.push(PartialReason::AmountNotSome);
    }

    if let Some(metadata) = &delegate_change.metadata {
      // Validate the delegate_change_target is present
      if metadata.get("delegate_change_target").is_none() {
        errors.push(PartialReason::InvalidMetadata(
          "Missing delegate_change_target in delegate_change metadata".to_string(),
        ));
      }
    } else {
      errors.push(PartialReason::InvalidMetadata(
        "Missing delegate_change metadata with delegate_change_target".to_string(),
      ));
    }

    if !errors.is_empty() {
      return Err(MinaMeshError::OperationsNotValid(errors));
    }

    Ok(PartialUserCommand {
      kind: UserCommandType::Delegation,
      fee_payer: Self::address_from_operation(fee_payment),
      source: Self::address_from_operation(fee_payment),
      receiver: Self::address_from_operation(delegate_change),
      fee_token,
      token,
      fee,
      amount: None,
      valid_until,
      memo,
    })
  }

  fn find_operation<'a>(operations: &'a [Operation], op_type: OperationType) -> Result<&'a Operation, MinaMeshError> {
    operations
      .iter()
      .find(|op| op.r#type == op_type.to_string())
      .ok_or_else(|| MinaMeshError::OperationsNotValid(vec![PartialReason::CanNotFindKind(op_type.to_string())]))
  }

  fn token_id_from_operation(operation: &Operation) -> String {
    operation
      .account
      .as_ref()
      .and_then(|account| account.metadata.as_ref())
      .and_then(|meta| meta.get("token_id").and_then(|t| t.as_str()))
      .unwrap_or(DEFAULT_TOKEN_ID)
      .to_string()
  }

  fn address_from_operation(operation: &Operation) -> String {
    if operation.r#type == DelegateChange.to_string() {
      operation
        .metadata
        .as_ref()
        .and_then(|meta| meta.get("delegate_change_target").and_then(|t| t.as_str()))
        .unwrap_or_default()
        .to_string()
    } else {
      operation.account.as_ref().map_or_else(String::new, |acc| acc.address.clone())
    }
  }

  fn amount_from_operation(operation: &Operation) -> Option<String> {
    operation.amount.as_ref().map_or_else(|| None, |amount| Some(amount.value.clone()))
  }
}
