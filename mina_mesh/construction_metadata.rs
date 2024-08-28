use crate::common::MinaMeshContext;
use anyhow::Result;
pub use mesh::models::{ConstructionMetadataRequest, ConstructionMetadataResponse};

/// https://github.com/MinaProtocol/mina/blob/985eda49bdfabc046ef9001d3c406e688bc7ec45/src/app/rosetta/lib/construction.ml#L133
pub fn construction_metadata(
  context: &MinaMeshContext,
  request: ConstructionMetadataRequest,
) -> Result<ConstructionMetadataResponse> {
  Ok(ConstructionMetadataResponse::new(serde_json::Value::Null))
}
