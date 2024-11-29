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
  pub height: Option<i64>,
  pub total_count: Option<i64>,
  pub block_id: Option<i32>,
}

#[derive(Debug, Display, Hash, PartialEq, Eq)]
pub enum CacheKey {
  NetworkId,
}
