use anyhow::Result;
use insta::assert_debug_snapshot;
use mina_mesh::{
  models::{NetworkIdentifier, SearchTransactionsRequest, TransactionIdentifier},
  MinaMeshConfig,
};

#[tokio::test]
async fn search_transactions_specified() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;

  let request = SearchTransactionsRequest {
    network_identifier: Box::new(NetworkIdentifier::new("mina".to_string(), "mainnet".to_string())),
    // cspell:disable
    address: Some("B62qpXXYbzeZkXrpa3EuZcXgqFSsBsSWrrvi16GJnXLhaqELBSfbnGF".to_string()),
    // cspell:enable
    limit: Some(5),
    offset: Some(0),
    ..Default::default()
  };

  let response = mina_mesh.search_transactions(request).await;

  assert!(response.is_ok());
  assert_debug_snapshot!(response);
  Ok(())
}

#[tokio::test]
async fn search_transactions_failed() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;

  let request = SearchTransactionsRequest {
    network_identifier: Box::new(NetworkIdentifier::new("mina".to_string(), "mainnet".to_string())),
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
    network_identifier: Box::new(NetworkIdentifier::new("mina".to_string(), "mainnet".to_string())),
    max_block: Some(44),
    transaction_identifier: Some(Box::new(TransactionIdentifier::new(
      // cspell:disable-next-line
      "CkpZZWqdA87JmPxHA5NmFEQ3qh7pUmqXi9GBWzf4pADtPEHQAeH7M".to_string(),
    ))),
    limit: Some(5),
    ..Default::default()
  };

  let response = mina_mesh.search_transactions(request).await;

  assert!(response.is_ok());
  assert_debug_snapshot!(response);
  Ok(())
}
