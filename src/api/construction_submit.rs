use anyhow::Result;
use coinbase_mesh::models::{ConstructionSubmitRequest, TransactionIdentifier};
use cynic::MutationBuilder;

use crate::{
  graphql::{SendDelegation, SendDelegationVariables, SendPayment, SendPaymentVariables},
  MinaMesh, MinaMeshError, Payment, StakeDelegation, TransactionSigned,
};

/// https://github.com/MinaProtocol/mina/blob/985eda49bdfabc046ef9001d3c406e688bc7ec45/src/app/rosetta/lib/construction.ml#L849
impl MinaMesh {
  pub async fn construction_submit(
    &self,
    request: ConstructionSubmitRequest,
  ) -> Result<TransactionIdentifier, MinaMeshError> {
    self.validate_network(&request.network_identifier).await?;

    let signed_transaction = TransactionSigned::from_json_string(&request.signed_transaction)?;
    if signed_transaction.payment.is_some() && signed_transaction.stake_delegation.is_some() {
      return Err(MinaMeshError::JsonParse(Some(
        "Signed transaction must have one of: payment, stake_delegation".to_string(),
      )));
    }

    // tracing::debug!("CACHE: {:?}", self.cache);
    if signed_transaction.payment.is_some() {
      tracing::info!("Payment transaction");
      let payment = signed_transaction.payment.unwrap();
      let hash = self.send_payment(payment, &signed_transaction.signature).await?;
      self.cache_transaction(&signed_transaction.signature);
      tracing::info!("Success! Transaction hash: {}", hash);
      Ok(TransactionIdentifier::new(hash))
    } else if signed_transaction.stake_delegation.is_some() {
      tracing::info!("Stake delegation transaction");
      let delegation = signed_transaction.stake_delegation.unwrap();
      let hash = self.send_delegation(delegation, &signed_transaction.signature).await?;
      self.cache_transaction(&signed_transaction.signature);
      tracing::info!("Success! Transaction hash: {}", hash);

      Ok(TransactionIdentifier::new(hash.to_string()))
    } else {
      tracing::debug!("Signed transaction missing payment or stake delegation");
      return Err(MinaMeshError::JsonParse(Some("Signed transaction missing payment or stake delegation".to_string())));
    }
  }

  async fn send_payment(&self, payment: Payment, signature: &str) -> Result<String, MinaMeshError> {
    let payment_clone = payment.clone();
    let variables = SendPaymentVariables {
      amount: payment.amount.into(),
      fee: payment.fee.into(),
      from: payment.from.into(),
      to: payment.to.into(),
      nonce: payment.nonce.into(),
      valid_until: payment.valid_until.map(|v| v.into()),
      memo: payment.memo.as_deref(),
      signature,
    };

    let response = self.graphql_client.send(SendPayment::build(variables)).await;

    match response {
      Ok(response) => Ok(response.send_payment.payment.hash.0),
      Err(err) => Err(self.map_error(err, signature, Some(payment_clone)).await),
    }
  }

  async fn send_delegation(&self, delegation: StakeDelegation, signature: &str) -> Result<String, MinaMeshError> {
    let variables = SendDelegationVariables {
      fee: delegation.fee.into(),
      from: delegation.delegator.into(),
      to: delegation.new_delegate.into(),
      nonce: delegation.nonce.into(),
      valid_until: delegation.valid_until.map(|v| v.into()),
      memo: delegation.memo.as_deref(),
      signature,
    };

    let response = self.graphql_client.send(SendDelegation::build(variables)).await;

    match response {
      Ok(response) => Ok(response.send_delegation.delegation.hash.0),
      Err(err) => Err(self.map_error(err, signature, None).await),
    }
  }

  async fn map_error(&self, err: MinaMeshError, signed_tx_str: &str, payment: Option<Payment>) -> MinaMeshError {
    match err {
      MinaMeshError::GraphqlMinaQuery(err) => {
        if err.contains("Couldn't infer nonce") {
          MinaMeshError::TransactionSubmitNoSender(err)
        } else if err.contains("less than the minimum fee") {
          MinaMeshError::TransactionSubmitFeeSmall(err)
        } else if err.contains("Invalid_signature") {
          MinaMeshError::TransactionSubmitInvalidSignature(err)
        } else if err.contains("below minimum_nonce") {
          if self.is_transaction_cached(signed_tx_str) {
            return MinaMeshError::TransactionSubmitDuplicate(err);
          }

          if let Some(payment) = payment {
            if self.is_transaction_in_db(payment).await.unwrap_or(false) {
              return MinaMeshError::TransactionSubmitDuplicate("Transaction already in database".to_string());
            }
          }

          MinaMeshError::TransactionSubmitBadNonce(err)
        } else if err.contains("Insufficient_funds") {
          MinaMeshError::TransactionSubmitInsufficientBalance(err)
        } else {
          MinaMeshError::GraphqlMinaQuery(err)
        }
      }
      _ => err,
    }
  }

  async fn is_transaction_in_db(&self, payment: Payment) -> Result<bool, MinaMeshError> {
    let sender = &payment.from;
    let receiver = &payment.to;
    let nonce = payment.nonce as i64;
    let amount = &payment.amount.to_string();
    let fee = &payment.fee.to_string();
    let row = sqlx::query_file!("sql/queries/query_payment.sql", nonce, sender, receiver, amount, fee)
      .fetch_optional(&self.pg_pool)
      .await?;

    Ok(row.is_some())
  }
}
