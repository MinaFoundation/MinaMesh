use coinbase_mesh::models::{AccountIdentifier, Amount, Currency, Operation, OperationIdentifier};
use convert_case::{Case, Casing};

use crate::{OperationStatus, OperationType, TransactionStatus};

pub fn operation(
  ident: i64,
  amount: Option<&String>,
  account: &AccountIdentifier,
  operation_type: OperationType,
  status: Option<&TransactionStatus>,
  related_operations: Option<Vec<i64>>,
  metadata: Option<&serde_json::Value>,
) -> Operation {
  Operation {
    operation_identifier: Box::new(OperationIdentifier::new(ident)),
    amount: amount.map(|value| Box::new(Amount::new(value.to_owned(), Currency::new("MINA".to_string(), 9)))),
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
