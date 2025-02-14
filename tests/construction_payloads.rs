use anyhow::Result;
use coinbase_mesh::models::ConstructionPayloadsRequest;
use insta::assert_debug_snapshot;
use mina_mesh::{
  test::{delegation_operations, network_id, payment_operations},
  MinaMeshConfig, TransactionMetadata,
};

#[tokio::test]
async fn construction_payloads_no_metadata() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;
  let request = ConstructionPayloadsRequest {
    network_identifier: network_id().into(),
    operations: vec![],
    metadata: None,
    public_keys: None,
  };
  let response = mina_mesh.construction_payloads(request).await;
  assert!(response.is_err());
  assert_debug_snapshot!(response);
  Ok(())
}

#[tokio::test]
async fn construction_payloads_empty() -> Result<()> {
  let metadata = TransactionMetadata::new("sender", "receiver", "3", "1", None::<&str>, None::<&str>, None::<&str>);
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;
  let request = ConstructionPayloadsRequest {
    network_identifier: network_id().into(),
    operations: vec![],
    metadata: Some(metadata.to_json()),
    public_keys: None,
  };
  let response = mina_mesh.construction_payloads(request).await;
  assert!(response.is_err());
  assert_debug_snapshot!(response);
  Ok(())
}

#[tokio::test]
async fn construction_payloads_payments() -> Result<()> {
  // cspell:disable
  let sender = "B62qkUHaJUHERZuCHQhXCQ8xsGBqyYSgjQsKnKN5HhSJecakuJ4pYyk";
  let receiver = "B62qoDWfBZUxKpaoQCoFqr12wkaY84FrhxXNXzgBkMUi2Tz4K8kBDiv";
  // cspell:enable
  let metadata =
    TransactionMetadata::new(sender, receiver, "3", "1", None::<&str>, Some("20000"), Some("hello, memo test here!"));
  let operations = payment_operations((sender, "-1010"), (sender, "50000"), (receiver, "-50000"));
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;
  let request = ConstructionPayloadsRequest {
    network_identifier: network_id().into(),
    operations,
    metadata: Some(metadata.to_json()),
    public_keys: None,
  };
  let response = mina_mesh.construction_payloads(request).await;
  assert!(response.is_ok());
  assert_debug_snapshot!(response);
  Ok(())
}

#[tokio::test]
async fn construction_payloads_delegation() -> Result<()> {
  // cspell:disable
  let delegator = "B62qkXajxfnicuCNtaurdAhQpkFsqjoyPJuw53aeJP848bsa3Ne3RvB";
  let deleg_target = "B62qiburnzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzmp7r7UN6X";
  // cspell:enable
  let metadata = TransactionMetadata::new(
    delegator,
    deleg_target,
    "355",
    "11",
    None::<&str>,
    Some("200009999"),
    Some("memo test here for delegation!"),
  );
  let operations = delegation_operations(delegator, "-1010000000", delegator, deleg_target);
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;
  let request = ConstructionPayloadsRequest {
    network_identifier: network_id().into(),
    operations,
    metadata: Some(metadata.to_json()),
    public_keys: None,
  };
  let response = mina_mesh.construction_payloads(request).await;
  // assert!(response.is_ok());
  assert_debug_snapshot!(response);
  Ok(())
}
