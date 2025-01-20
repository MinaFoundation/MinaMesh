use mina_mesh::{
  models::{AccountIdentifier, Amount, ConstructionPreprocessRequest, Currency, Operation, OperationIdentifier},
  test::network_id,
  OperationType::*,
  PreprocessMetadata,
};
use serde_json::json;

use super::CompareGroup;

pub fn construction_preprocess<'a>() -> CompareGroup<'a> {
  let payment_operations = vec![
    Operation {
      operation_identifier: OperationIdentifier::new(0).into(),
      related_operations: None,
      r#type: FeePayment.to_string(),
      account: Some(
        AccountIdentifier {
          address: "B62qkUHaJUHERZuCHQhXCQ8xsGBqyYSgjQsKnKN5HhSJecakuJ4pYyk".into(),
          sub_account: None,
          metadata: json!({ "token_id": "1" }).into(),
        }
        .into(),
      ),
      amount: Some(Box::new(Amount::new("-100000".into(), Currency::new("MINA".into(), 9)))),
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
          address: "B62qkUHaJUHERZuCHQhXCQ8xsGBqyYSgjQsKnKN5HhSJecakuJ4pYyk".into(),
          sub_account: None,
          metadata: json!({ "token_id": "1" }).into(),
        }
        .into(),
      ),
      amount: Some(Box::new(Amount::new("-9000000".into(), Currency::new("MINA".into(), 9)))),
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
          address: "B62qoDWfBZUxKpaoQCoFqr12wkaY84FrhxXNXzgBkMUi2Tz4K8kBDiv".into(),
          sub_account: None,
          metadata: json!({ "token_id": "1" }).into(),
        }
        .into(),
      ),
      amount: Some(Box::new(Amount::new("9000000".into(), Currency::new("MINA".into(), 9)))),
      coin_change: None,
      status: None,
      metadata: None,
    },
  ];

  let delegation_operations = vec![
    Operation {
      operation_identifier: OperationIdentifier::new(0).into(),
      related_operations: None,
      r#type: FeePayment.to_string(),
      account: Some(AccountIdentifier::new("B62qoDWfBZUxKpaoQCoFqr12wkaY84FrhxXNXzgBkMUi2Tz4K8kBDiv".into()).into()),
      amount: Some(Box::new(Amount::new("-500".into(), Currency::new("MINA".into(), 9)))),
      coin_change: None,
      metadata: None,
      status: None,
    },
    Operation {
      operation_identifier: OperationIdentifier::new(1).into(),
      related_operations: None,
      r#type: DelegateChange.to_string(),
      account: Some(AccountIdentifier::new("B62qoDWfBZUxKpaoQCoFqr12wkaY84FrhxXNXzgBkMUi2Tz4K8kBDiv".into()).into()),
      amount: None,
      coin_change: None,
      metadata: Some(json!({
          "delegate_change_target": "B62qiburnzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzmp7r7UN6X"
      })),
      status: None,
    },
  ];

  let metadata = PreprocessMetadata::new(Some("70000".into()), Some("test memo OK".into()));

  ("/construction/preprocess", vec![
    Box::new(ConstructionPreprocessRequest::new(network_id(), payment_operations.clone())),
    Box::new(ConstructionPreprocessRequest::new(network_id(), delegation_operations.clone())),
    Box::new(ConstructionPreprocessRequest {
      network_identifier: network_id().into(),
      operations: payment_operations,
      metadata: Some(metadata.to_json()),
    }),
    Box::new(ConstructionPreprocessRequest {
      network_identifier: network_id().into(),
      operations: delegation_operations,
      metadata: Some(metadata.to_json()),
    }),
  ])
}
