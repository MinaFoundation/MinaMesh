use anyhow::Result;
use insta::assert_debug_snapshot;
use mina_mesh::{
  models::{AccountIdentifier, Amount, ConstructionPreprocessRequest, Currency, Operation, OperationIdentifier},
  test::network_id,
  MinaMeshConfig,
  OperationType::*,
  PreprocessMetadata,
};
use serde_json::json;

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

fn payment_operations(
  (fee_act, fee_amt): (&str, &str),
  (sender_act, sender_amt): (&str, &str),
  (receiver_act, receiver_amt): (&str, &str),
) -> Vec<Operation> {
  vec![
    Operation {
      operation_identifier: OperationIdentifier::new(0).into(),
      related_operations: None,
      r#type: FeePayment.to_string(),
      account: Some(
        AccountIdentifier { address: fee_act.into(), sub_account: None, metadata: json!({ "token_id": "1" }).into() }
          .into(),
      ),
      amount: Some(Box::new(Amount::new(fee_amt.into(), Currency::new("MINA".into(), 9)))),
      coin_change: None,
      metadata: None,
      status: None,
    },
    Operation {
      operation_identifier: OperationIdentifier::new(1).into(),
      related_operations: None,
      r#type: PaymentSourceDec.to_string(),
      account: Some(
        AccountIdentifier {
          address: sender_act.into(),
          sub_account: None,
          metadata: json!({ "token_id": "1" }).into(),
        }
        .into(),
      ),
      amount: Some(Box::new(Amount::new(sender_amt.into(), Currency::new("MINA".into(), 9)))),
      coin_change: None,
      metadata: None,
      status: None,
    },
    Operation {
      operation_identifier: OperationIdentifier::new(2).into(),
      related_operations: vec![OperationIdentifier::new(1)].into(),
      r#type: PaymentReceiverInc.to_string(),
      account: Some(
        AccountIdentifier {
          address: receiver_act.into(),
          sub_account: None,
          metadata: json!({ "token_id": "1" }).into(),
        }
        .into(),
      ),
      amount: Some(Box::new(Amount::new(receiver_amt.into(), Currency::new("MINA".into(), 9)))),
      coin_change: None,
      status: None,
      metadata: None,
    },
  ]
}

fn delegation_operations(fee_act: &str, fee_amt: &str, source_act: &str, delegate_target_act: &str) -> Vec<Operation> {
  vec![
    Operation {
      operation_identifier: OperationIdentifier::new(0).into(),
      related_operations: None,
      r#type: FeePayment.to_string(),
      account: Some(AccountIdentifier::new(fee_act.into()).into()),
      amount: Some(Box::new(Amount::new(fee_amt.into(), Currency::new("MINA".into(), 9)))),
      coin_change: None,
      metadata: None,
      status: None,
    },
    Operation {
      operation_identifier: OperationIdentifier::new(1).into(),
      related_operations: None,
      r#type: DelegateChange.to_string(),
      account: Some(AccountIdentifier::new(source_act.into()).into()),
      amount: None,
      coin_change: None,
      metadata: Some(json!({
          "delegate_change_target": delegate_target_act
      })),
      status: None,
    },
  ]
}
