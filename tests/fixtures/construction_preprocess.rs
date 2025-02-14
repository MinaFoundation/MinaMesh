use mina_mesh::{
  models::ConstructionPreprocessRequest,
  test::{delegation_operations, network_id, payment_operations},
  PreprocessMetadata,
};

use super::CompareGroup;

pub fn construction_preprocess<'a>() -> CompareGroup<'a> {
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
