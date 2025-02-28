use mina_mesh::{
  models::{ConstructionCombineRequest, SignatureType},
  test::{network_id, signature, unsigned_transaction_delegation, unsigned_transaction_payment},
};

use super::CompareGroup;

pub fn construction_combine<'a>() -> CompareGroup<'a> {
  // cspell:disable
  // Signatures produced via ocaml-signer
  // pk=`signer.exe generate-private-key`
  // signer.exe sign -unsigned-transaction xxx -private-key `pk`
  let signature_hex_1 = "52DA947A59B79B62FB0E42BDB49390FFF43AA2997DFC415B78CD3097E15D0221D807069A35BE13D62A5B45F8590A7CC8684E45B076F8BC5F1E711442FA1A6506";
  let signature_hex_2 = "549E0B6AD43D1E894EBEE9255FEDC1C248CC947F0B548FE309ADDF5C35A95E2783390A8A5EA76561FB0EFEA9F12999FD3D6F9C41FEFEDB7D1D953C1F1861F412";
  // cspell:enable

  ("/construction/combine", vec![
    Box::new(ConstructionCombineRequest {
      network_identifier: network_id().into(),
      signatures: vec![signature(signature_hex_1, SignatureType::SchnorrPoseidon)],
      unsigned_transaction: unsigned_transaction_payment(),
    }),
    Box::new(ConstructionCombineRequest {
      network_identifier: network_id().into(),
      signatures: vec![signature(signature_hex_2, SignatureType::SchnorrPoseidon)],
      unsigned_transaction: unsigned_transaction_delegation(),
    }),
  ])
}
