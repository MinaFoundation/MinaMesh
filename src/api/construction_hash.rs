use anyhow::Result;
use ark_ff::{BigInteger256, PrimeField};
use coinbase_mesh::models::{ConstructionHashRequest, TransactionIdentifier, TransactionIdentifierResponse};
use mina_p2p_messages::{
  bigint::BigInt,
  v2::{
    CurrencyAmountStableV1, CurrencyFeeStableV1, MinaBasePaymentPayloadStableV2, MinaBaseSignatureStableV1,
    MinaBaseSignedCommandMemoStableV1, MinaBaseSignedCommandPayloadBodyStableV2,
    MinaBaseSignedCommandPayloadCommonStableV2, MinaBaseSignedCommandPayloadStableV2, MinaBaseSignedCommandStableV2,
    MinaBaseStakeDelegationStableV2, MinaNumbersGlobalSlotSinceGenesisMStableV1, NonZeroCurvePoint,
    NonZeroCurvePointUncompressedStableV1, UnsignedExtendedUInt32StableV1,
    UnsignedExtendedUInt64Int64ForVersionTagsStableV1,
  },
};
use mina_signer::{pubkey::PubKeyError, CompressedPubKey, PubKey};

use crate::{
  generate_operations_user_command, memo::Memo, MinaMesh, MinaMeshError, TransactionSigned, UserCommandBody,
  UserCommandPayload,
};

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
    let signer = non_zero_curve_point_from_compressed(signer_pk.into_compressed());

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

    let mina_base_signed = MinaBaseSignedCommandStableV2 {
      payload: user_command_payload.into(),
      signer,
      signature: default_signature().into(),
    };

    let hash = mina_base_signed.hash().map_err(|e| MinaMeshError::Exception(e.to_string()))?;

    Ok(TransactionIdentifierResponse::new(TransactionIdentifier::new(hash.to_string())))
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
}

impl From<UserCommandPayload> for MinaBaseSignedCommandPayloadStableV2 {
  fn from(payload: UserCommandPayload) -> Self {
    let common = MinaBaseSignedCommandPayloadCommonStableV2 {
      fee: CurrencyFeeStableV1(UnsignedExtendedUInt64Int64ForVersionTagsStableV1(payload.fee.into())),
      fee_payer_pk: non_zero_curve_point_from_compressed(payload.fee_payer),
      nonce: UnsignedExtendedUInt32StableV1(payload.nonce.into()),
      valid_until: match payload.valid_until {
        Some(valid_until) => {
          MinaNumbersGlobalSlotSinceGenesisMStableV1::SinceGenesis(UnsignedExtendedUInt32StableV1(valid_until.into()))
        }
        None => {
          MinaNumbersGlobalSlotSinceGenesisMStableV1::SinceGenesis(UnsignedExtendedUInt32StableV1(u32::MAX.into()))
        }
      },
      memo: payload.memo.into(),
    };

    let body = match payload.body {
      UserCommandBody::Payment { receiver, amount } => {
        MinaBaseSignedCommandPayloadBodyStableV2::Payment(MinaBasePaymentPayloadStableV2 {
          receiver_pk: non_zero_curve_point_from_compressed(receiver),
          amount: CurrencyAmountStableV1(UnsignedExtendedUInt64Int64ForVersionTagsStableV1(amount.into())),
        })
      }
      UserCommandBody::Delegation { new_delegate } => {
        MinaBaseSignedCommandPayloadBodyStableV2::StakeDelegation(MinaBaseStakeDelegationStableV2::SetDelegate {
          new_delegate: non_zero_curve_point_from_compressed(new_delegate),
        })
      }
    };

    MinaBaseSignedCommandPayloadStableV2 { common, body }
  }
}

/// Conversion `CompressedPubKey` into `NonZeroCurvePoint`
fn non_zero_curve_point_from_compressed(compressed: CompressedPubKey) -> NonZeroCurvePoint {
  let bigint: BigInteger256 = compressed.x.into_repr();
  NonZeroCurvePoint::from(NonZeroCurvePointUncompressedStableV1 { x: BigInt::from(bigint), is_odd: compressed.is_odd })
}

// Conversion `Memo` type to `MinaBaseSignedCommandMemoStableV1`
impl From<Memo> for MinaBaseSignedCommandMemoStableV1 {
  fn from(memo: Memo) -> Self {
    let memo_bytes = &memo.0[..];
    MinaBaseSignedCommandMemoStableV1(mina_p2p_messages::string::CharString::from(memo_bytes))
  }
}

fn default_signature() -> MinaBaseSignatureStableV1 {
  MinaBaseSignatureStableV1(BigInt::one(), BigInt::one())
}
