use anyhow::Result;
use coinbase_mesh::models::ConstructionSubmitRequest;
use insta::assert_debug_snapshot;
use mina_mesh::{test::network_id, MinaMeshConfig};

#[tokio::test]
async fn construction_submit_empty() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;

  let request = ConstructionSubmitRequest {
    network_identifier: network_id().into(),
    signed_transaction: serde_json::to_string(&serde_json::json!({})).unwrap(),
  };

  let response = mina_mesh.construction_submit(request).await;
  assert!(response.is_err());
  assert_debug_snapshot!(response);
  Ok(())
}

#[tokio::test]
async fn construction_submit_payment_and_delegation() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;

  let request = ConstructionSubmitRequest {
    network_identifier: network_id().into(),
    // cspell:disable
    signed_transaction: r#"{
      "signature": "89B921906609910B2EADC6735481B3D3D3E732542E2F1A208505C841C2A6A9036F53E64B2C25017E8E793D8C252FFD374FE442FC887F4D0FB1C6D44506872F12",
      "payment": {
          "to": "B62qj7nR7j5GiQLJEBrMq49nX8fKcLJKK57kDdja7w9YPJQMdsshtcL",
          "from": "B62qnuDyj65AQfcZt3MvcwSX7ohLcP1ayNQwu9Zi6YH7JQddJnc8mkz",
          "fee": "100000000",
          "token": "1",
          "nonce": "60",
          "memo": "hello",
          "amount": "1",
          "valid_until": null
      },
      "stake_delegation": {
          "delegator": "B62qnuDyj65AQfcZt3MvcwSX7ohLcP1ayNQwu9Zi6YH7JQddJnc8mkz",
          "new_delegate": "B62qj7nR7j5GiQLJEBrMq49nX8fKcLJKK57kDdja7w9YPJQMdsshtcL",
          "fee": "100000000",
          "nonce": "60",
          "memo": "hello",
          "valid_until": null
      }
  }"#.to_string()
  // cspell:enable
  };

  let response = mina_mesh.construction_submit(request).await;
  assert!(response.is_err());
  assert_debug_snapshot!(response);
  Ok(())
}

#[tokio::test]
async fn construction_submit_invalid_signature_payment() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;

  let request = ConstructionSubmitRequest {
    network_identifier: network_id().into(),
    // cspell:disable
    signed_transaction: r#"{
      "signature": "4AFD625E5A69575B98ED59C7BF84636CB635FA63BADDC061AC3740B3283A7D2A72735EA4F99ED2E7CCBFBFC92B49DB0B48E13EEA8912C4A837B5759F2746B723",
      "payment": {
          "to": "B62qj7nR7j5GiQLJEBrMq49nX8fKcLJKK57kDdja7w9YPJQMdsshtcL",
          "from": "B62qnuDyj65AQfcZt3MvcwSX7ohLcP1ayNQwu9Zi6YH7JQddJnc8mkz",
          "fee": "100000000",
          "token": "1",
          "nonce": "61",
          "memo": "hello",
          "amount": "1",
          "valid_until": null
      },
      "stake_delegation": null
  }"#.to_string(),
  // cspell:enable
  };

  let response = mina_mesh.construction_submit(request).await;
  assert!(response.is_err());
  assert_debug_snapshot!(response);
  Ok(())
}

#[tokio::test]
async fn construction_submit_invalid_signature_delegation() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;

  let request = ConstructionSubmitRequest {
    network_identifier: network_id().into(),
    // cspell:disable
    signed_transaction: r#"{
      "signature": "89B921906609910B2EADC6735481B3D3D3E732542E2F1A208505C841C2A6A9036F53E64B2C25017E8E793D8C252FFD374FE442FC887F4D0FB1C6D44506872F12",
      "payment": null,
      "stake_delegation": {
          "delegator": "B62qnuDyj65AQfcZt3MvcwSX7ohLcP1ayNQwu9Zi6YH7JQddJnc8mkz",
          "new_delegate": "B62qj7nR7j5GiQLJEBrMq49nX8fKcLJKK57kDdja7w9YPJQMdsshtcL",
          "fee": "100000000",
          "nonce": "61",
          "memo": "hello",
          "valid_until": null
      }
  }"#.to_string()
  // cspell:enable
  };

  let response = mina_mesh.construction_submit(request).await;
  assert!(response.is_err());
  assert_debug_snapshot!(response);
  Ok(())
}

#[tokio::test]
async fn construction_submit_duplicate() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;

  let request = ConstructionSubmitRequest {
    network_identifier: network_id().into(),
    // cspell:disable
    signed_transaction: r#"{
        "signature": "C4DAE5AA865661D4E60C451FDED55AEAFD2346009A531B8090EA35E0F71B2423F44E2352E0EB341EAFD4855278B549A0DBDEF8BC4F8D3066801CE7CB8EB73222",
        "payment": {
          "to": "B62qnvdfRmG8vFqBwvPs6XhZvvtGi95xW9pcG6tMQqqszEhMfvoCKRn",
          "from": "B62qnuDyj65AQfcZt3MvcwSX7ohLcP1ayNQwu9Zi6YH7JQddJnc8mkz",
          "fee": "100000000",
          "token": "1",
          "nonce": "56",
          "memo": "hello",
          "amount": "7",
          "valid_until": "2000000"
        },
        "stake_delegation": null
    }"#.to_string(),
    // cspell:enable
  };

  let response = mina_mesh.construction_submit(request).await;
  assert!(response.is_err());
  assert_debug_snapshot!(response);
  Ok(())
}

#[tokio::test]
async fn construction_submit_no_sender_exists() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;

  let request = ConstructionSubmitRequest {
    network_identifier: network_id().into(),
    // cspell:disable
    signed_transaction: r#"{
        "signature": "7862D4BF3AF2A8B383230A62DEC8E51C29ABE38177395312FA77CAC68A21D8342F95743E3D9ACAA5E0E1EE54D021112C73E7D28457B7CBC315BB1D74C39C1A09",
        "payment": {
          "to": "B62qj7nR7j5GiQLJEBrMq49nX8fKcLJKK57kDdja7w9YPJQMdsshtcL",
          "from": "B62qjJhLursd9rcpo1vEV2CxRc29fQNP79K3KNHjhmUJCEtBcwvtbpz",
          "fee": "100000000",
          "token": "1",
          "nonce": "61",
          "memo": "hello",
          "amount": "1",
          "valid_until": null
        },
        "stake_delegation": null
    }"#.to_string(),
    // cspell:enable
  };

  let response = mina_mesh.construction_submit(request).await;
  assert!(response.is_err());
  assert_debug_snapshot!(response);
  Ok(())
}

#[tokio::test]
async fn construction_submit_insufficient_balance() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;

  let request = ConstructionSubmitRequest {
    network_identifier: network_id().into(),
    // cspell:disable
    signed_transaction: r#"{
        "signature": "5B045BE461E3B51E6FF08119D3C016BC944392CB5CB93D24AB52B4CCFD7638257DCBDFFAA1135848C459D4FC8A7BF5A9C2110C7DF7155BCC07A110BE695E7904",
        "payment": {
            "to": "B62qj7nR7j5GiQLJEBrMq49nX8fKcLJKK57kDdja7w9YPJQMdsshtcL",
            "from": "B62qnuDyj65AQfcZt3MvcwSX7ohLcP1ayNQwu9Zi6YH7JQddJnc8mkz",
            "fee": "145000000",
            "token": "1",
            "nonce": "61",
            "memo": "hello",
            "amount": "1000000000000000",
            "valid_until": null
        },
        "stake_delegation": null
    }"#.to_string(),
    // cspell:enable
  };

  let response = mina_mesh.construction_submit(request).await;
  assert!(response.is_err());
  assert_debug_snapshot!(response);
  Ok(())
}

#[tokio::test]
async fn construction_submit_insufficient_fee() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;

  let request = ConstructionSubmitRequest {
    network_identifier: network_id().into(),
    // cspell:disable
    signed_transaction: r#"{
        "signature": "0E82F18B53117A9A37587F1C68DCF5B942540FE04863743F3A3DEF34A17B580550F10DCF7601B12D7CE5797DF753066A7A8234BE0E8DE8109352943CBCED7407",
        "payment": {
            "to": "B62qj7nR7j5GiQLJEBrMq49nX8fKcLJKK57kDdja7w9YPJQMdsshtcL",
            "from": "B62qnuDyj65AQfcZt3MvcwSX7ohLcP1ayNQwu9Zi6YH7JQddJnc8mkz",
            "fee": "1",
            "token": "1",
            "nonce": "61",
            "memo": "hello",
            "amount": "1",
            "valid_until": null
        },
        "stake_delegation": null
    }"#.to_string(),
    // cspell:enable
  };

  let response = mina_mesh.construction_submit(request).await;
  assert!(response.is_err());
  assert_debug_snapshot!(response);
  Ok(())
}
