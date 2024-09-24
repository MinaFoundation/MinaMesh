use anyhow::Result;
pub use mesh::models::{ConstructionPayloadsRequest, ConstructionPayloadsResponse};

use crate::MinaMesh;

/// https://github.com/MinaProtocol/mina/blob/985eda49bdfabc046ef9001d3c406e688bc7ec45/src/app/rosetta/lib/construction.ml#L473
impl MinaMesh {
  pub async fn construction_payloads(
    &self,
    _request: ConstructionPayloadsRequest,
  ) -> Result<ConstructionPayloadsResponse> {
    Ok(ConstructionPayloadsResponse::new("".to_string(), vec![]))
  }
}
