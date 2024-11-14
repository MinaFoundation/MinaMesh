use derive_more::derive::Display;
use serde::Serialize;
use sqlx::{FromRow, Type};

use crate::util::DEFAULT_TOKEN_ID;

#[derive(Type, Debug, PartialEq, Eq, Serialize)]
#[sqlx(type_name = "chain_status_type", rename_all = "lowercase")]
pub enum ChainStatus {
  Canonical,
  Pending,
  Orphaned,
}

#[derive(Type, Debug, PartialEq, Eq, Serialize)]
#[sqlx(type_name = "command_type", rename_all = "lowercase")]
pub enum UserCommandType {
  Payment,
  Delegation,
}

#[derive(Type, Debug, PartialEq, Eq, Serialize, Display)]
#[sqlx(type_name = "internal_command_type", rename_all = "snake_case")]
pub enum InternalCommandType {
  FeeTransferViaCoinbase,
  FeeTransfer,
  Coinbase,
}

#[derive(Type, Debug, PartialEq, Eq, Serialize, Display, Clone)]
#[sqlx(type_name = "transaction_status", rename_all = "lowercase")]
pub enum TransactionStatus {
  Applied,
  Failed,
}

#[derive(Debug, Display)]
pub enum OperationStatus {
  Success,
  Failed,
}

impl From<TransactionStatus> for OperationStatus {
  fn from(status: TransactionStatus) -> Self {
    match status {
      TransactionStatus::Applied => OperationStatus::Success,
      TransactionStatus::Failed => OperationStatus::Failed,
    }
  }
}

#[derive(Debug, Display)]
pub enum OperationType {
  FeePayerDec,
  FeeReceiverInc,
  CoinbaseInc,
  AccountCreationFeeViaPayment,
  AccountCreationFeeViaFeePayer,
  AccountCreationFeeViaFeeReceiver,
  PaymentSourceDec,
  PaymentReceiverInc,
  FeePayment,
  DelegateChange,
  CreateToken,
  MintTokens,
  ZkappFeePayerDec,
  ZkappBalanceUpdate,
}

#[derive(Type, Debug, PartialEq, Eq, Serialize, Display)]
#[sqlx(type_name = "may_use_token")]
pub enum MayUseToken {
  No,
  ParentsOwnToken,
  InheritFromParent,
}

#[derive(Type, Debug, PartialEq, Eq, Serialize, Display)]
#[sqlx(type_name = "authorization_kind_type")]
pub enum AuthorizationKindType {
  #[sqlx(rename = "None_given")]
  NoneGiven,
  #[sqlx(rename = "Signature")]
  Signature,
  #[sqlx(rename = "Proof")]
  Proof,
}

#[allow(dead_code)]
#[derive(FromRow)]
pub struct ZkAppCommand {
  pub id: Option<i32>,
  pub memo: Option<String>,
  pub hash: String,
  pub fee_payer: String,
  pub pk_update_body: String,
  pub fee: String,
  pub valid_until: Option<i64>,
  pub nonce: Option<i64>,
  pub sequence_no: i32,
  pub status: TransactionStatus,
  pub balance_change: String,
  pub state_hash: Option<String>,
  pub failure_reasons: Option<Vec<String>>,
  pub token: Option<String>,
  pub token_symbol: Option<String>,
  pub height: Option<i64>,
  pub total_count: Option<i64>,
  pub block_id: Option<i32>,
}

#[derive(Clone, Debug)]
pub struct Token {
  pub id: String,
  pub symbol: Option<String>,
}

impl Token {
  pub fn new(id: &String, symbol: &String) -> Self {
    Self { id: id.to_owned(), symbol: Some(symbol.to_owned()) }
  }

  pub fn default() -> Self {
    Self { id: DEFAULT_TOKEN_ID.to_owned(), symbol: Some("MINA".to_owned()) }
  }

  pub fn new_or_default(id: &String, symbol: &String) -> Self {
    if id == &Token::default().id {
      Token::default()
    } else {
      Token::new(id, symbol)
    }
  }

  pub fn new_or_default_opt(id: Option<&String>, symbol: Option<&String>) -> Self {
    match (id, symbol) {
      (Some(id), Some(symbol)) => Token::new_or_default(id, symbol),
      (Some(id), None) => Token { id: id.to_owned(), symbol: None },
      _ => Token::default(),
    }
  }
}
