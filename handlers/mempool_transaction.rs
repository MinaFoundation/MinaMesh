use crate::graphql::QueryMempoolTransactions;
use crate::graphql::QueryMempoolTransactionsVariables;
use crate::MinaMesh;
use anyhow::Result;
use cynic::QueryBuilder;
pub use mesh::models::MempoolTransactionRequest;
pub use mesh::models::MempoolTransactionResponse;
pub use mesh::models::Transaction;
pub use mesh::models::TransactionIdentifier;

/// https://github.com/MinaProtocol/mina/blob/985eda49bdfabc046ef9001d3c406e688bc7ec45/src/app/rosetta/lib/mempool.ml#L137
impl MinaMesh {
  pub async fn mempool_transaction(&self, request: MempoolTransactionRequest) -> Result<MempoolTransactionResponse> {
    let QueryMempoolTransactions {
      daemon_status: _daemon_status,
      initial_peers: _initial_peers,
      pooled_user_commands,
    } = self
      .graphql_client
      .send(QueryMempoolTransactions::build(QueryMempoolTransactionsVariables {
        hashes: Some(vec![request.transaction_identifier.hash.as_str()]),
      }))
      .await?;
    let operations = pooled_user_commands.into_iter().map(|command| command.into()).collect();
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
