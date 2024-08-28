use crate::common::MinaMeshContext;
use anyhow::Result;
use mesh::models::{ConstructionPreprocessRequest, ConstructionPreprocessResponse};

/// https://github.com/MinaProtocol/mina/blob/985eda49bdfabc046ef9001d3c406e688bc7ec45/src/app/rosetta/lib/construction.ml#L392
pub fn construction_preprocess(
  context: &MinaMeshContext,
  request: ConstructionPreprocessRequest,
) -> Result<ConstructionPreprocessResponse> {
  Ok(ConstructionPreprocessResponse::new())
}
