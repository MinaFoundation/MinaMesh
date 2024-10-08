use std::num::ParseIntError;

use cynic::http::CynicReqwestError;
use serde_json::Error as SerdeError;
use sqlx::Error as SqlxError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum MinaMeshError {
  #[error("SQL failure")]
  Sql(String),

  #[error("JSON parse error")]
  JsonParse(Option<String>),

  #[error("GraphQL query failed")]
  GraphqlMinaQuery(String),

  #[error("Network doesn't exist")]
  NetworkDne(String, String),

  #[error("Chain info missing")]
  ChainInfoMissing,

  #[error("Account not found")]
  AccountNotFound(String),

  #[error("Internal invariant violation (you found a bug)")]
  InvariantViolation,

  #[error("Transaction not found")]
  TransactionNotFound(String),

  #[error("Block not found")]
  BlockMissing(String),

  #[error("Malformed public key")]
  MalformedPublicKey,

  #[error("Cannot convert operations to valid transaction")]
  OperationsNotValid(Vec<PartialReason>),

  #[error("Unsupported operation for construction")]
  UnsupportedOperationForConstruction,

  #[error("Signature missing")]
  SignatureMissing,

  #[error("Invalid public key format")]
  PublicKeyFormatNotValid,

  #[error("No options provided")]
  NoOptionsProvided,

  #[error("Exception")]
  Exception(String),

  #[error("Invalid signature")]
  SignatureInvalid,

  #[error("Invalid memo")]
  MemoInvalid,

  #[error("No GraphQL URI set")]
  GraphqlUriNotSet,

  #[error("Can't send transaction: No sender found in ledger")]
  TransactionSubmitNoSender,

  #[error("Can't send transaction: A duplicate is detected")]
  TransactionSubmitDuplicate,

  #[error("Can't send transaction: Nonce invalid")]
  TransactionSubmitBadNonce,

  #[error("Can't send transaction: Fee too small")]
  TransactionSubmitFeeSmall,

  #[error("Can't send transaction: Invalid signature")]
  TransactionSubmitInvalidSignature,

  #[error("Can't send transaction: Insufficient balance")]
  TransactionSubmitInsufficientBalance,

  #[error("Can't send transaction: Expired")]
  TransactionSubmitExpired,
}

#[derive(Debug, PartialEq)]
pub enum PartialReason {
  LengthMismatch,
  FeePayerAndSourceMismatch,
  FeeNotNegative,
  AmountNotSome,
  AccountNotSome,
  InvalidMetadata,
  IncorrectTokenId,
  AmountIncDecMismatch,
  StatusNotPending,
  CanNotFindKind(String),
}

impl MinaMeshError {
  pub fn error_code(&self) -> u8 {
    unimplemented!();
  }

  pub fn description(&self) -> String {
    match self {
      MinaMeshError::Sql(s) => s.clone(),
      _ => unimplemented!(),
    }
  }

  pub fn is_retriable(&self) -> bool {
    unimplemented!();
  }

  pub fn context(&self) -> Option<String> {
    unimplemented!();
  }
}

impl From<SqlxError> for MinaMeshError {
  fn from(value: SqlxError) -> Self {
    MinaMeshError::Sql(value.to_string())
  }
}

impl From<ParseIntError> for MinaMeshError {
  fn from(value: ParseIntError) -> Self {
    MinaMeshError::Exception(value.to_string())
  }
}

impl From<CynicReqwestError> for MinaMeshError {
  fn from(value: CynicReqwestError) -> Self {
    MinaMeshError::GraphqlMinaQuery(value.to_string())
  }
}

// TODO: this isn't necessarily accurate, as we use this for a serialization
// errors as well.
impl From<SerdeError> for MinaMeshError {
  fn from(value: SerdeError) -> Self {
    MinaMeshError::JsonParse(Some(value.to_string()))
  }
}
