use anyhow::Result;
use insta::assert_debug_snapshot;
use mina_mesh::{models::ConstructionMetadataRequest, test::network_id, MinaMeshConfig};

#[tokio::test]
async fn construction_metadata_ok() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;

  let request = ConstructionMetadataRequest {
    network_identifier: network_id().into(),
    options: Some(serde_json::json!({
      // cspell:disable
      "sender": "B62qkd6yYALkQMq2SFd5B57bJbGBMA2QuGtLPMzRhhnvexRtVRycZWP",
      "receiver": "B62qnXy1f75qq8c6HS2Am88Gk6UyvTHK3iSYh4Hb3nD6DS2eS6wZ4or",
      "token_id": "wSHV2S4qX9jFsLjQo8r1BsMLH2ZRKsZx6EJd1sbozGPieEC4Jf",
      // cspell:enable
      "valid_until": "20000",
      "memo": "test transaction"
    })),
    public_keys: None,
  };

  let response = mina_mesh.construction_metadata(request).await;

  assert!(response.is_ok());
  let response = response.unwrap();
  let metadata_str = serde_json::to_string(&response.metadata)?;
  // cspell:disable
  assert!(metadata_str.contains("B62qkd6yYALkQMq2SFd5B57bJbGBMA2QuGtLPMzRhhnvexRtVRycZWP"));
  assert!(metadata_str.contains("B62qnXy1f75qq8c6HS2Am88Gk6UyvTHK3iSYh4Hb3nD6DS2eS6wZ4or"));
  assert!(metadata_str.contains("wSHV2S4qX9jFsLjQo8r1BsMLH2ZRKsZx6EJd1sbozGPieEC4Jf"));
  // cspell:enable
  assert!(metadata_str.contains("test transaction"));
  assert!(metadata_str.contains("20000"));
  assert!(metadata_str.contains("account_creation_fee"));
  Ok(())
}

#[tokio::test]
async fn construction_metadata_acct_not_found() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;

  let request = ConstructionMetadataRequest {
    network_identifier: network_id().into(),
    options: Some(serde_json::json!({
      // cspell:disable
      "sender": "B62qpLST3UC1rpVT6SHfB7wqW2iQgiopFAGfrcovPgLjgfpDUN2LLeg",
      "receiver": "B62qnXy1f75qq8c6HS2Am88Gk6UyvTHK3iSYh4Hb3nD6DS2eS6wZ4or",
      // cspell:enable
      "token_id": "1",
      "valid_until": "20000",
      "memo": "test transaction"
    })),
    public_keys: None,
  };

  let response = mina_mesh.construction_metadata(request).await;

  assert!(response.is_err());
  assert_debug_snapshot!(response);
  Ok(())
}

#[tokio::test]
async fn construction_metadata_empty() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;
  let request = ConstructionMetadataRequest::new(network_id());
  let response = mina_mesh.construction_metadata(request).await;
  assert!(response.is_err());
  assert_debug_snapshot!(response);
  Ok(())
}

#[tokio::test]
async fn construction_metadata_missing_sender() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;
  let request = ConstructionMetadataRequest {
    network_identifier: network_id().into(),
    // cspell:disable-next-line
    options: Some(serde_json::json!({ "receiver": "B62qnXy1f75qq8c6HS2Am88Gk6UyvTHK3iSYh4Hb3nD6DS2eS6wZ4or" })),
    public_keys: None,
  };

  let response = mina_mesh.construction_metadata(request).await;

  assert!(response.is_err());
  assert_debug_snapshot!(response);
  Ok(())
}

#[tokio::test]
async fn construction_metadata_missing_receiver() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;
  let request = ConstructionMetadataRequest {
    network_identifier: network_id().into(),
    // cspell:disable-next-line
    options: Some(serde_json::json!({ "sender": "B62qnXy1f75qq8c6HS2Am88Gk6UyvTHK3iSYh4Hb3nD6DS2eS6wZ4or" })),
    public_keys: None,
  };

  let response = mina_mesh.construction_metadata(request).await;

  assert!(response.is_err());
  assert_debug_snapshot!(response);
  Ok(())
}

#[tokio::test]
async fn construction_metadata_missing_token_id() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;
  let request = ConstructionMetadataRequest {
    network_identifier: network_id().into(),
    options: Some(serde_json::json!({
      // cspell:disable
      "sender": "B62qnXy1f75qq8c6HS2Am88Gk6UyvTHK3iSYh4Hb3nD6DS2eS6wZ4or",
      "receiver": "B62qpLST3UC1rpVT6SHfB7wqW2iQgiopFAGfrcovPgLjgfpDUN2LLeg",
      // cspell:enable
    })),
    public_keys: None,
  };

  let response = mina_mesh.construction_metadata(request).await;

  assert!(response.is_err());
  assert_debug_snapshot!(response);
  Ok(())
}

#[tokio::test]
async fn construction_metadata_invalid_sender() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;
  let request = ConstructionMetadataRequest {
    network_identifier: network_id().into(),
    options: Some(serde_json::json!({
      "sender": "invalid",
      // cspell:disable-next-line
      "receiver": "B62qpLST3UC1rpVT6SHfB7wqW2iQgiopFAGfrcovPgLjgfpDUN2LLeg",
      "token_id": "1",
    })),
    public_keys: None,
  };

  let response = mina_mesh.construction_metadata(request).await;

  assert!(response.is_err());
  assert_debug_snapshot!(response);
  Ok(())
}

#[tokio::test]
async fn construction_metadata_invalid_receiver() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;
  let request = ConstructionMetadataRequest {
    network_identifier: network_id().into(),
    options: Some(serde_json::json!({
      // cspell:disable-next-line
      "sender": "B62qpLST3UC1rpVT6SHfB7wqW2iQgiopFAGfrcovPgLjgfpDUN2LLeg",
      "receiver": "invalid",
      "token_id": "1",
    })),
    public_keys: None,
  };

  let response = mina_mesh.construction_metadata(request).await;

  assert!(response.is_err());
  assert_debug_snapshot!(response);
  Ok(())
}
