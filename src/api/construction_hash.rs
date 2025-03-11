use anyhow::Result;
use coinbase_mesh::models::{ConstructionHashRequest, TransactionIdentifier, TransactionIdentifierResponse};
use mina_signer::{pubkey::PubKeyError, PubKey};

use crate::{generate_operations_user_command, MinaMesh, MinaMeshError, TransactionSigned};

/// https://github.com/MinaProtocol/mina/blob/985eda49bdfabc046ef9001d3c406e688bc7ec45/src/app/rosetta/lib/construction.ml#L786
impl MinaMesh {
  pub async fn construction_hash(
    &self,
    request: ConstructionHashRequest,
  ) -> Result<TransactionIdentifierResponse, MinaMeshError> {
    self.validate_network(&request.network_identifier).await?;

    let tx: TransactionSigned = TransactionSigned::from_json_string(&request.signed_transaction)
      .map_err(|_| MinaMeshError::JsonParse(Some("Failed to parse signed transaction".to_string())))?;

    let signer_pk = self.extract_signer(&tx)?;

    let user_command_payload = if let Some(payment) = &tx.payment {
      let operations = generate_operations_user_command(payment);
      self.validate_operations(&tx, &operations, payment.valid_until, payment.memo.clone())?
    } else if let Some(stake_delegation) = &tx.stake_delegation {
      let operations = generate_operations_user_command(stake_delegation);
      self.validate_operations(&tx, &operations, stake_delegation.valid_until, stake_delegation.memo.clone())?
    } else {
      return Err(MinaMeshError::JsonParse(Some(
        "Signed transaction must have one of: payment, stake_delegation".to_string(),
      )));
    };

    tracing::debug!("signer_pk: {:?}", signer_pk);
    tracing::debug!("User command payload: {:?}", user_command_payload);

    let tx_hash = self.hash_signed_transaction(&tx)?;

    Ok(TransactionIdentifierResponse::new(TransactionIdentifier::new(tx_hash)))
  }

  /// Extract and decompress the signer from the transaction.
  fn extract_signer(&self, tx: &TransactionSigned) -> Result<PubKey, MinaMeshError> {
    let source = &tx.get_source_address()?;
    let pubkey = PubKey::from_address(source).map_err(|e| match e {
      PubKeyError::AddressBase58
      | PubKeyError::AddressLength
      | PubKeyError::AddressRawByteLength
      | PubKeyError::AddressChecksum => {
        MinaMeshError::PublicKeyFormatNotValid(format!("Source address pk compression failed: {}", e))
      }
      PubKeyError::NonCurvePoint | PubKeyError::XCoordinate => {
        MinaMeshError::PublicKeyFormatNotValid(format!("Source address pk decompression failed: {}", e))
      }
      _ => MinaMeshError::PublicKeyFormatNotValid("Source address not valid".into()),
    })?;

    Ok(pubkey)
  }

  fn hash_signed_transaction(&self, tx: &TransactionSigned) -> Result<String, MinaMeshError> {
    let serialized_tx = serde_json::to_string(tx)
      .map_err(|_| MinaMeshError::JsonParse(Some("Failed to serialize signed transaction".to_string())))?;

    let hash_b58 = bs58::encode(serialized_tx).into_string();

    Ok(hash_b58)
  }
}
