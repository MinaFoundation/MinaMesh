use crate::MinaMesh;
use anyhow::Result;
pub use mesh::models::ConstructionSubmitRequest;
pub use mesh::models::TransactionIdentifier;

/// https://github.com/MinaProtocol/mina/blob/985eda49bdfabc046ef9001d3c406e688bc7ec45/src/app/rosetta/lib/construction.ml#L849
impl MinaMesh {
  pub async fn construction_submit(&self, _request: ConstructionSubmitRequest) -> Result<TransactionIdentifier> {
    Ok(TransactionIdentifier::new("".to_string()))
  }
}