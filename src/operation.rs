use convert_case::{Case, Casing};
use mesh::models::{AccountIdentifier, Amount, Currency, Operation, OperationIdentifier};

use crate::{OperationStatus, OperationType, TransactionStatus};

pub fn operation(
  ident: i64,
  amount: Option<&String>,
  account: &String,
  operation_type: OperationType,
  status: Option<&TransactionStatus>,
) -> Operation {
  Operation {
    operation_identifier: Box::new(OperationIdentifier::new(ident)),
    amount: amount.map(|value| Box::new(Amount::new(value.to_owned(), Currency::new("mina".to_string(), 9)))),
    account: Some(Box::new(AccountIdentifier::new(account.to_owned()))),
    status: Some(
      status.map(|item| OperationStatus::from(item.to_owned())).unwrap_or(OperationStatus::Success).to_string(),
    ),
    related_operations: None,
    coin_change: None,
    r#type: operation_type.to_string().to_case(Case::Snake),
    metadata: None, // TODO: get the correct metadata
  }
}
