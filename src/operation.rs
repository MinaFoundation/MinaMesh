use coinbase_mesh::models::{AccountIdentifier, Amount, Currency, Operation, OperationIdentifier};
use convert_case::{Case, Casing};
use serde_json::json;

use crate::{util::DEFAULT_TOKEN_ID, OperationStatus, OperationType, TransactionStatus};

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
  // if token is provided and different from DEFAULT_TOKEN_ID, then create a new
  // currency with the token else create a new currency with "MINA"
  let currency = token
    .map(|token_id| {
      if token_id != DEFAULT_TOKEN_ID {
        Currency { symbol: "MINA+".to_owned(), decimals: 9, metadata: Some(json!({ "token_id": token_id })) }
      } else {
        Currency::new("MINA".to_owned(), 9)
      }
    })
    .unwrap_or(Currency::new("MINA".to_owned(), 9));

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
