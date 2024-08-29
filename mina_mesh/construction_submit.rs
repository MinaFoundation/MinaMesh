use crate::common::MinaMesh;
use anyhow::Result;
pub use mesh::models::{ConstructionSubmitRequest, TransactionIdentifier};

/// https://github.com/MinaProtocol/mina/blob/985eda49bdfabc046ef9001d3c406e688bc7ec45/src/app/rosetta/lib/construction.ml#L849
impl MinaMesh {
  pub fn construction_submit(&self, _request: ConstructionSubmitRequest) -> Result<TransactionIdentifier> {
    Ok(TransactionIdentifier::new("".to_string()))
  }
}
