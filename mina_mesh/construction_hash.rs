use crate::common::MinaMeshContext;
use anyhow::Result;
use mesh::models::{ConstructionHashRequest, TransactionIdentifier};

/// https://github.com/MinaProtocol/mina/blob/985eda49bdfabc046ef9001d3c406e688bc7ec45/src/app/rosetta/lib/construction.ml#L786
pub fn construction_hash(context: &MinaMeshContext, request: ConstructionHashRequest) -> Result<TransactionIdentifier> {
  Ok(TransactionIdentifier::new("".to_string()))
}
