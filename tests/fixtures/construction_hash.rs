use mina_mesh::{
  models::ConstructionHashRequest,
  test::{network_id, signed_transaction_delegation, signed_transaction_payment},
};

use super::CompareGroup;

pub fn construction_hash<'a>() -> CompareGroup<'a> {
  ("/construction/hash", vec![
    Box::new(ConstructionHashRequest {
      network_identifier: network_id().into(),
      signed_transaction: signed_transaction_payment(),
    }),
    Box::new(ConstructionHashRequest {
      network_identifier: network_id().into(),
      signed_transaction: signed_transaction_delegation(),
    }),
  ])
}
