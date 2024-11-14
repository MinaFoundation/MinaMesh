use coinbase_mesh::models::{AccountIdentifier, Amount, Currency, Operation, OperationIdentifier};
use convert_case::{Case, Casing};
use serde_json::json;

use crate::{util::DEFAULT_TOKEN_ID, OperationStatus, OperationType, Token, TransactionStatus};

pub fn operation(
  ident: i64,
  amount: Option<&String>,
  account: &AccountIdentifier,
  operation_type: OperationType,
  status: Option<&TransactionStatus>,
  related_operations: Option<Vec<i64>>,
  metadata: Option<&serde_json::Value>,
  token: Option<&Token>,
) -> Operation {
  // Determine the currency symbol and metadata based on the token
  let currency = match token {
    // If a custom token is provided and is not the default, use its symbol and id
    Some(token) if token.id != DEFAULT_TOKEN_ID => {
      if token.symbol.is_some() {
        Currency {
          symbol: token.symbol.to_owned().unwrap(),
          decimals: 9,
          metadata: Some(json!({ "token_id": token.id })),
        }
      } else {
        Currency { symbol: "MINA+".to_owned(), decimals: 9, metadata: Some(json!({ "token_id": token.id })) }
      }
    }
    // Default case: Use "MINA" as the currency
    _ => Currency::new("MINA".to_owned(), 9),
  };

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
