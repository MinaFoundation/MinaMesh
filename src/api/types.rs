use serde::Serialize;
use sqlx::Type;

#[derive(Type, Debug, PartialEq, Eq, Serialize)]
#[sqlx(type_name = "chain_status_type", rename_all = "lowercase")]
pub enum ChainStatus {
  Canonical,
  Pending,
  Orphaned,
}

#[derive(Type, Debug, PartialEq, Eq, Serialize)]
#[sqlx(type_name = "command_type", rename_all = "lowercase")]
pub enum CommandType {
  Payment,
  Delegation,
}

#[derive(Type, Debug, PartialEq, Eq, Serialize)]
#[sqlx(type_name = "transaction_status", rename_all = "lowercase")]
pub enum TransactionStatus {
  Applied,
  Failed,
}

impl ToString for TransactionStatus {
  fn to_string(&self) -> String {
    match self {
      Self::Applied => "Applied",
      Self::Failed => "Failed",
    }
    .to_string()
  }
}
