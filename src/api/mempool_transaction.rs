use coinbase_mesh::models::{
  MempoolTransactionRequest, MempoolTransactionResponse, Transaction, TransactionIdentifier,
};
use cynic::QueryBuilder;

use crate::{
  graphql::{QueryMempoolTransactions, QueryMempoolTransactionsVariables},
  MinaMesh, MinaMeshError,
};

/// https://github.com/MinaProtocol/mina/blob/985eda49bdfabc046ef9001d3c406e688bc7ec45/src/app/rosetta/lib/mempool.ml#L137
impl MinaMesh {
  pub async fn mempool_transaction(
    &self,
    request: MempoolTransactionRequest,
  ) -> Result<MempoolTransactionResponse, MinaMeshError> {
    self.validate_network(&request.network_identifier).await?;
    let QueryMempoolTransactions { daemon_status: _daemon_status, initial_peers: _initial_peers, pooled_user_commands } =
      self
        .graphql_client
        .send(QueryMempoolTransactions::build(QueryMempoolTransactionsVariables {
          hashes: Some(vec![request.transaction_identifier.hash.as_str()]),
        }))
        .await?;

    // Check if the transaction is absent
    if pooled_user_commands.is_empty() {
      return Err(MinaMeshError::TransactionNotFound(request.transaction_identifier.hash));
    }

    let operations = pooled_user_commands.into_iter().map(Into::into).collect();

    Ok(MempoolTransactionResponse {
      metadata: None,
      transaction: Box::new(Transaction {
        operations,
        related_transactions: Some(vec![]),
        transaction_identifier: Box::new(TransactionIdentifier::new(request.transaction_identifier.hash)),
        metadata: None,
      }),
    })
  }
}
