use coinbase_mesh::models::{AccountIdentifier, Amount, Currency, Operation, OperationIdentifier};
use convert_case::{Case, Casing};
use serde_json::json;

use crate::{util::DEFAULT_TOKEN_ID, OperationStatus, OperationType, TransactionStatus};

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
