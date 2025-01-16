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

  #[error("Network doesn't exist, expected: {0}, actual: {1}")]
  NetworkDne(String, String),

  #[error("Chain info missing")]
  ChainInfoMissing,

  #[error("Account not found: {0}")]
  AccountNotFound(String),

  #[error("Internal invariant violation (you found a bug)")]
  InvariantViolation,

  #[error("Transaction not found: {0}")]
  TransactionNotFound(String),

  #[error("Block not found")]
  BlockMissing(Option<i64>, Option<String>),

  #[error("Malformed public key: {0}")]
  MalformedPublicKey(String),

  #[error("Cannot convert operations to valid transaction")]
  OperationsNotValid(Vec<PartialReason>),

  #[error("Unsupported operation for construction")]
  UnsupportedOperationForConstruction,

  #[error("Signature missing")]
  SignatureMissing,

  #[error("Invalid public key format")]
  PublicKeyFormatNotValid(String),

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
  AmountNotValid,
  AccountNotSome,
  InvalidMetadata,
  IncorrectTokenId,
  AmountIncDecMismatch,
  StatusNotPending,
  CanNotFindKind(String),
}

impl MinaMeshError {
  pub fn all_errors() -> Vec<Self> {
    vec![
      MinaMeshError::Sql("SQL syntax error".to_string()),
      MinaMeshError::JsonParse(Some("Missing field".to_string())),
      MinaMeshError::GraphqlMinaQuery("Timeout".to_string()),
      MinaMeshError::NetworkDne("mina:expected".to_string(), "mina:actual".to_string()),
      MinaMeshError::ChainInfoMissing,
      MinaMeshError::AccountNotFound("Account ID".to_string()),
      MinaMeshError::InvariantViolation,
      MinaMeshError::TransactionNotFound("Transaction ID".to_string()),
      MinaMeshError::BlockMissing(Some(-1), Some("test_hash".to_string())),
      MinaMeshError::MalformedPublicKey("Error message".to_string()),
      MinaMeshError::OperationsNotValid(vec![]),
      MinaMeshError::UnsupportedOperationForConstruction,
      MinaMeshError::SignatureMissing,
      MinaMeshError::PublicKeyFormatNotValid("Error message".to_string()),
      MinaMeshError::NoOptionsProvided,
      MinaMeshError::Exception("Unexpected error".to_string()),
      MinaMeshError::SignatureInvalid,
      MinaMeshError::MemoInvalid,
      MinaMeshError::GraphqlUriNotSet,
      MinaMeshError::TransactionSubmitNoSender,
      MinaMeshError::TransactionSubmitDuplicate,
      MinaMeshError::TransactionSubmitBadNonce,
      MinaMeshError::TransactionSubmitFeeSmall,
      MinaMeshError::TransactionSubmitInvalidSignature,
      MinaMeshError::TransactionSubmitInsufficientBalance,
      MinaMeshError::TransactionSubmitExpired,
    ]
  }

  /// Returns the error code for the error.
  pub fn error_code(&self) -> i32 {
    match self {
      MinaMeshError::Sql(_) => 1,
      MinaMeshError::JsonParse(_) => 2,
      MinaMeshError::GraphqlMinaQuery(_) => 3,
      MinaMeshError::NetworkDne(_, _) => 4,
      MinaMeshError::ChainInfoMissing => 5,
      MinaMeshError::AccountNotFound(_) => 6,
      MinaMeshError::InvariantViolation => 7,
      MinaMeshError::TransactionNotFound(_) => 8,
      MinaMeshError::BlockMissing(_, _) => 9,
      MinaMeshError::MalformedPublicKey(_) => 10,
      MinaMeshError::OperationsNotValid(_) => 11,
      MinaMeshError::UnsupportedOperationForConstruction => 12,
      MinaMeshError::SignatureMissing => 13,
      MinaMeshError::PublicKeyFormatNotValid(_) => 14,
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
        | MinaMeshError::BlockMissing(_, _)
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
      MinaMeshError::Sql(msg) => json!({
        "error": msg,
        "extra": "Internal SQL query failed"
      }),
      MinaMeshError::JsonParse(Some(msg)) => json!({
        "error": msg,
        "extra": "Failed to parse JSON body"
      }),
      MinaMeshError::AccountNotFound(account) => json!({
        "error": format!("You attempted to lookup {}, but we couldn't find it in the ledger.", account),
        "account": account,
      }),
      MinaMeshError::Exception(msg) => json!({
        "error": msg,
      }),
      MinaMeshError::NetworkDne(expected, actual) => json!({
        "error": format!("You are requesting the status for the network {}, but you are connected to the network {}", expected, actual),
      }),
      MinaMeshError::TransactionNotFound(tx) => json!({
        "error": format!(
          "You attempted to lookup transaction {}, but it is missing from the mempool. {} {}",
          tx,
          "This may be due to its inclusion in a block -- try looking for this transaction in a recent block.",
          "It also could be due to the transaction being evicted from the mempool."
          ),
        "transaction": tx,
      }),
      MinaMeshError::MalformedPublicKey(err) => json!({
        "error": err,
      }),
      MinaMeshError::PublicKeyFormatNotValid(err) => json!({
        "error": err,
      }),
      MinaMeshError::OperationsNotValid(reasons) => json!({
        "error": "We could not convert those operations to a valid transaction.",
        "reasons": reasons,
      }),
      MinaMeshError::BlockMissing(index, hash) => {
        let block_identifier = match (index, hash) {
          (Some(idx), Some(hsh)) => format!("index={}, hash={}", idx, hsh),
          (Some(idx), None) => format!("index={}", idx),
          (None, Some(hsh)) => format!("hash={}", hsh),
          (None, None) => "no identifying information (index or hash)".to_string(),
        };

        let error_message =
          format!("We couldn't find the block in the archive node, specified by {}.", block_identifier);
        json!({
            "error": error_message,
            "block": {
                "index": index,
                "hash": hash,
            },
        })
      }

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
      MinaMeshError::Sql(_) => "We encountered a SQL failure.".to_string(),
      MinaMeshError::JsonParse(_) => "We encountered an error while parsing JSON.".to_string(),
      MinaMeshError::GraphqlMinaQuery(_) => "The GraphQL query failed.".to_string(),
      MinaMeshError::NetworkDne(_, _) => "The specified network does not exist.".to_string(),
      MinaMeshError::ChainInfoMissing => "Chain info is missing.".to_string(),
      MinaMeshError::AccountNotFound(_) => "The specified account could not be found.".to_string(),
      MinaMeshError::InvariantViolation => "An internal invariant was violated.".to_string(),
      MinaMeshError::TransactionNotFound(_) => "The specified transaction could not be found.".to_string(),
      MinaMeshError::BlockMissing(_, _) => "The specified block could not be found.".to_string(),
      MinaMeshError::MalformedPublicKey(_) => "The provided public key is malformed.".to_string(),
      MinaMeshError::OperationsNotValid(_) => {
        "We could not convert those operations to a valid transaction.".to_string()
      }
      MinaMeshError::UnsupportedOperationForConstruction => {
        "The operation is not supported for transaction construction.".to_string()
      }
      MinaMeshError::SignatureMissing => "Your request is missing a signature.".to_string(),
      MinaMeshError::PublicKeyFormatNotValid(_) => "The public key you provided had an invalid format.".to_string(),
      MinaMeshError::NoOptionsProvided => "Your request is missing options.".to_string(),
      MinaMeshError::Exception(_) => "An internal exception occurred.".to_string(),
      MinaMeshError::SignatureInvalid => "Your request has an invalid signature.".to_string(),
      MinaMeshError::MemoInvalid => "Your request has an invalid memo.".to_string(),
      MinaMeshError::GraphqlUriNotSet => "No GraphQL URI has been set.".to_string(),
      MinaMeshError::TransactionSubmitNoSender => {
        "This could occur because the node isn't fully synced or the account doesn't actually exist in the ledger yet."
          .to_string()
      }
      MinaMeshError::TransactionSubmitDuplicate => "A duplicate transaction was detected.".to_string(),
      MinaMeshError::TransactionSubmitBadNonce => "The nonce is invalid.".to_string(),
      MinaMeshError::TransactionSubmitFeeSmall => "The transaction fee is too small.".to_string(),
      MinaMeshError::TransactionSubmitInvalidSignature => {
        "An invalid signature is attached to this transaction.".to_string()
      }
      MinaMeshError::TransactionSubmitInsufficientBalance => {
        "This account do not have sufficient balance perform the requested transaction.".to_string()
      }
      MinaMeshError::TransactionSubmitExpired => {
        "This transaction is expired. Please try again with a larger valid_until.".to_string()
      }
    }
  }
}

impl IntoResponse for MinaMeshError {
  fn into_response(self) -> Response {
    let status_code = match self {
      MinaMeshError::Sql(_) => StatusCode::INTERNAL_SERVER_ERROR,
      MinaMeshError::JsonParse(_) => StatusCode::BAD_REQUEST,
      MinaMeshError::GraphqlMinaQuery(_) => StatusCode::BAD_GATEWAY,
      MinaMeshError::NetworkDne(_, _) => StatusCode::NOT_FOUND,
      MinaMeshError::ChainInfoMissing => StatusCode::INTERNAL_SERVER_ERROR,
      MinaMeshError::AccountNotFound(_) => StatusCode::NOT_FOUND,
      MinaMeshError::InvariantViolation => StatusCode::INTERNAL_SERVER_ERROR,
      MinaMeshError::TransactionNotFound(_) => StatusCode::NOT_FOUND,
      MinaMeshError::BlockMissing(_, _) => StatusCode::NOT_FOUND,
      MinaMeshError::MalformedPublicKey(_) => StatusCode::BAD_REQUEST,
      MinaMeshError::OperationsNotValid(_) => StatusCode::BAD_REQUEST,
      MinaMeshError::UnsupportedOperationForConstruction => StatusCode::BAD_REQUEST,
      MinaMeshError::SignatureMissing => StatusCode::BAD_REQUEST,
      MinaMeshError::PublicKeyFormatNotValid(_) => StatusCode::BAD_REQUEST,
      MinaMeshError::NoOptionsProvided => StatusCode::BAD_REQUEST,
      MinaMeshError::Exception(_) => StatusCode::INTERNAL_SERVER_ERROR,
      MinaMeshError::SignatureInvalid => StatusCode::BAD_REQUEST,
      MinaMeshError::MemoInvalid => StatusCode::BAD_REQUEST,
      MinaMeshError::GraphqlUriNotSet => StatusCode::INTERNAL_SERVER_ERROR,
      MinaMeshError::TransactionSubmitNoSender => StatusCode::BAD_REQUEST,
      MinaMeshError::TransactionSubmitDuplicate => StatusCode::CONFLICT,
      MinaMeshError::TransactionSubmitBadNonce => StatusCode::BAD_REQUEST,
      MinaMeshError::TransactionSubmitFeeSmall => StatusCode::BAD_REQUEST,
      MinaMeshError::TransactionSubmitInvalidSignature => StatusCode::BAD_REQUEST,
      MinaMeshError::TransactionSubmitInsufficientBalance => StatusCode::BAD_REQUEST,
      MinaMeshError::TransactionSubmitExpired => StatusCode::BAD_REQUEST,
    };

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
      mina_error.clone()
    } else {
      // Fallback to wrapping as Exception if it's not a MinaMeshError
      MinaMeshError::Exception(error.to_string())
    }
  }
}

/// Convert Axum's JsonRejection into MinaMeshError.
impl From<JsonRejection> for MinaMeshError {
  fn from(err: JsonRejection) -> Self {
    MinaMeshError::JsonParse(Some(err.body_text()))
  }
}

impl From<reqwest::Error> for MinaMeshError {
  fn from(value: reqwest::Error) -> Self {
    MinaMeshError::Exception(value.to_string())
  }
}

impl From<MinaMeshError> for coinbase_mesh::models::Error {
  fn from(error: MinaMeshError) -> Self {
    coinbase_mesh::models::Error {
      code: error.error_code(),
      message: error.to_string(),
      description: Some(error.description()),
      retriable: error.is_retriable(),
      details: Some(error.details()),
    }
  }
}
