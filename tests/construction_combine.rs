use anyhow::Result;
use coinbase_mesh::models::{ConstructionCombineRequest, Signature};
use insta::assert_debug_snapshot;
use mina_mesh::{
  models::{CurveType, PublicKey, SignatureType, SigningPayload},
  test::network_id,
  MinaMeshConfig, MinaMeshError,
};

#[tokio::test]
async fn construction_combine_no_signatures() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;
  let request = ConstructionCombineRequest {
    network_identifier: network_id().into(),
    unsigned_transaction: unsigned_transaction_payment(),
    signatures: vec![],
  };
  let response = mina_mesh.construction_combine(request).await;
  assert!(response.is_err());
  assert_debug_snapshot!(response);
  Ok(())
}

#[tokio::test]
async fn construction_combine_invalid_signature_type() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;
  let sig_hex = "CA5B636101409503297B02AB94DE28FACBD53C91919A77E57CCBEDBF9D6C2D1893FDE9B63BF44618270F6D404B7C4B783BB322B05C9347BEF238FDB841BF3320";
  let request = ConstructionCombineRequest {
    network_identifier: network_id().into(),
    unsigned_transaction: unsigned_transaction_payment(),
    signatures: vec![signature(sig_hex, SignatureType::Ecdsa)], // Not SchnorrPoseidon
  };
  let response = mina_mesh.construction_combine(request).await;
  assert!(matches!(response, Err(MinaMeshError::SignatureInvalid(_))));
  assert_debug_snapshot!(response);
  Ok(())
}

#[tokio::test]
async fn construction_combine_invalid_signature_format() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;
  let sig_hex = "ZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZ";
  let request = ConstructionCombineRequest {
    network_identifier: network_id().into(),
    unsigned_transaction: unsigned_transaction_payment(),
    signatures: vec![signature(sig_hex, SignatureType::SchnorrPoseidon)],
  };
  let response = mina_mesh.construction_combine(request).await;
  assert!(matches!(response, Err(MinaMeshError::SignatureInvalid(_))));
  assert_debug_snapshot!(response);
  Ok(())
}

#[tokio::test]
async fn construction_combine_valid_payment() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;
  let sig_hex = "CA5B636101409503297B02AB94DE28FACBD53C91919A77E57CCBEDBF9D6C2D1893FDE9B63BF44618270F6D404B7C4B783BB322B05C9347BEF238FDB841BF3320";
  let request = ConstructionCombineRequest {
    network_identifier: network_id().into(),
    unsigned_transaction: unsigned_transaction_payment(),
    signatures: vec![signature(sig_hex, SignatureType::SchnorrPoseidon)],
  };
  let response = mina_mesh.construction_combine(request).await;
  assert!(response.is_ok());
  assert_debug_snapshot!(response);
  Ok(())
}

#[tokio::test]
async fn construction_combine_valid_delegation() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;
  let sig_hex = "CA5B636101409503297B02AB94DE28FACBD53C91919A77E57CCBEDBF9D6C2D1893FDE9B63BF44618270F6D404B7C4B783BB322B05C9347BEF238FDB841BF3320";
  let request = ConstructionCombineRequest {
    network_identifier: network_id().into(),
    unsigned_transaction: unsigned_transaction_delegation(),
    signatures: vec![signature(sig_hex, SignatureType::SchnorrPoseidon)],
  };
  let response = mina_mesh.construction_combine(request).await;
  assert!(response.is_ok());
  assert_debug_snapshot!(response);
  Ok(())
}

fn unsigned_transaction_payment() -> String {
  r#"{
      "randomOracleInput": "000000035E6737A0AC0A147918437FC8C21EA57CECFB613E711CA2E4FD328401657C291C5E6737A0AC0A147918437FC8C21EA57CECFB613E711CA2E4FD328401657C291C5E6737A0AC0A147918437FC8C21EA57CECFB613E711CA2E4FD328401657C291C000002570561800000000000800000000000000081F0000001586000401013570767000000000000000000000000000000000000000000000000000000000E0000000000000000013E815200000000",
      "signerInput": {
          "prefix": [
              "5E6737A0AC0A147918437FC8C21EA57CECFB613E711CA2E4FD328401657C291C",
              "5E6737A0AC0A147918437FC8C21EA57CECFB613E711CA2E4FD328401657C291C",
              "5E6737A0AC0A147918437FC8C21EA57CECFB613E711CA2E4FD328401657C291C"
          ],
          "suffix": [
              "0001CDC1D5901004000C350000001F0200000000000000020000000000030D40",
              "0000000003800000000000000000000000000000000000000000000000000000",
              "00000000000000000000000000000000000000000000000009502F9000000000"
          ]
      },
      "payment": {
          "to": "B62qoDWfBZUxKpaoQCoFqr12wkaY84FrhxXNXzgBkMUi2Tz4K8kBDiv",
          "from": "B62qoDWfBZUxKpaoQCoFqr12wkaY84FrhxXNXzgBkMUi2Tz4K8kBDiv",
          "fee": "100000",
          "token": "1",
          "nonce": "1984",
          "memo": "dups",
          "amount": "5000000000",
          "valid_until": "200000"
      },
      "stakeDelegation": null
  }"#.to_string()
}

pub fn unsigned_transaction_delegation() -> String {
  r#"{
      "randomOracleInput": "0000000334411FBC9BF58536335A3B711494DA6EC9916AFC520F389B66D00796DCD9BA7A34411FBC9BF58536335A3B711494DA6EC9916AFC520F389B66D00796DCD9BA7A0131D887E9AE69AF4D40469B25411CB7EB94CDAD60E23B71E608B0A58FBCD4080000025704B85900000000008000000000000000E00000000158600040500B531B1B7B0000000000000000000000000000000000000000000000000000001A00000000000000000000000000000000",
      "signerInput": {
          "prefix": [
              "34411FBC9BF58536335A3B711494DA6EC9916AFC520F389B66D00796DCD9BA7A",
              "34411FBC9BF58536335A3B711494DA6EC9916AFC520F389B66D00796DCD9BA7A",
              "0131D887E9AE69AF4D40469B25411CB7EB94CDAD60E23B71E608B0A58FBCD408"
          ],
          "suffix": [
              "01BDB1B195A01404000C35000000000E00000000000000020000000001343A40",
              "0000000002C00000000000000000000000000000000000000000000000000000",
              "0000000000000000000000000000000000000000000000000000000000000000"
          ]
      },
      "payment": null,
      "stakeDelegation": {
          "delegator": "B62qkXajxfnicuCNtaurdAhQpkFsqjoyPJuw53aeJP848bsa3Ne3RvB",
          "new_delegate": "B62qiburnzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzmp7r7UN6X",
          "fee": "10100000",
          "nonce": "3",
          "memo": "hello",
          "valid_until": "200000"
      }
  }"#.to_string()
}

fn signature(sig_hex: &str, signature_type: SignatureType) -> Signature {
  Signature {
    signing_payload: SigningPayload::new("xxx".to_owned()).into(),
    public_key: PublicKey::new("xxx".to_owned(), CurveType::Tweedle).into(),
    signature_type,
    hex_bytes: sig_hex.into(),
  }
}
