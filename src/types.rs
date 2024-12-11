use convert_case::{Case, Casing};
use derive_more::derive::Display;
use serde::Serialize;
use sqlx::{FromRow, Type};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

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

#[derive(Debug, Display, EnumIter)]
pub enum OperationType {
  FeePayerDec,
  FeeReceiverInc,
  CoinbaseInc,
  AccountCreationFeeViaPayment,
  AccountCreationFeeViaFeeReceiver,
  PaymentSourceDec,
  PaymentReceiverInc,
  FeePayment,
  DelegateChange,
  ZkappFeePayerDec,
  ZkappBalanceUpdate,
}

pub fn operation_types() -> Vec<String> {
  OperationType::iter().map(|variant| format!("{:?}", variant).to_case(Case::Snake)).collect()
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
  pub pk_update_body: Option<String>,
  pub fee: String,
  pub valid_until: Option<i64>,
  pub nonce: Option<i64>,
  pub sequence_no: i32,
  pub status: TransactionStatus,
  pub balance_change: Option<String>,
  pub state_hash: Option<String>,
  pub failure_reasons: Option<Vec<String>>,
  pub token: Option<String>,
  pub height: Option<i64>,
  pub total_count: Option<i64>,
  pub block_id: Option<i32>,
}

// Used in search transactions
#[derive(Debug, FromRow)]
pub struct UserCommand {
  pub id: Option<i32>,
  pub command_type: UserCommandType,
  pub fee_payer_id: Option<i32>,
  pub source_id: Option<i32>,
  pub receiver_id: Option<i32>,
  pub nonce: i64,
  pub amount: Option<String>,
  pub fee: Option<String>,
  pub valid_until: Option<i64>,
  pub memo: Option<String>,
  pub hash: String,
  pub block_id: Option<i32>,
  pub sequence_no: Option<i32>,
  pub status: TransactionStatus,
  pub failure_reason: Option<String>,
  pub state_hash: Option<String>,
  pub chain_status: Option<ChainStatus>,
  pub height: Option<i64>,
  pub total_count: Option<i64>,
  pub fee_payer: String,
  pub source: String,
  pub receiver: String,
  pub creation_fee: Option<String>,
}

// Used in block
#[derive(Debug, PartialEq, Eq, FromRow, Serialize)]
pub struct UserCommandMetadata {
  pub command_type: UserCommandType,
  pub nonce: i64,
  pub amount: Option<String>,
  pub fee: Option<String>,
  pub valid_until: Option<i64>,
  pub memo: Option<String>,
  pub hash: String,
  pub fee_payer: String,
  pub source: String,
  pub receiver: String,
  pub status: TransactionStatus,
  pub failure_reason: Option<String>,
  pub creation_fee: Option<String>,
}

// Common trait for producing operations from user commands
pub trait UserCommandOperationsData {
  fn command_type(&self) -> &UserCommandType;
  fn fee_payer(&self) -> &str;
  fn source(&self) -> &str;
  fn receiver(&self) -> &str;
  fn nonce(&self) -> i64;
  fn memo(&self) -> Option<String>;
  fn amount(&self) -> Option<&str>;
  fn fee(&self) -> &str;
  fn status(&self) -> &TransactionStatus;
  fn failure_reason(&self) -> Option<&str>;
  fn creation_fee(&self) -> Option<&str>;
}

impl UserCommandOperationsData for UserCommand {
  fn command_type(&self) -> &UserCommandType {
    &self.command_type
  }

  fn fee_payer(&self) -> &str {
    &self.fee_payer
  }

  fn source(&self) -> &str {
    &self.source
  }

  fn receiver(&self) -> &str {
    &self.receiver
  }

  fn nonce(&self) -> i64 {
    self.nonce
  }

  fn memo(&self) -> Option<String> {
    self.memo.clone()
  }

  fn amount(&self) -> Option<&str> {
    self.amount.as_deref()
  }

  fn fee(&self) -> &str {
    self.fee.as_deref().unwrap_or("0")
  }

  fn status(&self) -> &TransactionStatus {
    &self.status
  }

  fn failure_reason(&self) -> Option<&str> {
    self.failure_reason.as_deref()
  }

  fn creation_fee(&self) -> Option<&str> {
    self.creation_fee.as_deref()
  }
}

impl UserCommandOperationsData for UserCommandMetadata {
  fn command_type(&self) -> &UserCommandType {
    &self.command_type
  }

  fn fee_payer(&self) -> &str {
    &self.fee_payer
  }

  fn source(&self) -> &str {
    &self.source
  }

  fn receiver(&self) -> &str {
    &self.receiver
  }

  fn nonce(&self) -> i64 {
    self.nonce
  }

  fn memo(&self) -> Option<String> {
    self.memo.clone()
  }

  fn amount(&self) -> Option<&str> {
    self.amount.as_deref()
  }

  fn fee(&self) -> &str {
    self.fee.as_deref().unwrap_or("0")
  }

  fn status(&self) -> &TransactionStatus {
    &self.status
  }

  fn failure_reason(&self) -> Option<&str> {
    self.failure_reason.as_deref()
  }

  fn creation_fee(&self) -> Option<&str> {
    self.creation_fee.as_deref()
  }
}

// Used in search transactions
#[derive(Debug, FromRow)]
pub struct InternalCommand {
  pub id: Option<i32>,
  pub command_type: InternalCommandType,
  pub receiver_id: Option<i32>,
  pub fee: Option<String>,
  pub hash: String,
  pub receiver: String,
  pub coinbase_receiver: Option<String>,
  pub sequence_no: i32,
  pub secondary_sequence_no: i32,
  pub block_id: i32,
  pub status: TransactionStatus,
  pub state_hash: Option<String>,
  pub height: Option<i64>,
  pub total_count: Option<i64>,
  pub creation_fee: Option<String>,
}

// Used in block
#[derive(Debug, PartialEq, Eq, FromRow, Serialize)]
pub struct InternalCommandMetadata {
  pub command_type: InternalCommandType,
  pub receiver: String,
  pub fee: Option<String>,
  pub hash: String,
  pub creation_fee: Option<String>,
  pub sequence_no: i32,
  pub secondary_sequence_no: i32,
  pub status: TransactionStatus,
  pub coinbase_receiver: Option<String>,
}

pub trait InternalCommandOperationsData {
  fn command_type(&self) -> &InternalCommandType;
  fn receiver(&self) -> &str;
  fn fee(&self) -> String;
  fn creation_fee(&self) -> Option<&String>;
  fn coinbase_receiver(&self) -> Option<&str>;
  fn status(&self) -> &TransactionStatus;
}

impl InternalCommandOperationsData for InternalCommand {
  fn command_type(&self) -> &InternalCommandType {
    &self.command_type
  }

  fn receiver(&self) -> &str {
    &self.receiver
  }

  fn fee(&self) -> String {
    self.fee.clone().unwrap_or("0".to_string())
  }

  fn creation_fee(&self) -> Option<&String> {
    self.creation_fee.as_ref()
  }

  fn coinbase_receiver(&self) -> Option<&str> {
    self.coinbase_receiver.as_deref()
  }

  fn status(&self) -> &TransactionStatus {
    &self.status
  }
}

impl InternalCommandOperationsData for InternalCommandMetadata {
  fn command_type(&self) -> &InternalCommandType {
    &self.command_type
  }

  fn receiver(&self) -> &str {
    &self.receiver
  }

  fn fee(&self) -> String {
    self.fee.clone().unwrap_or("0".to_string())
  }

  fn creation_fee(&self) -> Option<&String> {
    self.creation_fee.as_ref()
  }

  fn coinbase_receiver(&self) -> Option<&str> {
    self.coinbase_receiver.as_deref()
  }

  fn status(&self) -> &TransactionStatus {
    // Assuming metadata always represents applied status
    &TransactionStatus::Applied
  }
}

#[derive(Debug, Display, Hash, PartialEq, Eq)]
pub enum CacheKey {
  NetworkId,
}
