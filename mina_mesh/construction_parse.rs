use crate::common::MinaMeshContext;
use anyhow::Result;
use mesh::models::{ConstructionParseRequest, ConstructionParseResponse};

/// https://github.com/MinaProtocol/mina/blob/985eda49bdfabc046ef9001d3c406e688bc7ec45/src/app/rosetta/lib/construction.ml#L615
pub fn construction_parse(
  context: &MinaMeshContext,
  request: ConstructionParseRequest,
) -> Result<ConstructionParseResponse> {
  Ok(ConstructionParseResponse::new(vec![]))
}
