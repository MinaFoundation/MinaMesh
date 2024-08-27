use crate::common::MinaMeshContext;
use anyhow::Result;
use cynic::QueryBuilder;
use mesh::models::{MempoolTransactionRequest, MempoolTransactionResponse, Transaction, TransactionIdentifier};
use mina_mesh_graphql::{QueryMempoolTransactions, QueryMempoolTransactionsVariables};

/// https://github.com/MinaProtocol/mina/blob/985eda49bdfabc046ef9001d3c406e688bc7ec45/src/app/rosetta/lib/mempool.ml#L137
pub async fn mempool_transaction(
  context: &MinaMeshContext,
  request: MempoolTransactionRequest,
) -> Result<MempoolTransactionResponse> {
  let QueryMempoolTransactions {
    daemon_status,
    initial_peers,
    pooled_user_commands,
  } = context
    .graphql(QueryMempoolTransactions::build(QueryMempoolTransactionsVariables {
      hashes: Some(vec![request.transaction_identifier.hash.as_str()]),
    }))
    .await?;
  Ok(MempoolTransactionResponse {
    metadata: None,
    transaction: Box::new(Transaction {
      metadata: None,
      operations: vec![],
      related_transactions: None,
      transaction_identifier: Box::new(TransactionIdentifier::new("".to_string())),
    }),
  })
}
