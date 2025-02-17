use erased_serde::Serialize as ErasedSerialize;

mod account_balance;
mod block;
mod construction_derive;
mod construction_metadata;
mod construction_preprocess;
mod mempool;
mod network;
mod search_transactions;

pub use account_balance::*;
pub use block::*;
pub use construction_derive::*;
pub use construction_metadata::*;
pub use construction_preprocess::*;
pub use mempool::*;
pub use network::*;
#[allow(unused_imports)]
pub use search_transactions::*;

pub type CompareGroup<'a> = (&'a str, Vec<Box<dyn ErasedSerialize>>);
