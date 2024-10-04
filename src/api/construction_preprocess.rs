use anyhow::Result;
use coinbase_mesh::models::{ConstructionPreprocessRequest, ConstructionPreprocessResponse};

use crate::MinaMesh;

/// https://github.com/MinaProtocol/mina/blob/985eda49bdfabc046ef9001d3c406e688bc7ec45/src/app/rosetta/lib/construction.ml#L392
impl MinaMesh {
  pub async fn construction_preprocess(
    &self,
    _request: ConstructionPreprocessRequest,
  ) -> Result<ConstructionPreprocessResponse> {
    Ok(ConstructionPreprocessResponse::new())
  }
}
