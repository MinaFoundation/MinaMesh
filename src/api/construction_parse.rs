use anyhow::Result;
use coinbase_mesh::models::{ConstructionParseRequest, ConstructionParseResponse};

use crate::MinaMesh;

/// https://github.com/MinaProtocol/mina/blob/985eda49bdfabc046ef9001d3c406e688bc7ec45/src/app/rosetta/lib/construction.ml#L615
impl MinaMesh {
  pub async fn construction_parse(&self, request: ConstructionParseRequest) -> Result<ConstructionParseResponse> {
    self.validate_network(&request.network_identifier).await?;
    Ok(ConstructionParseResponse::new(vec![]))
  }
}
