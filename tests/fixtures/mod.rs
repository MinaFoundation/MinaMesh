use erased_serde::Serialize as ErasedSerialize;

mod account_balance;
mod block;
mod mempool;
mod network;
mod search_transactions;

pub use account_balance::*;
#[allow(unused_imports)]
pub use block::*;
pub use mempool::*;
use mina_mesh::models::{NetworkIdentifier, NetworkRequest};
pub use network::*;
pub use search_transactions::*;

pub type CompareGroup<'a> = (&'a str, Vec<Box<dyn ErasedSerialize>>);

pub fn network_id() -> NetworkIdentifier {
  NetworkIdentifier::new("mina".to_string(), "devnet".to_string())
}

pub fn network_request() -> NetworkRequest {
  NetworkRequest::new(network_id())
}
