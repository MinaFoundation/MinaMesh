use anyhow::Result;
use insta::assert_debug_snapshot;
use mina_mesh::{
  models::ConstructionHashRequest,
  test::{network_id, signed_transaction_delegation, signed_transaction_payment},
  MinaMeshConfig,
};

#[tokio::test]
async fn construction_hash_valid_signed_payment() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;

  let request = ConstructionHashRequest {
    network_identifier: network_id().into(),
    signed_transaction: signed_transaction_payment(),
  };

  let response = mina_mesh.construction_hash(request).await;
  assert!(response.is_ok());
  assert_debug_snapshot!(response);
  Ok(())
}

#[tokio::test]
async fn construction_hash_valid_signed_delegation() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;

  let request = ConstructionHashRequest {
    network_identifier: network_id().into(),
    signed_transaction: signed_transaction_delegation(),
  };

  let response = mina_mesh.construction_hash(request).await;
  assert!(response.is_ok());
  assert_debug_snapshot!(response);
  Ok(())
}

#[tokio::test]
async fn construction_hash_invalid_format_signed() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;

  let request = ConstructionHashRequest {
    network_identifier: network_id().into(),
    signed_transaction: r#"{
            "signature": "EE1D10B5EF283026177B8C61F75C84F09B35C94C6D1417C2C88707E2D26CBB21D1371F16F3AEC696E055C235D9EA2F707630EB395813AEBFE120BBDD5B5E8908",
            "payment": null,
            "stake_delegation": null  
        }"#
      .to_string(),
  };

  let response = mina_mesh.construction_hash(request).await;
  assert!(response.is_err());
  assert_debug_snapshot!(response);
  Ok(())
}
