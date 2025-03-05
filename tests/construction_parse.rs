use anyhow::Result;
use coinbase_mesh::models::ConstructionParseRequest;
use insta::assert_debug_snapshot;
use mina_mesh::{
  test::{
    network_id, signed_transaction_delegation, signed_transaction_payment, unsigned_transaction_delegation,
    unsigned_transaction_payment,
  },
  MinaMeshConfig,
};

#[tokio::test]
async fn construction_parse_valid_signed_payment() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;

  let request = ConstructionParseRequest {
    network_identifier: network_id().into(),
    transaction: signed_transaction_payment(),
    signed: true,
  };

  let response = mina_mesh.construction_parse(request).await;
  assert!(response.is_ok());
  assert_debug_snapshot!(response);
  Ok(())
}

#[tokio::test]
async fn construction_parse_valid_signed_delegation() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;

  let request = ConstructionParseRequest {
    network_identifier: network_id().into(),
    transaction: signed_transaction_delegation(),
    signed: true,
  };

  let response = mina_mesh.construction_parse(request).await;
  assert!(response.is_ok());
  assert_debug_snapshot!(response);
  Ok(())
}

#[tokio::test]
async fn construction_parse_valid_unsigned_payment() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;

  let request = ConstructionParseRequest {
    network_identifier: network_id().into(),
    transaction: unsigned_transaction_payment(),
    signed: false,
  };

  let response = mina_mesh.construction_parse(request).await;
  assert!(response.is_ok());
  assert_debug_snapshot!(response);
  Ok(())
}

#[tokio::test]
async fn construction_parse_valid_unsigned_delegation() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;

  let request = ConstructionParseRequest {
    network_identifier: network_id().into(),
    transaction: unsigned_transaction_delegation(),
    signed: false,
  };

  let response = mina_mesh.construction_parse(request).await;
  assert!(response.is_ok());
  assert_debug_snapshot!(response);
  Ok(())
}

#[tokio::test]
async fn construction_parse_invalid_fee() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;

  let request = ConstructionParseRequest {
    network_identifier: network_id().into(),
    transaction: r#"{
            "signature": "EE1D10B5EF283026177B8C61F75C84F09B35C94C6D1417C2C88707E2D26CBB21D1371F16F3AEC696E055C235D9EA2F707630EB395813AEBFE120BBDD5B5E8908",
            "payment": {
                "to": "B62qm3B76ruDJc4aQJyEw3iTjpHJ9xrTceJosqVFkhoUTbSjEdF3xgU",
                "from": "B62qogmsu15o7DJQkzLtAux6THLrfPeLUJLzpmiBPkEWaYLdWze4Tj9",
                "fee": "10", 
                "token": "1",
                "nonce": "10",
                "memo": "test payment",
                "amount": "100",
                "valid_until": "200000"
            },
            "stake_delegation": null
        }"#
      .to_string(),
    signed: true,
  };

  let response = mina_mesh.construction_parse(request).await;
  assert!(response.is_err());
  assert_debug_snapshot!(response);
  Ok(())
}

#[tokio::test]
async fn construction_parse_invalid_format_signed() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;

  let request = ConstructionParseRequest {
    network_identifier: network_id().into(),
    transaction: r#"{
            "signature": "EE1D10B5EF283026177B8C61F75C84F09B35C94C6D1417C2C88707E2D26CBB21D1371F16F3AEC696E055C235D9EA2F707630EB395813AEBFE120BBDD5B5E8908",
            "payment": null,
            "stake_delegation": null  
        }"#
      .to_string(),
    signed: true,
  };

  let response = mina_mesh.construction_parse(request).await;
  assert!(response.is_err());
  assert_debug_snapshot!(response);
  Ok(())
}

#[tokio::test]
async fn construction_parse_invalid_format_unsigned() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;

  let request = ConstructionParseRequest {
    network_identifier: network_id().into(),
    transaction: r#"{
            "randomOracleInput": "000000033024262EC0279FAE8D8984B4DFF694E3DE1A8513B8DB9E13A2BA190581EC974A3024262EC0279FAE8D8984B4DFF694E3DE1A8513B8DB9E13A2BA190581EC974A5E6737A0AC0A147918437FC8C21EA57CECFB613E711CA2E4FD328401657C291C000002570561800000000000800000000000000081F0000001586000407C0382808784808087848080A1C2A1C287848080C30607848080C10784808086000E0000000000000000013E815200000000",
            "signerInput": { "prefix": [], "suffix": [] },
            "payment": null,
            "stakeDelegation": null  
        }"#
      .to_string(),
    signed: false,
  };

  let response = mina_mesh.construction_parse(request).await;
  assert!(response.is_err());
  assert_debug_snapshot!(response);
  Ok(())
}

#[tokio::test]
async fn construction_parse_invalid_operations_signed() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;

  let request = ConstructionParseRequest {
    network_identifier: network_id().into(),
    transaction: r#"{
            "signature": "FE1D10B5EF283026177B8C61F75C84F09B35C94C6D1417C2C88707E2D26CBB21D1371F16F3AEC696E055C235D9EA2F707630EB395813AEBFE120BBDD5B5E8908",
            "payment": {
                "to": "B62qm21Bx7bV1mGvk5JwBy3QjxTY71MNg6FQNNR6ce4HEHoUGsKRcYA",
                "from": "B62qm21Bx7bV1mGvk5JwBy3QjxTY71MNg6FQNNR6ce4HEHoUGsKRcYQ",
                "fee": "100000000",
                "token": "1",
                "nonce": "10",
                "memo": "test payment",
                "amount": "100",
                "valid_until": "200000"
            },
            "stake_delegation": null
        }"#
      .to_string(),
    signed: true,
  };

  let response = mina_mesh.construction_parse(request).await;
  assert!(response.is_err());
  assert_debug_snapshot!(response);
  Ok(())
}

#[tokio::test]
async fn construction_parse_invalid_operations_unsigned() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;

  let request = ConstructionParseRequest {
    network_identifier: network_id().into(),
    transaction: r#"{
      "randomOracleInput":"000000031BAED3545AE7F7236ED78A3B7F059768D7C6E9A039A512AFF346F1C51F8661941BAED3545AE7F7236ED78A3B7F059768D7C6E9A039A512AFF346F1C51F8661947F3BA0154F672B1E82F84C207B84502A38A8967D61C86727E43054C0FFB59026000002570242F000000000008000000000000000080000000021EBE840500B531B1B7B00000000000000000000000000000000000000000000000000000006000000000000000005A4640000000000",
      "signerInput":{
          "prefix":[
              "1BAED3545AE7F7236ED78A3B7F059768D7C6E9A039A512AFF346F1C51F866194",
              "1BAED3545AE7F7236ED78A3B7F059768D7C6E9A039A512AFF346F1C51F866194",
              "7F3BA0154F672B1E82F84C207B84502A38A8967D61C86727E43054C0FFB59026"
          ],
          "suffix":[
              "01BDB1B195A014042FAF080000000020000000000000000200000000001E8480",
              "0000000003000000000000000000000000000000000000000000000000000000",
              "0000000000000000000000000000000000000000000000000004C4B400000000"
          ]
      },
      "payment":{
          "to":"B62qj7nR7j5GiQLJEBrMq49nX8fKcLJKK57kDdja7w9YPJQMdsshtcL",
          "from":"B62qqEMfUYCW4ePTDE9ZGVfn42ugxQ6CSe8PdTCrRpZAnCRezAGsVp7",
          "fee":"1000000",
          "token":"wSHV2S4qX9jFsLjQo8r1BsMLH2ZRKsZx6EJd1sbozGPieEC4Jf",
          "nonce":"8",
          "memo":"hello",
          "amount":"10777000000",
          "valid_until":"200000000"
      },
      "stakeDelegation":null
    }"#.to_string(),
    signed: false,
  };

  let response = mina_mesh.construction_parse(request).await;
  assert!(response.is_err());
  assert_debug_snapshot!(response);
  Ok(())
}
