use anyhow::Result;
pub use mesh::models::{ConstructionCombineRequest, ConstructionCombineResponse};

use crate::MinaMesh;

/// https://github.com/MinaProtocol/mina/blob/985eda49bdfabc046ef9001d3c406e688bc7ec45/src/app/rosetta/lib/construction.ml#L561
impl MinaMesh {
  pub async fn construction_combine(
    &self,
    _request: ConstructionCombineRequest,
  ) -> Result<ConstructionCombineResponse> {
    Ok(ConstructionCombineResponse::new("".to_string()))
  }
}
