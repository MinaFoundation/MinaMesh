use crate::common::MinaMesh;
use anyhow::Result;
pub use mesh::models::{Block, BlockIdentifier, BlockRequest, BlockResponse};

/// https://github.com/MinaProtocol/mina/blob/985eda49bdfabc046ef9001d3c406e688bc7ec45/src/app/rosetta/lib/block.ml#L7
impl MinaMesh {
  pub async fn block(&self, _request: BlockRequest) -> Result<BlockResponse> {
    Ok(BlockResponse {
      block: Some(Box::new(Block::new(
        BlockIdentifier::new(0, "".to_string()),
        BlockIdentifier::new(0, "".to_string()),
        0,
        vec![],
      ))),
      other_transactions: Some(vec![]),
    })
  }
}
