use erased_serde::Serialize as ErasedSerialize;

mod account_balance;
mod block;
mod mempool;
mod network;
mod search_transactions;

pub use account_balance::*;
pub use block::*;
pub use mempool::*;
pub use network::*;
pub use search_transactions::*;

pub type CompareGroup<'a> = (&'a str, Vec<Box<dyn ErasedSerialize>>);
