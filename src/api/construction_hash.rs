use anyhow::Result;
use coinbase_mesh::models::{ConstructionHashRequest, TransactionIdentifier, TransactionIdentifierResponse};

use crate::{MinaMesh, MinaMeshError, TransactionSigned};

/// https://github.com/MinaProtocol/mina/blob/985eda49bdfabc046ef9001d3c406e688bc7ec45/src/app/rosetta/lib/construction.ml#L786
impl MinaMesh {
  pub async fn construction_hash(
    &self,
    request: ConstructionHashRequest,
  ) -> Result<TransactionIdentifierResponse, MinaMeshError> {
    self.validate_network(&request.network_identifier).await?;

    let tx: TransactionSigned = TransactionSigned::from_json_string(&request.signed_transaction)
      .map_err(|_| MinaMeshError::JsonParse(Some("Failed to parse signed transaction".to_string())))?;

    let tx_hash = self.hash_signed_transaction(&tx)?;

    Ok(TransactionIdentifierResponse::new(TransactionIdentifier::new(tx_hash)))
  }

  fn hash_signed_transaction(&self, tx: &TransactionSigned) -> Result<String, MinaMeshError> {
    let serialized_tx = serde_json::to_string(tx)
      .map_err(|_| MinaMeshError::JsonParse(Some("Failed to serialize signed transaction".to_string())))?;

    let hash_b58 = bs58::encode(serialized_tx).into_string();

    Ok(hash_b58)
  }
}
