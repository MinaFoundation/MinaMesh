use mina_mesh::{
  models::{MempoolTransactionRequest, NetworkRequest, TransactionIdentifier},
  test::network_id,
};

use super::CompareGroup;

#[allow(dead_code)]
pub fn mempool<'a>() -> CompareGroup<'a> {
  ("/mempool", vec![Box::new(NetworkRequest::new(network_id()))])
}

pub fn mempool_transaction<'a>() -> CompareGroup<'a> {
  (
    "/mempool/transaction",
    vec![Box::new(MempoolTransactionRequest::new(
      network_id(),
      TransactionIdentifier::new("hash_not_exists".to_string()),
    ))],
  )
}
