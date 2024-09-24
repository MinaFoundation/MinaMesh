use cynic::QueryBuilder;
pub use mesh::models::{MempoolTransactionRequest, MempoolTransactionResponse, Transaction, TransactionIdentifier};

use crate::{
  MinaMesh, MinaMeshError,
  graphql::{QueryMempoolTransactions, QueryMempoolTransactionsVariables},
};

/// https://github.com/MinaProtocol/mina/blob/985eda49bdfabc046ef9001d3c406e688bc7ec45/src/app/rosetta/lib/mempool.ml#L137
impl MinaMesh {
  pub async fn mempool_transaction(
    &self,
    request: MempoolTransactionRequest,
  ) -> Result<MempoolTransactionResponse, MinaMeshError> {
    let QueryMempoolTransactions { daemon_status: _daemon_status, initial_peers: _initial_peers, pooled_user_commands } =
      self
        .graphql_client
        .send(QueryMempoolTransactions::build(QueryMempoolTransactionsVariables {
          hashes: Some(vec![request.transaction_identifier.hash.as_str()]),
        }))
        .await?;
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
