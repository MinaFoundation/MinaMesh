use anyhow::Result;
use coinbase_mesh::models::{ConstructionCombineRequest, ConstructionCombineResponse, SignatureType};

use crate::{signer_utils::decode_signature, MinaMesh, MinaMeshError, TransactionSigned, TransactionUnsigned};

/// https://github.com/MinaProtocol/mina/blob/985eda49bdfabc046ef9001d3c406e688bc7ec45/src/app/rosetta/lib/construction.ml#L561
impl MinaMesh {
  pub async fn construction_combine(
    &self,
    request: ConstructionCombineRequest,
  ) -> Result<ConstructionCombineResponse, MinaMeshError> {
    self.validate_network(&request.network_identifier).await?;

    let unsigned_transaction = TransactionUnsigned::from_json_string(&request.unsigned_transaction)?;
    let signatures = request.signatures;
    if signatures.len() != 1 {
      return Err(MinaMeshError::SignatureInvalid(format!("Expected 1 signature, found {}", signatures.len())));
    }
    let signature = signatures.first().ok_or(MinaMeshError::SignatureMissing)?;
    if signature.signature_type != SignatureType::SchnorrPoseidon {
      return Err(MinaMeshError::SignatureInvalid(format!(
        "Expected SchnorrPoseidon, found {:?}",
        signature.signature_type
      )));
    }

    hex::decode(&unsigned_transaction.random_oracle_input)
      .map_err(|e| MinaMeshError::JsonParse(Some(format!("Decoding of randomOracleInput failed: {}", e))))?;
    // TODO: Verify the random oracle input

    decode_signature(signature.hex_bytes.as_str())?;

    let payment = unsigned_transaction.payment;
    let stake_delegation = unsigned_transaction.stake_delegation;

    let signed_transaction = TransactionSigned { signature: signature.hex_bytes.clone(), payment, stake_delegation };
    let signed_transaction_json = signed_transaction.as_json_string()?;

    Ok(ConstructionCombineResponse::new(signed_transaction_json))
  }
}
