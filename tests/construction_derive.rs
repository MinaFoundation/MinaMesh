use anyhow::Result;
use insta::assert_debug_snapshot;
use mina_mesh::{
  models::{ConstructionDeriveRequest, CurveType::Tweedle, PublicKey},
  test::network_id,
  MinaMeshConfig,
};
use serde_json::json;

#[tokio::test]
async fn construction_derive_success() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;
  let request = ConstructionDeriveRequest::new(
    network_id(),
    PublicKey::new(
      // cspell:disable-next-line
      "3C2B5B48C22DC8B8C9D2C9D76A2CEAAF02BEABB364301726C3F8E989653AF513".to_string(),
      Tweedle,
    ),
  );
  let response = mina_mesh.construction_derive(request).await;
  assert!(response.is_ok());
  assert_debug_snapshot!(response);
  Ok(())
}

#[tokio::test]
async fn construction_derive_fail() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;
  let request = ConstructionDeriveRequest::new(
    network_id(),
    PublicKey::new(
      // cspell:disable-next-line
      "12345".to_string(),
      Tweedle,
    ),
  );
  let response = mina_mesh.construction_derive(request).await;
  assert!(response.is_err());
  assert_debug_snapshot!(response);
  Ok(())
}

#[tokio::test]
async fn construction_derive_token_id() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;
  let request = ConstructionDeriveRequest {
    network_identifier: Box::new(network_id()),
    public_key: Box::new(PublicKey::new(
      // cspell:disable-next-line
      "fad1d3e31aede102793fb2cce62b4f1e71a214c94ce18ad5756eba67ef398390".to_string(),
      Tweedle,
    )),
    // cspell:disable-next-line
    metadata: Some(json!({ "token_id": "weihj2SSP7Z96acs56ygP64Te6wauzvWWfAPHKb1gzqem9J4Ne" })),
  };
  let response = mina_mesh.construction_derive(request).await;
  assert!(response.is_ok());
  assert_debug_snapshot!(response);
  Ok(())
}

#[tokio::test]
async fn construction_derive_token_id_fail() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;
  let request = ConstructionDeriveRequest {
    network_identifier: Box::new(network_id()),
    public_key: Box::new(PublicKey::new(
      // cspell:disable-next-line
      "fad1d3e31aede102793fb2cce62b4f1e71a214c94ce18ad5756eba67ef398390".to_string(),
      Tweedle,
    )),
    // cspell:disable-next-line
    metadata: Some(json!({ "token_id": "fake" })),
  };
  let response = mina_mesh.construction_derive(request).await;
  assert!(response.is_err());
  assert_debug_snapshot!(response);
  Ok(())
}
