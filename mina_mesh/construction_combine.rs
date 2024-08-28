use crate::common::MinaMeshContext;
use anyhow::Result;
pub use mesh::models::{ConstructionCombineRequest, ConstructionCombineResponse};

/// https://github.com/MinaProtocol/mina/blob/985eda49bdfabc046ef9001d3c406e688bc7ec45/src/app/rosetta/lib/construction.ml#L561
pub fn construction_combine(
  context: &MinaMeshContext,
  request: ConstructionCombineRequest,
) -> Result<ConstructionCombineResponse> {
  Ok(ConstructionCombineResponse::new("".to_string()))
}
