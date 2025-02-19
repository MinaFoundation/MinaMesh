use mina_mesh::{
  models::{ConstructionCombineRequest, CurveType, PublicKey, Signature, SignatureType, SigningPayload},
  test::network_id,
  TransactionUnsigned,
};

use super::CompareGroup;

pub fn construction_combine<'a>() -> CompareGroup<'a> {
  // cspell:disable
  let unsigned_tx_payment = TransactionUnsigned::from_json_string("{\"randomOracleInput\":\"000000035E6737A0AC0A147918437FC8C21EA57CECFB613E711CA2E4FD328401657C291C5E6737A0AC0A147918437FC8C21EA57CECFB613E711CA2E4FD328401657C291C5E6737A0AC0A147918437FC8C21EA57CECFB613E711CA2E4FD328401657C291C000002570561800000000000800000000000000081F0000001586000401013570767000000000000000000000000000000000000000000000000000000000E0000000000000000013E815200000000\",\"signerInput\":{\"prefix\":[\"5E6737A0AC0A147918437FC8C21EA57CECFB613E711CA2E4FD328401657C291C\",\"5E6737A0AC0A147918437FC8C21EA57CECFB613E711CA2E4FD328401657C291C\",\"5E6737A0AC0A147918437FC8C21EA57CECFB613E711CA2E4FD328401657C291C\"],\"suffix\":[\"0001CDC1D5901004000C350000001F0200000000000000020000000000030D40\",\"0000000003800000000000000000000000000000000000000000000000000000\",\"00000000000000000000000000000000000000000000000009502F9000000000\"]},\"payment\":{\"to\":\"B62qoDWfBZUxKpaoQCoFqr12wkaY84FrhxXNXzgBkMUi2Tz4K8kBDiv\",\"from\":\"B62qoDWfBZUxKpaoQCoFqr12wkaY84FrhxXNXzgBkMUi2Tz4K8kBDiv\",\"fee\":\"100000\",\"token\":\"1\",\"nonce\":\"1984\",\"memo\":\"dups\",\"amount\":\"5000000000\",\"valid_until\":\"200000\"},\"stakeDelegation\":null}").expect("Payment deserialization failure");
  let unsigned_tx_delegation = TransactionUnsigned::from_json_string("{\"randomOracleInput\":\"0000000334411FBC9BF58536335A3B711494DA6EC9916AFC520F389B66D00796DCD9BA7A34411FBC9BF58536335A3B711494DA6EC9916AFC520F389B66D00796DCD9BA7A0131D887E9AE69AF4D40469B25411CB7EB94CDAD60E23B71E608B0A58FBCD4080000025704B85900000000008000000000000000E00000000158600040500B531B1B7B0000000000000000000000000000000000000000000000000000001A00000000000000000000000000000000\",\"signerInput\":{\"prefix\":[\"34411FBC9BF58536335A3B711494DA6EC9916AFC520F389B66D00796DCD9BA7A\",\"34411FBC9BF58536335A3B711494DA6EC9916AFC520F389B66D00796DCD9BA7A\",\"0131D887E9AE69AF4D40469B25411CB7EB94CDAD60E23B71E608B0A58FBCD408\"],\"suffix\":[\"01BDB1B195A01404000C35000000000E00000000000000020000000001343A40\",\"0000000002C00000000000000000000000000000000000000000000000000000\",\"0000000000000000000000000000000000000000000000000000000000000000\"]},\"payment\":null,\"stakeDelegation\":{\"delegator\":\"B62qkXajxfnicuCNtaurdAhQpkFsqjoyPJuw53aeJP848bsa3Ne3RvB\",\"new_delegate\":\"B62qiburnzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzmp7r7UN6X\",\"fee\":\"10100000\",\"nonce\":\"3\",\"memo\":\"hello\",\"valid_until\":\"200000\"}}").expect("Delegation deserialization failure");
  // Signatures produced via ocaml-signer
  // pk=`signer.exe generate-private-key`
  // signer.exe sign -unsigned-transaction xxx -private-key `pk`
  let signature_hex_1 = "52DA947A59B79B62FB0E42BDB49390FFF43AA2997DFC415B78CD3097E15D0221D807069A35BE13D62A5B45F8590A7CC8684E45B076F8BC5F1E711442FA1A6506";
  let signature_hex_2 = "549E0B6AD43D1E894EBEE9255FEDC1C248CC947F0B548FE309ADDF5C35A95E2783390A8A5EA76561FB0EFEA9F12999FD3D6F9C41FEFEDB7D1D953C1F1861F412";
  // cspell:enable

  ("/construction/combine", vec![
    Box::new(ConstructionCombineRequest {
      network_identifier: network_id().into(),
      signatures: vec![signature(signature_hex_1)],
      unsigned_transaction: unsigned_tx_payment.as_json_string().unwrap().into(),
    }),
    Box::new(ConstructionCombineRequest {
      network_identifier: network_id().into(),
      signatures: vec![signature(signature_hex_2)],
      unsigned_transaction: unsigned_tx_delegation.as_json_string().unwrap().into(),
    }),
  ])
}

fn signature(sig_hex: &str) -> Signature {
  Signature {
    signing_payload: SigningPayload::new("xxx".to_owned()).into(),
    public_key: PublicKey::new("xxx".to_owned(), CurveType::Tweedle).into(),
    signature_type: SignatureType::SchnorrPoseidon,
    hex_bytes: sig_hex.into(),
  }
}
