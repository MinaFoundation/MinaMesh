use anyhow::Result;
use coinbase_mesh::models::{ConstructionSubmitRequest, TransactionIdentifier};

use crate::MinaMesh;

/// https://github.com/MinaProtocol/mina/blob/985eda49bdfabc046ef9001d3c406e688bc7ec45/src/app/rosetta/lib/construction.ml#L849
impl MinaMesh {
  pub async fn construction_submit(&self, request: ConstructionSubmitRequest) -> Result<TransactionIdentifier> {
    self.validate_network(&request.network_identifier).await?;
    Ok(TransactionIdentifier::new("".to_string()))
  }
}
