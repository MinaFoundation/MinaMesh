use std::env;

// use axum::http::StatusCode;
use mina_mesh::{MinaMeshConfig, MinaMeshError};
// use serde_json::json;

#[test]
fn test_error_codes_and_descriptions() {
  let error = MinaMeshError::Sql("SQL syntax error".to_string());
  assert_eq!(error.error_code(), 1);
  assert_eq!(error.description(), "An SQL error occurred.");
  assert!(!error.is_retriable());

  let error = MinaMeshError::JsonParse(Some("Missing field".to_string()));
  assert_eq!(error.error_code(), 2);
  assert_eq!(error.description(), "We encountered an error while parsing JSON.");
  assert!(!error.is_retriable());

  let error = MinaMeshError::GraphqlMinaQuery("Timeout".to_string());
  assert_eq!(error.error_code(), 3);
  assert_eq!(error.description(), "The GraphQL query failed.");
  assert!(error.is_retriable());

  let error = MinaMeshError::NetworkDne("blockchain".to_string(), "network".to_string());
  assert_eq!(error.error_code(), 4);
  assert_eq!(error.description(), "The specified network does not exist.");
  assert!(!error.is_retriable());

  let error = MinaMeshError::ChainInfoMissing;
  assert_eq!(error.error_code(), 5);
  assert_eq!(error.description(), "Chain info is missing.");
  assert!(error.is_retriable());

  let error = MinaMeshError::AccountNotFound("Account ID".to_string());
  assert_eq!(error.error_code(), 6);
  assert_eq!(error.description(), "The specified account could not be found.");
  assert!(error.is_retriable());

  let error = MinaMeshError::InvariantViolation;
  assert_eq!(error.error_code(), 7);
  assert_eq!(error.description(), "An internal invariant was violated.");
  assert!(!error.is_retriable());

  let error = MinaMeshError::TransactionNotFound("Transaction ID".to_string());
  assert_eq!(error.error_code(), 8);
  assert_eq!(error.description(), "The specified transaction could not be found.");
  assert!(error.is_retriable());

  let error = MinaMeshError::BlockMissing("Block ID".to_string());
  assert_eq!(error.error_code(), 9);
  assert_eq!(error.description(), "The specified block could not be found.");
  assert!(error.is_retriable());

  let error = MinaMeshError::MalformedPublicKey;
  assert_eq!(error.error_code(), 10);
  assert_eq!(error.description(), "The provided public key is malformed.");
  assert!(!error.is_retriable());

  let error = MinaMeshError::OperationsNotValid(vec![]);
  assert_eq!(error.error_code(), 11);
  assert_eq!(error.description(), "The provided operations are not valid.");
  assert!(!error.is_retriable());

  let error = MinaMeshError::UnsupportedOperationForConstruction;
  assert_eq!(error.error_code(), 12);
  assert_eq!(error.description(), "The operation is not supported for transaction construction.");
  assert!(!error.is_retriable());

  let error = MinaMeshError::SignatureMissing;
  assert_eq!(error.error_code(), 13);
  assert_eq!(error.description(), "A signature is missing.");
  assert!(!error.is_retriable());

  let error = MinaMeshError::PublicKeyFormatNotValid;
  assert_eq!(error.error_code(), 14);
  assert_eq!(error.description(), "The public key format is not valid.");
  assert!(!error.is_retriable());

  let error = MinaMeshError::NoOptionsProvided;
  assert_eq!(error.error_code(), 15);
  assert_eq!(error.description(), "No options were provided.");
  assert!(!error.is_retriable());

  let error = MinaMeshError::Exception("Unexpected error".to_string());
  assert_eq!(error.error_code(), 16);
  assert_eq!(error.description(), "An internal exception occurred.");
  assert!(!error.is_retriable());

  let error = MinaMeshError::SignatureInvalid;
  assert_eq!(error.error_code(), 17);
  assert_eq!(error.description(), "The signature is invalid.");
  assert!(!error.is_retriable());

  let error = MinaMeshError::MemoInvalid;
  assert_eq!(error.error_code(), 18);
  assert_eq!(error.description(), "The memo is invalid.");
  assert!(!error.is_retriable());

  let error = MinaMeshError::GraphqlUriNotSet;
  assert_eq!(error.error_code(), 19);
  assert_eq!(error.description(), "No GraphQL URI has been set.");
  assert!(!error.is_retriable());

  let error = MinaMeshError::TransactionSubmitNoSender;
  assert_eq!(error.error_code(), 20);
  assert_eq!(error.description(), "No sender was found in the ledger.");
  assert!(error.is_retriable());

  let error = MinaMeshError::TransactionSubmitDuplicate;
  assert_eq!(error.error_code(), 21);
  assert_eq!(error.description(), "A duplicate transaction was detected.");
  assert!(!error.is_retriable());

  let error = MinaMeshError::TransactionSubmitBadNonce;
  assert_eq!(error.error_code(), 22);
  assert_eq!(error.description(), "The nonce is invalid.");
  assert!(!error.is_retriable());

  let error = MinaMeshError::TransactionSubmitFeeSmall;
  assert_eq!(error.error_code(), 23);
  assert_eq!(error.description(), "The transaction fee is too small.");
  assert!(!error.is_retriable());

  let error = MinaMeshError::TransactionSubmitInvalidSignature;
  assert_eq!(error.error_code(), 24);
  assert_eq!(error.description(), "The transaction signature is invalid.");
  assert!(!error.is_retriable());

  let error = MinaMeshError::TransactionSubmitInsufficientBalance;
  assert_eq!(error.error_code(), 25);
  assert_eq!(error.description(), "The account has insufficient balance.");
  assert!(!error.is_retriable());

  let error = MinaMeshError::TransactionSubmitExpired;
  assert_eq!(error.error_code(), 26);
  assert_eq!(error.description(), "The transaction has expired.");
  assert!(!error.is_retriable());
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
