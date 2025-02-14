use anyhow::Result;
use insta::assert_debug_snapshot;
use mina_mesh::{
  models::ConstructionPreprocessRequest,
  test::{delegation_operations, network_id, payment_operations},
  MinaMeshConfig, PreprocessMetadata,
};

#[tokio::test]
async fn construction_preprocess_empty() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;
  let request = ConstructionPreprocessRequest::new(network_id(), vec![]);
  let response = mina_mesh.construction_preprocess(request).await;
  assert!(response.is_err());
  assert_debug_snapshot!(response);
  Ok(())
}

#[tokio::test]
async fn construction_preprocess_payment() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;
  let operations = payment_operations(
    // cspell:disable
    ("B62qkUHaJUHERZuCHQhXCQ8xsGBqyYSgjQsKnKN5HhSJecakuJ4pYyk", "-1010"),
    ("B62qkUHaJUHERZuCHQhXCQ8xsGBqyYSgjQsKnKN5HhSJecakuJ4pYyk", "50000"),
    ("B62qoDWfBZUxKpaoQCoFqr12wkaY84FrhxXNXzgBkMUi2Tz4K8kBDiv", "-50000"),
    // cspell:enable
  );
  let request = ConstructionPreprocessRequest::new(network_id(), operations);
  let response = mina_mesh.construction_preprocess(request).await;
  assert!(response.is_ok());
  assert_debug_snapshot!(response);
  Ok(())
}

#[tokio::test]
async fn construction_preprocess_payment_with_metadata() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;
  let operations = payment_operations(
    // cspell:disable
    ("B62qkUHaJUHERZuCHQhXCQ8xsGBqyYSgjQsKnKN5HhSJecakuJ4pYyk", "-1010"),
    ("B62qkUHaJUHERZuCHQhXCQ8xsGBqyYSgjQsKnKN5HhSJecakuJ4pYyk", "50000"),
    ("B62qoDWfBZUxKpaoQCoFqr12wkaY84FrhxXNXzgBkMUi2Tz4K8kBDiv", "-50000"),
    // cspell:enable
  );
  let metadata = PreprocessMetadata::new(Some("20000".into()), Some("hello".into()));
  let request = ConstructionPreprocessRequest {
    network_identifier: network_id().into(),
    operations,
    metadata: Some(metadata.to_json()),
  };
  let response = mina_mesh.construction_preprocess(request).await;
  assert!(response.is_ok());
  assert_debug_snapshot!(response);
  Ok(())
}

#[tokio::test]
async fn construction_preprocess_payment_fee_not_negative() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;
  let operations = payment_operations(
    // cspell:disable
    ("B62qkUHaJUHERZuCHQhXCQ8xsGBqyYSgjQsKnKN5HhSJecakuJ4pYyk", "1010"),
    ("B62qkUHaJUHERZuCHQhXCQ8xsGBqyYSgjQsKnKN5HhSJecakuJ4pYyk", "50000"),
    ("B62qoDWfBZUxKpaoQCoFqr12wkaY84FrhxXNXzgBkMUi2Tz4K8kBDiv", "-50000"),
    // cspell:enable
  );
  let request = ConstructionPreprocessRequest::new(network_id(), operations);
  let response = mina_mesh.construction_preprocess(request).await;
  assert!(response.is_err());
  assert_debug_snapshot!(response);
  Ok(())
}

#[tokio::test]
async fn construction_preprocess_payment_dec_inc_mismatch() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;
  let operations = payment_operations(
    // cspell:disable
    ("B62qkUHaJUHERZuCHQhXCQ8xsGBqyYSgjQsKnKN5HhSJecakuJ4pYyk", "-1010"),
    ("B62qkUHaJUHERZuCHQhXCQ8xsGBqyYSgjQsKnKN5HhSJecakuJ4pYyk", "50000"),
    ("B62qoDWfBZUxKpaoQCoFqr12wkaY84FrhxXNXzgBkMUi2Tz4K8kBDiv", "50000"),
    // cspell:enable
  );
  let request = ConstructionPreprocessRequest::new(network_id(), operations);
  let response = mina_mesh.construction_preprocess(request).await;
  assert!(response.is_err());
  assert_debug_snapshot!(response);
  Ok(())
}

#[tokio::test]
async fn construction_preprocess_payment_invalid_pk() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;
  let operations = payment_operations(
    // cspell:disable
    ("B62qkUHaJUHERZuCHQhXCQ8xsGBqyYSgjQsKnKN5HhSJecakuJ4pYyk", "-1010"),
    ("B62qkUHaJUHERZuCHQhXCQ8xsGBqyYSgjQsKnKN5HhSJecakuJ4pYyk", "50000"),
    ("B62qoDWfBZUxKpaoQCoFqr12wkaY84FrhxXNXzgBkMUi2Tz4K8kBDivkk", "-50000"),
    // cspell:enable
  );
  let request = ConstructionPreprocessRequest::new(network_id(), operations);
  let response = mina_mesh.construction_preprocess(request).await;
  assert!(response.is_err());
  assert_debug_snapshot!(response);
  Ok(())
}

#[tokio::test]
async fn construction_preprocess_payment_fee_payer_mismatch() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;
  let operations = payment_operations(
    // cspell:disable
    ("B62qkUHaJUHERZuCHQhXCQ8xsGBqyYSgjQsKnKN5HhSJecakuJ4pYyk", "-1010"),
    ("B62qoDWfBZUxKpaoQCoFqr12wkaY84FrhxXNXzgBkMUi2Tz4K8kBDiv", "50000"),
    ("B62qoDWfBZUxKpaoQCoFqr12wkaY84FrhxXNXzgBkMUi2Tz4K8kBDiv", "-50000"),
    // cspell:enable
  );
  let request = ConstructionPreprocessRequest::new(network_id(), operations);
  let response = mina_mesh.construction_preprocess(request).await;
  assert!(response.is_err());
  assert_debug_snapshot!(response);
  Ok(())
}

#[tokio::test]
async fn construction_preprocess_delegation() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;
  let operations = delegation_operations(
    // cspell:disable
    "B62qkXajxfnicuCNtaurdAhQpkFsqjoyPJuw53aeJP848bsa3Ne3RvB",
    "-10100000",
    "B62qkXajxfnicuCNtaurdAhQpkFsqjoyPJuw53aeJP848bsa3Ne3RvB",
    "B62qiburnzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzmp7r7UN6X",
    // cspell:enable
  );
  let request = ConstructionPreprocessRequest::new(network_id(), operations);
  let response = mina_mesh.construction_preprocess(request).await;
  assert!(response.is_ok());
  assert_debug_snapshot!(response);
  Ok(())
}

#[tokio::test]
async fn construction_preprocess_delegation_with_metadata() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;
  let operations = delegation_operations(
    // cspell:disable
    "B62qkXajxfnicuCNtaurdAhQpkFsqjoyPJuw53aeJP848bsa3Ne3RvB",
    "-10100000",
    "B62qkXajxfnicuCNtaurdAhQpkFsqjoyPJuw53aeJP848bsa3Ne3RvB",
    "B62qiburnzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzmp7r7UN6X",
    // cspell:enable
  );
  let metadata = PreprocessMetadata::new(Some("20000".into()), Some("hello".into()));
  let request = ConstructionPreprocessRequest {
    network_identifier: network_id().into(),
    operations,
    metadata: Some(metadata.to_json()),
  };
  let response = mina_mesh.construction_preprocess(request).await;
  assert!(response.is_ok());
  assert_debug_snapshot!(response);
  Ok(())
}

#[tokio::test]
async fn construction_preprocess_delegation_fee_not_negative() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;
  let operations = delegation_operations(
    // cspell:disable
    "B62qkXajxfnicuCNtaurdAhQpkFsqjoyPJuw53aeJP848bsa3Ne3RvB",
    "10100000",
    "B62qkXajxfnicuCNtaurdAhQpkFsqjoyPJuw53aeJP848bsa3Ne3RvB",
    "B62qiburnzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzmp7r7UN6X",
    // cspell:enable
  );
  let request = ConstructionPreprocessRequest::new(network_id(), operations);
  let response = mina_mesh.construction_preprocess(request).await;
  assert!(response.is_err());
  assert_debug_snapshot!(response);
  Ok(())
}

#[tokio::test]
async fn construction_preprocess_delegation_fee_amt_invalid() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;
  let operations = delegation_operations(
    // cspell:disable
    "B62qkXajxfnicuCNtaurdAhQpkFsqjoyPJuw53aeJP848bsa3Ne3RvB",
    "xxxx",
    "B62qkXajxfnicuCNtaurdAhQpkFsqjoyPJuw53aeJP848bsa3Ne3RvB",
    "B62qiburnzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzmp7r7UN6X",
    // cspell:enable
  );
  let request = ConstructionPreprocessRequest::new(network_id(), operations);
  let response = mina_mesh.construction_preprocess(request).await;
  assert!(response.is_err());
  assert_debug_snapshot!(response);
  Ok(())
}

#[tokio::test]
async fn construction_preprocess_delegation_fee_payer_mismatch() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;
  let operations = delegation_operations(
    // cspell:disable
    "B62qkXajxfnicuCNtaurdAhQpkFsqjoyPJuw53aeJP848bsa3Ne3RvB",
    "-1",
    "B62qiburnzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzmp7r7UN6X",
    "B62qiburnzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzmp7r7UN6X",
    // cspell:enable
  );
  let request = ConstructionPreprocessRequest::new(network_id(), operations);
  let response = mina_mesh.construction_preprocess(request).await;
  assert!(response.is_err());
  assert_debug_snapshot!(response);
  Ok(())
}

#[tokio::test]
async fn construction_preprocess_delegation_invalid_pk() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;
  let operations = delegation_operations(
    // cspell:disable
    "B62qkXajxfnicuCNtaurdAhQpkFsqjoyPJuw53aeJP848bsa3Ne3RvBx",
    "-1",
    "B62qkXajxfnicuCNtaurdAhQpkFsqjoyPJuw53aeJP848bsa3Ne3RvBx",
    "B62qiburnzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzmp7r7UN6X",
    // cspell:enable
  );
  let request = ConstructionPreprocessRequest::new(network_id(), operations);
  let response = mina_mesh.construction_preprocess(request).await;
  assert!(response.is_err());
  assert_debug_snapshot!(response);
  Ok(())
}
