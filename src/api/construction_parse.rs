use anyhow::Result;
pub use mesh::models::{ConstructionParseRequest, ConstructionParseResponse};

use crate::MinaMesh;

/// https://github.com/MinaProtocol/mina/blob/985eda49bdfabc046ef9001d3c406e688bc7ec45/src/app/rosetta/lib/construction.ml#L615
impl MinaMesh {
  pub async fn construction_parse(&self, _request: ConstructionParseRequest) -> Result<ConstructionParseResponse> {
    Ok(ConstructionParseResponse::new(vec![]))
  }
}
