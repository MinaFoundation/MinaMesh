use mina_mesh::{
  models::ConstructionParseRequest,
  test::{
    network_id, signed_transaction_delegation, signed_transaction_payment, unsigned_transaction_delegation,
    unsigned_transaction_payment,
  },
};

use super::CompareGroup;

pub fn construction_parse<'a>() -> CompareGroup<'a> {
  (
    "/construction/parse",
    vec![
      Box::new(ConstructionParseRequest {
        network_identifier: network_id().into(),
        signed: false,
        transaction: unsigned_transaction_payment(),
      }),
      Box::new(ConstructionParseRequest {
        network_identifier: network_id().into(),
        signed: false,
        transaction: unsigned_transaction_delegation(),
      }),
      Box::new(ConstructionParseRequest {
        network_identifier: network_id().into(),
        signed: true,
        transaction: signed_transaction_payment(),
      }),
      Box::new(ConstructionParseRequest {
        network_identifier: network_id().into(),
        signed: true,
        transaction: signed_transaction_delegation(),
      }),
    ],
  )
}
