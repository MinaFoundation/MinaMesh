use mina_mesh::{
  models::ConstructionPayloadsRequest,
  test::{delegation_operations, network_id, payment_operations},
  TransactionMetadata,
};

use super::CompareGroup;

pub fn construction_payloads<'a>() -> CompareGroup<'a> {
  // cspell:disable
  let sender = "B62qkUHaJUHERZuCHQhXCQ8xsGBqyYSgjQsKnKN5HhSJecakuJ4pYyk";
  let receiver = "B62qoDWfBZUxKpaoQCoFqr12wkaY84FrhxXNXzgBkMUi2Tz4K8kBDiv";
  // cspell:enable
  let payment_operations = payment_operations((sender, "-1010"), (sender, "50000"), (receiver, "-50000"));

  // cspell:disable
  let delegator = "B62qkXajxfnicuCNtaurdAhQpkFsqjoyPJuw53aeJP848bsa3Ne3RvB";
  let delegation_target = "B62qiburnzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzmp7r7UN6X";
  // cspell:enable
  let delegation_operations = delegation_operations(delegator, "-1010000000", delegator, delegation_target);

  let metadata =
    TransactionMetadata::new(sender, receiver, "355", "11", None::<&str>, Some("200009999"), Some("memo test!"));

  ("/construction/payloads", vec![
    Box::new(ConstructionPayloadsRequest {
      network_identifier: network_id().into(),
      operations: payment_operations.clone(),
      metadata: metadata.to_json().into(),
      public_keys: None,
    }),
    Box::new(ConstructionPayloadsRequest {
      network_identifier: network_id().into(),
      operations: delegation_operations.clone(),
      metadata: metadata.to_json().into(),
      public_keys: None,
    }),
  ])
}
