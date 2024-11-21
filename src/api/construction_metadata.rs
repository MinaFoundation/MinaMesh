use anyhow::Result;
use coinbase_mesh::models::{ConstructionMetadataRequest, ConstructionMetadataResponse};

use crate::MinaMesh;

/// https://github.com/MinaProtocol/mina/blob/985eda49bdfabc046ef9001d3c406e688bc7ec45/src/app/rosetta/lib/construction.ml#L133
impl MinaMesh {
  pub async fn construction_metadata(
    &self,
    request: ConstructionMetadataRequest,
  ) -> Result<ConstructionMetadataResponse> {
    self.validate_network(&request.network_identifier).await?;
    Ok(ConstructionMetadataResponse::new(serde_json::Value::Null))
  }
}
