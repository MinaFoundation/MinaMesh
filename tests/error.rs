use std::env;

use axum::{body::to_bytes, http::StatusCode, response::IntoResponse};
use mina_mesh::{MinaMeshConfig, MinaMeshError};

async fn assert_error_properties(
  error: MinaMeshError,
  expected_code: i32,
  expected_description: &str,
  expected_retriable: bool,
  expected_status: StatusCode,
) {
  assert_eq!(error.error_code(), expected_code);
  assert_eq!(error.description(), expected_description);
  assert_eq!(error.is_retriable(), expected_retriable);

  let message = error.to_string();
  let response = error.into_response();
  assert_eq!(response.status(), expected_status);

  let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
  let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

  assert_eq!(json["code"], expected_code);
  assert_eq!(json["message"], message);
  assert_eq!(json["description"], expected_description);
  assert_eq!(json["retriable"], expected_retriable);
}

#[tokio::test]
async fn test_error_properties() {
  use MinaMeshError::*;

  let cases = vec![
    (Sql("SQL syntax error".to_string()), 1, "We encountered a SQL failure.", false, StatusCode::INTERNAL_SERVER_ERROR),
    (
      JsonParse(Some("Missing field".to_string())),
      2,
      "We encountered an error while parsing JSON.",
      false,
      StatusCode::BAD_REQUEST,
    ),
    (GraphqlMinaQuery("Timeout".to_string()), 3, "The GraphQL query failed.", true, StatusCode::BAD_GATEWAY),
    (
      NetworkDne("blockchain".to_string(), "network".to_string()),
      4,
      "The specified network does not exist.",
      false,
      StatusCode::NOT_FOUND,
    ),
    (ChainInfoMissing, 5, "Chain info is missing.", true, StatusCode::INTERNAL_SERVER_ERROR),
    (
      AccountNotFound("Account ID".to_string()),
      6,
      "The specified account could not be found.",
      true,
      StatusCode::NOT_FOUND,
    ),
    (InvariantViolation, 7, "An internal invariant was violated.", false, StatusCode::INTERNAL_SERVER_ERROR),
    (
      TransactionNotFound("Transaction ID".to_string()),
      8,
      "The specified transaction could not be found.",
      true,
      StatusCode::NOT_FOUND,
    ),
    (BlockMissing("Block ID".to_string()), 9, "The specified block could not be found.", true, StatusCode::NOT_FOUND),
    (MalformedPublicKey, 10, "The provided public key is malformed.", false, StatusCode::BAD_REQUEST),
    (
      OperationsNotValid(vec![]),
      11,
      "We could not convert those operations to a valid transaction.",
      false,
      StatusCode::BAD_REQUEST,
    ),
    (
      UnsupportedOperationForConstruction,
      12,
      "The operation is not supported for transaction construction.",
      false,
      StatusCode::BAD_REQUEST,
    ),
    (SignatureMissing, 13, "Your request is missing a signature.", false, StatusCode::BAD_REQUEST),
    (PublicKeyFormatNotValid, 14, "The public key you provided had an invalid format.", false, StatusCode::BAD_REQUEST),
    (NoOptionsProvided, 15, "Your request is missing options.", false, StatusCode::BAD_REQUEST),
    (
      Exception("Unexpected error".to_string()),
      16,
      "An internal exception occurred.",
      false,
      StatusCode::INTERNAL_SERVER_ERROR,
    ),
    (SignatureInvalid, 17, "Your request has an invalid signature.", false, StatusCode::BAD_REQUEST),
    (MemoInvalid, 18, "Your request has an invalid memo.", false, StatusCode::BAD_REQUEST),
    (GraphqlUriNotSet, 19, "No GraphQL URI has been set.", false, StatusCode::INTERNAL_SERVER_ERROR),
    (
      TransactionSubmitNoSender,
      20,
      "This could occur because the node isn't fully synced or the account doesn't actually exist in the ledger yet.",
      true,
      StatusCode::BAD_REQUEST,
    ),
    (TransactionSubmitDuplicate, 21, "A duplicate transaction was detected.", false, StatusCode::CONFLICT),
    (TransactionSubmitBadNonce, 22, "The nonce is invalid.", false, StatusCode::BAD_REQUEST),
    (TransactionSubmitFeeSmall, 23, "The transaction fee is too small.", false, StatusCode::BAD_REQUEST),
    (
      TransactionSubmitInvalidSignature,
      24,
      "An invalid signature is attached to this transaction.",
      false,
      StatusCode::BAD_REQUEST,
    ),
    (
      TransactionSubmitInsufficientBalance,
      25,
      "This account do not have sufficient balance perform the requested transaction.",
      false,
      StatusCode::BAD_REQUEST,
    ),
    (
      TransactionSubmitExpired,
      26,
      "This transaction is expired. Please try again with a larger valid_until.",
      false,
      StatusCode::BAD_REQUEST,
    ),
  ];

  for (error, code, description, retriable, status) in cases {
    assert_error_properties(error, code, description, retriable, status).await;
  }
}

#[test]
fn test_conversion_from_sqlx_error() {
  let sqlx_error = sqlx::Error::RowNotFound;
  let error: MinaMeshError = sqlx_error.into();
  assert!(matches!(error, MinaMeshError::Sql(_)));
}

#[test]
fn test_conversion_from_parse_int_error() {
  let parse_error: Result<i32, _> = "abc".parse();
  if let Err(err) = parse_error {
    let error: MinaMeshError = err.into();
    assert!(matches!(error, MinaMeshError::Exception(_)));
  }
}

#[tokio::test]
async fn test_conversion_from_cynic_reqwest_error() -> Result<(), MinaMeshError> {
  dotenv::dotenv().ok();
  let res = MinaMeshConfig {
    proxy_url: "http://wrong-graphql".to_string(),
    archive_database_url: env::var("MINAMESH_ARCHIVE_DATABASE_URL").unwrap(),
    max_db_pool_size: 10,
    db_pool_idle_timeout: 1,
    use_search_tx_optimizations: false,
  }
  .to_mina_mesh()
  .await;
  // Assert that the error matches MinaMeshError::GraphqlMinaQuery
  assert!(matches!(res, Err(MinaMeshError::GraphqlMinaQuery(_))));
  Ok(())
}

#[test]
fn test_conversion_from_anyhow_error() {
  let anyhow_error = anyhow::Error::msg("Unexpected issue");
  let error: MinaMeshError = anyhow_error.into();

  assert!(matches!(error, MinaMeshError::Exception(_)));
}

#[tokio::test]
async fn test_graphql_uri_not_set_error() -> Result<(), MinaMeshError> {
  dotenv::dotenv().ok();
  let res = MinaMeshConfig {
    proxy_url: "".to_string(),
    archive_database_url: env::var("MINAMESH_ARCHIVE_DATABASE_URL").unwrap(),
    max_db_pool_size: 10,
    db_pool_idle_timeout: 1,
    use_search_tx_optimizations: false,
  }
  .to_mina_mesh()
  .await;

  assert!(matches!(res, Err(MinaMeshError::GraphqlUriNotSet)));
  Ok(())
}
