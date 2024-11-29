use mina_mesh::models::{MempoolTransactionRequest, NetworkIdentifier, NetworkRequest, TransactionIdentifier};

use super::CompareGroup;

pub fn mempool<'a>() -> CompareGroup<'a> {
  ("/mempool", vec![Box::new(NetworkRequest::new(network_id()))])
}

pub fn mempool_transaction<'a>() -> CompareGroup<'a> {
  ("/mempool/transaction", vec![Box::new(MempoolTransactionRequest::new(
    network_id(),
    TransactionIdentifier::new("hash_not_exists".to_string()),
  ))])
}

fn network_id() -> NetworkIdentifier {
  NetworkIdentifier::new("mina".to_string(), "devnet".to_string())
}
