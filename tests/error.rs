use std::env;

use axum::{body::to_bytes, http::StatusCode, response::IntoResponse};
use mina_mesh::{MinaMeshConfig, MinaMeshError};

async fn assert_error_properties(
  error: MinaMeshError,
  expected_code: u8,
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
    (Sql("SQL syntax error".to_string()), 1, "An SQL error occurred.", false, StatusCode::INTERNAL_SERVER_ERROR),
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
    (OperationsNotValid(vec![]), 11, "The provided operations are not valid.", false, StatusCode::BAD_REQUEST),
    (
      UnsupportedOperationForConstruction,
      12,
      "The operation is not supported for transaction construction.",
      false,
      StatusCode::BAD_REQUEST,
    ),
    (SignatureMissing, 13, "A signature is missing.", false, StatusCode::BAD_REQUEST),
    (PublicKeyFormatNotValid, 14, "The public key format is not valid.", false, StatusCode::BAD_REQUEST),
    (NoOptionsProvided, 15, "No options were provided.", false, StatusCode::BAD_REQUEST),
    (
      Exception("Unexpected error".to_string()),
      16,
      "An internal exception occurred.",
      false,
      StatusCode::INTERNAL_SERVER_ERROR,
    ),
    (SignatureInvalid, 17, "The signature is invalid.", false, StatusCode::BAD_REQUEST),
    (MemoInvalid, 18, "The memo is invalid.", false, StatusCode::BAD_REQUEST),
    (GraphqlUriNotSet, 19, "No GraphQL URI has been set.", false, StatusCode::INTERNAL_SERVER_ERROR),
    (TransactionSubmitNoSender, 20, "No sender was found in the ledger.", true, StatusCode::BAD_REQUEST),
    (TransactionSubmitDuplicate, 21, "A duplicate transaction was detected.", false, StatusCode::CONFLICT),
    (TransactionSubmitBadNonce, 22, "The nonce is invalid.", false, StatusCode::BAD_REQUEST),
    (TransactionSubmitFeeSmall, 23, "The transaction fee is too small.", false, StatusCode::BAD_REQUEST),
    (TransactionSubmitInvalidSignature, 24, "The transaction signature is invalid.", false, StatusCode::BAD_REQUEST),
    (TransactionSubmitInsufficientBalance, 25, "The account has insufficient balance.", false, StatusCode::BAD_REQUEST),
    (TransactionSubmitExpired, 26, "The transaction has expired.", false, StatusCode::BAD_REQUEST),
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
    genesis_block_identifier_height: 1,
    genesis_block_identifier_state_hash: "test".to_string(),
    use_search_tx_optimizations: false,
  }
  .to_mina_mesh()
  .await?
  .network_list()
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
    genesis_block_identifier_height: 1,
    genesis_block_identifier_state_hash: "test".to_string(),
    use_search_tx_optimizations: false,
  }
  .to_mina_mesh()
  .await;

  assert!(matches!(res, Err(MinaMeshError::GraphqlUriNotSet)));
  Ok(())
}
