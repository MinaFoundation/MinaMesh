use anyhow::Result;
use coinbase_mesh::models::ConstructionCombineRequest;
use insta::assert_debug_snapshot;
use mina_mesh::{
  models::SignatureType,
  test::{network_id, signature, unsigned_transaction_delegation, unsigned_transaction_payment},
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
  // cspell:disable-next-line
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
  // cspell:disable-next-line
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
  // cspell:disable-next-line
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
  // cspell:disable-next-line
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
