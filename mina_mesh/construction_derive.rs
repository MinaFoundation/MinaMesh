use crate::common::MinaMeshContext;
use anyhow::Result;
pub use mesh::models::{ConstructionDeriveRequest, ConstructionDeriveResponse};

/// https://github.com/MinaProtocol/mina/blob/985eda49bdfabc046ef9001d3c406e688bc7ec45/src/app/rosetta/lib/construction.ml#L162
pub fn construction_derive(
  context: &MinaMeshContext,
  request: ConstructionDeriveRequest,
) -> Result<ConstructionDeriveResponse> {
  Ok(ConstructionDeriveResponse::new())
}
