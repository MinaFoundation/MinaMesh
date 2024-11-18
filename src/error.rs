use std::num::ParseIntError;

use axum::{
  extract::rejection::JsonRejection,
  http::StatusCode,
  response::{IntoResponse, Response},
  Json,
};
use cynic::http::CynicReqwestError;
use serde::Serialize;
use serde_json::{json, Error as SerdeError};
use sqlx::Error as SqlxError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Serialize, Clone)]
pub enum MinaMeshError {
  #[error("SQL failure: {0}")]
  Sql(String),

  #[error("JSON parse error")]
  JsonParse(Option<String>),

  #[error("GraphQL query failed: {0}")]
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

  #[error("Exception {0}")]
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

#[derive(Clone, Debug, PartialEq, Serialize)]
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
  /// Returns the error code for the error.
  pub fn error_code(&self) -> u8 {
    match self {
      MinaMeshError::Sql(_) => 1,
      MinaMeshError::JsonParse(_) => 2,
      MinaMeshError::GraphqlMinaQuery(_) => 3,
      MinaMeshError::NetworkDne(_, _) => 4,
      MinaMeshError::ChainInfoMissing => 5,
      MinaMeshError::AccountNotFound(_) => 6,
      MinaMeshError::InvariantViolation => 7,
      MinaMeshError::TransactionNotFound(_) => 8,
      MinaMeshError::BlockMissing(_) => 9,
      MinaMeshError::MalformedPublicKey => 10,
      MinaMeshError::OperationsNotValid(_) => 11,
      MinaMeshError::UnsupportedOperationForConstruction => 12,
      MinaMeshError::SignatureMissing => 13,
      MinaMeshError::PublicKeyFormatNotValid => 14,
      MinaMeshError::NoOptionsProvided => 15,
      MinaMeshError::Exception(_) => 16,
      MinaMeshError::SignatureInvalid => 17,
      MinaMeshError::MemoInvalid => 18,
      MinaMeshError::GraphqlUriNotSet => 19,
      MinaMeshError::TransactionSubmitNoSender => 20,
      MinaMeshError::TransactionSubmitDuplicate => 21,
      MinaMeshError::TransactionSubmitBadNonce => 22,
      MinaMeshError::TransactionSubmitFeeSmall => 23,
      MinaMeshError::TransactionSubmitInvalidSignature => 24,
      MinaMeshError::TransactionSubmitInsufficientBalance => 25,
      MinaMeshError::TransactionSubmitExpired => 26,
    }
  }

  /// Returns whether the error is retriable.
  pub fn is_retriable(&self) -> bool {
    matches!(
      self,
      MinaMeshError::GraphqlMinaQuery(_)
        | MinaMeshError::TransactionSubmitNoSender
        | MinaMeshError::AccountNotFound(_)
        | MinaMeshError::TransactionNotFound(_)
        | MinaMeshError::BlockMissing(_)
        | MinaMeshError::ChainInfoMissing
    )
  }

  /// Provides additional details about the error.
  pub fn details(&self) -> serde_json::Value {
    match self {
      MinaMeshError::GraphqlMinaQuery(msg) => json!({
          "error": msg,
          "extra": "Internal POST to Mina Daemon failed"
      }),
      MinaMeshError::Sql(msg) => json!({ "error": msg }),
      MinaMeshError::JsonParse(Some(msg)) => json!({ "error": msg }),
      _ => json!(""),
    }
  }

  /// Converts the error into a JSON representation.
  pub fn to_json(&self) -> serde_json::Value {
    json!({
        "code": self.error_code(),
        "message": self.to_string(),
        "description": self.description(),
        "retriable": self.is_retriable(),
        "details": self.details(),
    })
  }

  /// Returns a human-readable description of the error.
  pub fn description(&self) -> String {
    match self {
      MinaMeshError::Sql(_) => "An SQL error occurred.".to_string(),
      MinaMeshError::JsonParse(_) => "Failed to parse JSON.".to_string(),
      MinaMeshError::GraphqlMinaQuery(_) => "The GraphQL query failed.".to_string(),
      MinaMeshError::NetworkDne(_, _) => "The specified network does not exist.".to_string(),
      MinaMeshError::ChainInfoMissing => "Chain info is missing.".to_string(),
      MinaMeshError::AccountNotFound(_) => "The specified account could not be found.".to_string(),
      MinaMeshError::InvariantViolation => "An internal invariant was violated.".to_string(),
      MinaMeshError::TransactionNotFound(_) => "The specified transaction could not be found.".to_string(),
      MinaMeshError::BlockMissing(_) => "The specified block could not be found.".to_string(),
      MinaMeshError::MalformedPublicKey => "The provided public key is malformed.".to_string(),
      MinaMeshError::OperationsNotValid(_) => "The provided operations are not valid.".to_string(),
      MinaMeshError::UnsupportedOperationForConstruction => {
        "The operation is not supported for transaction construction.".to_string()
      }
      MinaMeshError::SignatureMissing => "A signature is missing.".to_string(),
      MinaMeshError::PublicKeyFormatNotValid => "The public key format is not valid.".to_string(),
      MinaMeshError::NoOptionsProvided => "No options were provided.".to_string(),
      MinaMeshError::Exception(_) => "An internal exception occurred.".to_string(),
      MinaMeshError::SignatureInvalid => "The signature is invalid.".to_string(),
      MinaMeshError::MemoInvalid => "The memo is invalid.".to_string(),
      MinaMeshError::GraphqlUriNotSet => "No GraphQL URI has been set.".to_string(),
      MinaMeshError::TransactionSubmitNoSender => "No sender was found in the ledger.".to_string(),
      MinaMeshError::TransactionSubmitDuplicate => "A duplicate transaction was detected.".to_string(),
      MinaMeshError::TransactionSubmitBadNonce => "The nonce is invalid.".to_string(),
      MinaMeshError::TransactionSubmitFeeSmall => "The transaction fee is too small.".to_string(),
      MinaMeshError::TransactionSubmitInvalidSignature => "The transaction signature is invalid.".to_string(),
      MinaMeshError::TransactionSubmitInsufficientBalance => "The account has insufficient balance.".to_string(),
      MinaMeshError::TransactionSubmitExpired => "The transaction has expired.".to_string(),
    }
  }
}

impl IntoResponse for MinaMeshError {
  fn into_response(self) -> Response {
    let status_code = StatusCode::BAD_REQUEST;
    let body = json!({
        "code": self.error_code(),
        "message": self.to_string(),
        "description": self.description(),
        "retriable": self.is_retriable(),
        "details": self.details(),
    });

    (status_code, Json(body)).into_response()
  }
}

/// Implement `From` conversions for third-party errors.
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

impl From<anyhow::Error> for MinaMeshError {
  fn from(error: anyhow::Error) -> Self {
    if let Some(mina_error) = error.downcast_ref::<MinaMeshError>() {
      // Clone the original MinaMeshError if it exists
      (*mina_error).clone()
    } else {
      // Fallback to wrapping as Exception if it's not a MinaMeshError
      MinaMeshError::Exception(error.to_string())
    }
  }
}

/// Convert Axum's JsonRejection into MinaMeshError.
impl From<JsonRejection> for MinaMeshError {
  fn from(err: JsonRejection) -> Self {
    MinaMeshError::JsonParse(Some(err.to_string()))
  }
}
