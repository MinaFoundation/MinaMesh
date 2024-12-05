use coinbase_mesh::models::{AccountIdentifier, Amount, Currency, Operation, OperationIdentifier};
use convert_case::{Case, Casing};
use serde_json::{json, Map, Value};

use crate::{
  util::DEFAULT_TOKEN_ID, InternalCommandOperationsData, InternalCommandType, OperationStatus, OperationType,
  TransactionStatus, UserCommandOperationsData, UserCommandType,
};

/// Creates a `Currency` based on the token provided.
/// If the token is `DEFAULT_TOKEN_ID`, it creates a MINA currency.
/// Otherwise, it creates a MINA+ currency with the token ID in metadata.
pub fn create_currency(token: Option<&String>) -> Currency {
  match token {
    Some(token_id) if token_id != DEFAULT_TOKEN_ID => {
      Currency { symbol: "MINA+".to_owned(), decimals: 9, metadata: Some(json!({ "token_id": token_id })) }
    }
    _ => Currency::new("MINA".to_owned(), 9),
  }
}

#[allow(clippy::too_many_arguments)]
pub fn operation(
  ident: i64,
  amount: Option<&String>,
  account: &AccountIdentifier,
  operation_type: OperationType,
  status: Option<&TransactionStatus>,
  related_operations: Option<Vec<i64>>,
  metadata: Option<&serde_json::Value>,
  token: Option<&String>,
) -> Operation {
  let currency = create_currency(token);
  Operation {
    operation_identifier: Box::new(OperationIdentifier::new(ident)),
    amount: amount.map(|value| Box::new(Amount::new(value.to_owned(), currency))),
    account: Some(Box::new(account.to_owned())),
    status: Some(
      status.map(|item| OperationStatus::from(item.to_owned())).unwrap_or(OperationStatus::Success).to_string(),
    ),
    related_operations: related_operations
      .map(|items| items.iter().map(|item| OperationIdentifier::new(item.to_owned())).collect()),
    coin_change: None,
    r#type: operation_type.to_string().to_case(Case::Snake),
    metadata: metadata.cloned(),
  }
}

pub fn generate_operations_user_command<T: UserCommandOperationsData>(data: &T) -> Vec<Operation> {
  let amt = data.amount().unwrap_or("0").to_string();
  let receiver_account_id = &AccountIdentifier {
    address: data.receiver().to_string(),
    metadata: Some(json!({ "token_id": DEFAULT_TOKEN_ID })),
    sub_account: None,
  };
  let source_account_id = &AccountIdentifier {
    address: data.source().to_string(),
    metadata: Some(json!({ "token_id": DEFAULT_TOKEN_ID })),
    sub_account: None,
  };
  let fee_payer_account_id = &AccountIdentifier {
    address: data.fee_payer().to_string(),
    metadata: Some(json!({ "token_id": DEFAULT_TOKEN_ID })),
    sub_account: None,
  };

  // Construct operations_metadata
  let mut operations_metadata = Map::new();
  if let Some(failure_reason) = data.failure_reason() {
    operations_metadata.insert("reason".to_string(), json!(failure_reason));
  }
  let operations_metadata_value =
    if operations_metadata.is_empty() { None } else { Some(Value::Object(operations_metadata)) };

  let mut operations = Vec::new();
  let mut operation_index = 0;

  // Operation 1: Fee Payment
  operations.push(operation(
    operation_index,
    Some(&format!("-{}", data.fee())),
    fee_payer_account_id,
    OperationType::FeePayment,
    Some(&TransactionStatus::Applied),
    None,
    operations_metadata_value.as_ref(),
    None,
  ));
  operation_index += 1;

  // Operation 2: Account Creation Fee (if applicable)
  if let Some(creation_fee) = data.creation_fee() {
    let negated_creation_fee = format!("-{}", creation_fee);
    operations.push(operation(
      operation_index,
      if data.status() == &TransactionStatus::Applied { Some(&negated_creation_fee) } else { None },
      receiver_account_id,
      OperationType::AccountCreationFeeViaPayment,
      Some(data.status()),
      None,
      operations_metadata_value.as_ref(),
      None,
    ));
    operation_index += 1;
  }

  // Decide on the type of operation based on command type
  match data.command_type() {
    UserCommandType::Payment => {
      let negated_amt = format!("-{}", amt);
      operations.push(operation(
        operation_index,
        if data.status() == &TransactionStatus::Applied { Some(&negated_amt) } else { None },
        source_account_id,
        OperationType::PaymentSourceDec,
        Some(data.status()),
        None,
        operations_metadata_value.as_ref(),
        None,
      ));
      operation_index += 1;

      operations.push(operation(
        operation_index,
        if data.status() == &TransactionStatus::Applied { Some(&amt) } else { None },
        receiver_account_id,
        OperationType::PaymentReceiverInc,
        Some(data.status()),
        Some(vec![operation_index - 1]),
        operations_metadata_value.as_ref(),
        None,
      ));
    }
    UserCommandType::Delegation => {
      operations.push(operation(
        operation_index,
        None,
        source_account_id,
        OperationType::DelegateChange,
        Some(data.status()),
        None,
        Some(&json!({ "delegate_change_target": data.receiver() })),
        None,
      ));
    }
  }

  operations
}

pub fn generate_operations_internal_command<T: InternalCommandOperationsData>(data: &T) -> Vec<Operation> {
  let mut operations = Vec::new();
  let mut operation_index = 0;

  // Receiver Account Identifier
  let receiver_account_id = &AccountIdentifier {
    address: data.receiver().to_string(),
    metadata: Some(json!({ "token_id": DEFAULT_TOKEN_ID })),
    sub_account: None,
  };

  // Handle Account Creation Fee if applicable
  if let Some(creation_fee) = data.creation_fee() {
    operations.push(operation(
      operation_index,
      Some(creation_fee),
      receiver_account_id,
      OperationType::AccountCreationFeeViaFeeReceiver,
      Some(data.status()),
      None,
      None,
      None,
    ));
    operation_index += 1;
  }

  // Process operations based on command type
  match data.command_type() {
    InternalCommandType::Coinbase => {
      operations.push(operation(
        operation_index,
        Some(&data.fee()),
        receiver_account_id,
        OperationType::CoinbaseInc,
        Some(data.status()),
        None,
        None,
        None,
      ));
    }

    InternalCommandType::FeeTransfer => {
      operations.push(operation(
        operation_index,
        Some(&data.fee()),
        receiver_account_id,
        OperationType::FeeReceiverInc,
        Some(data.status()),
        None,
        None,
        None,
      ));
    }

    InternalCommandType::FeeTransferViaCoinbase => {
      if let Some(coinbase_receiver) = data.coinbase_receiver() {
        operations.push(operation(
          operation_index,
          Some(&data.fee()),
          receiver_account_id,
          OperationType::FeeReceiverInc,
          Some(data.status()),
          None,
          None,
          None,
        ));
        operation_index += 1;

        operations.push(operation(
          operation_index,
          Some(&data.fee()),
          &AccountIdentifier::new(coinbase_receiver.to_string()),
          OperationType::FeePayerDec,
          Some(data.status()),
          Some(vec![operation_index - 1]),
          None,
          None,
        ));
      }
    }
  }

  operations
}
