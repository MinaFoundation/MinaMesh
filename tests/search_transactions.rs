use anyhow::Result;
use insta::assert_debug_snapshot;
use mina_mesh::{
  models::{AccountIdentifier, SearchTransactionsRequest, TransactionIdentifier},
  test::network_id,
  MinaMeshConfig,
};

#[tokio::test]
async fn search_transactions_specified() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;
  // cspell:disable-next-line
  let address = "B62qkd6yYALkQMq2SFd5B57bJbGBMA2QuGtLPMzRhhnvexRtVRycZWP";

  let request_addr = SearchTransactionsRequest {
    network_identifier: Box::new(network_id()),
    address: Some(address.to_string()),
    limit: Some(5),
    offset: Some(0),
    ..Default::default()
  };

  let response_addr = mina_mesh.search_transactions(request_addr).await;

  let request_acct_id = SearchTransactionsRequest {
    network_identifier: Box::new(network_id()),
    account_identifier: Some(Box::new(AccountIdentifier::new(address.to_string()))),
    limit: Some(5),
    offset: Some(0),
    ..Default::default()
  };

  let response_acct_id = mina_mesh.search_transactions(request_acct_id).await;

  assert_eq!(response_addr, response_acct_id);
  assert!(response_addr.is_ok());
  assert_debug_snapshot!(response_addr);
  Ok(())
}

#[tokio::test]
async fn search_transactions_failed() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;

  let request = SearchTransactionsRequest {
    network_identifier: Box::new(network_id()),
    max_block: Some(44),
    status: Some("failed".to_string()),
    limit: Some(5),
    ..Default::default()
  };

  let response = mina_mesh.search_transactions(request).await;

  assert!(response.is_ok());
  assert_debug_snapshot!(response);
  Ok(())
}

#[tokio::test]
async fn search_transactions_internal_command() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;

  let request = SearchTransactionsRequest {
    network_identifier: Box::new(network_id()),
    max_block: Some(44),
    transaction_identifier: Some(Box::new(TransactionIdentifier::new(
      // cspell:disable-next-line
      "CkpYcKc2oGs8JUd4tmdGBsZXQCQVkayuyffEjrNWctX5Wuad3vVNe".to_string(),
    ))),
    limit: Some(5),
    ..Default::default()
  };

  let response = mina_mesh.search_transactions(request).await;

  assert!(response.is_ok());
  assert_debug_snapshot!(response);
  Ok(())
}

#[tokio::test]
async fn search_transactions_zkapp_success() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;

  let request = SearchTransactionsRequest {
    network_identifier: Box::new(network_id()),
    transaction_identifier: Some(Box::new(TransactionIdentifier::new(
      // cspell:disable-next-line
      "5JvFoEyvuPu9zmi4bDGbhqsakre2SPQU1KKbeh2Lk5uC9eYrc2h2".to_string(),
    ))),
    limit: Some(1),
    ..Default::default()
  };

  let response = mina_mesh.search_transactions(request).await;

  assert!(response.is_ok());
  assert_debug_snapshot!(response);
  Ok(())
}

#[tokio::test]
async fn search_transactions_zkapp_failed() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;

  let request = SearchTransactionsRequest {
    network_identifier: Box::new(network_id()),
    transaction_identifier: Some(Box::new(TransactionIdentifier::new(
      // cspell:disable-next-line
      "5JujBt8rnKheA7CHBnTwUDXrHtQxqPB9LL5Q8y4KwLjPBsBSJuSE".to_string(),
    ))),
    limit: Some(1),
    ..Default::default()
  };

  let response = mina_mesh.search_transactions(request).await;

  assert!(response.is_ok());
  assert_debug_snapshot!(response);
  Ok(())
}

#[tokio::test]
async fn search_transactions_zkapp_tokens_account_identifier() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;
  let max_block = 319857;
  // cspell:disable
  let token_id = "xBxjFpJkbWpbGua7Lf36S1NLhffFoEChyP3pz6SYKnx7dFCTwg";
  let address1 = "B62qituGxc1ZNbWfz4SnftNUaJ78YYYsmuuuJr1FFHjbRqLir7tvBew";
  let address2 = "B62qjwDWxjf4LtJ4YWJQDdTNPqZ69ZyeCzbpAFKN7EoZzYig5ZRz8JE";
  // cspell:enable
  let metadata = serde_json::json!({ "token_id": token_id });

  let request_address1_token = SearchTransactionsRequest {
    network_identifier: Box::new(network_id()),
    max_block: Some(max_block),
    account_identifier: Some(Box::new(AccountIdentifier {
      address: address1.to_string(),
      metadata: Some(metadata.clone()),
      ..Default::default()
    })),
    limit: Some(1),
    ..Default::default()
  };
  let response_address1_token = mina_mesh.search_transactions(request_address1_token).await;

  let request_address2_token = SearchTransactionsRequest {
    network_identifier: Box::new(network_id()),
    max_block: Some(max_block),
    account_identifier: Some(Box::new(AccountIdentifier {
      address: address2.to_string(),
      metadata: Some(metadata.clone()),
      ..Default::default()
    })),
    limit: Some(1),
    ..Default::default()
  };
  let response_address2_token = mina_mesh.search_transactions(request_address2_token).await;

  assert!(response_address1_token.is_ok());
  assert_eq!(response_address1_token, response_address2_token);
  assert_debug_snapshot!(response_address1_token);
  Ok(())
}

#[tokio::test]
async fn search_transactions_zkapp_tokens_tx_hash() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;

  let request_tx_hash = SearchTransactionsRequest {
    network_identifier: Box::new(network_id()),
    transaction_identifier: Some(Box::new(TransactionIdentifier::new(
      // cspell:disable-next-line
      "5JuotEHhjuYbu2oucyTiVhJX3Abx5DPL4NXnM7CP9hfJZLE5G8n9".to_string(),
    ))),
    limit: Some(1),
    ..Default::default()
  };
  let response_tx_hash = mina_mesh.search_transactions(request_tx_hash).await;

  assert!(response_tx_hash.is_ok());
  assert_debug_snapshot!(response_tx_hash);
  Ok(())
}

#[tokio::test]
async fn search_transactions_offset_limit() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;
  // cspell:disable-next-line
  let address = "B62qrHd4Wg8z6N6tCC9pVtRtxEuBXLWPH61gbcgotURdiU1rSURMdFB";
  let max_block = 370_000;
  let request = SearchTransactionsRequest {
    network_identifier: Box::new(network_id()),
    address: Some(address.to_string()),
    max_block: Some(max_block),
    limit: Some(15),
    offset: Some(5),
    ..Default::default()
  };
  let response = mina_mesh.search_transactions(request).await;

  assert!(response.is_ok());
  assert_debug_snapshot!(response);
  Ok(())
}

#[tokio::test]
async fn search_transactions_uc_include_timestamp() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;

  let request = SearchTransactionsRequest {
    network_identifier: Box::new(network_id()),
    include_timestamp: Some(true),
    address: Some(
      // cspell:disable-next-line
      "B62qkd6yYALkQMq2SFd5B57bJbGBMA2QuGtLPMzRhhnvexRtVRycZWP".to_string(),
    ),
    limit: Some(5),
    ..Default::default()
  };

  let response = mina_mesh.search_transactions(request).await;

  assert!(response.is_ok());
  assert_debug_snapshot!(response);
  Ok(())
}

#[tokio::test]
async fn search_transactions_ic_include_timestamp() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;

  let request = SearchTransactionsRequest {
    network_identifier: Box::new(network_id()),
    max_block: Some(44),
    include_timestamp: Some(true),
    transaction_identifier: Some(Box::new(TransactionIdentifier::new(
      // cspell:disable-next-line
      "CkpYcKc2oGs8JUd4tmdGBsZXQCQVkayuyffEjrNWctX5Wuad3vVNe".to_string(),
    ))),
    limit: Some(5),
    ..Default::default()
  };

  let response = mina_mesh.search_transactions(request).await;

  assert!(response.is_ok());
  assert_debug_snapshot!(response);
  Ok(())
}

#[tokio::test]
async fn search_transactions_zk_include_timestamp() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;

  let request = SearchTransactionsRequest {
    network_identifier: Box::new(network_id()),
    transaction_identifier: Some(Box::new(TransactionIdentifier::new(
      // cspell:disable-next-line
      "5JvFoEyvuPu9zmi4bDGbhqsakre2SPQU1KKbeh2Lk5uC9eYrc2h2".to_string(),
    ))),
    limit: Some(1),
    include_timestamp: Some(true),
    ..Default::default()
  };

  let response = mina_mesh.search_transactions(request).await;

  assert!(response.is_ok());
  assert_debug_snapshot!(response);
  Ok(())
}
