use erased_serde::Serialize as ErasedSerialize;

mod account_balance;
mod block;
mod construction_combine;
mod construction_derive;
mod construction_hash;
mod construction_metadata;
mod construction_parse;
mod construction_payloads;
mod construction_preprocess;
mod mempool;
mod network;
mod search_transactions;

pub use account_balance::*;
pub use block::*;
pub use construction_combine::*;
pub use construction_derive::*;
pub use construction_hash::*;
pub use construction_metadata::*;
pub use construction_parse::*;
pub use construction_payloads::*;
pub use construction_preprocess::*;
pub use mempool::*;
pub use network::*;
pub use search_transactions::*;

pub type CompareGroup<'a> = (&'a str, Vec<Box<dyn ErasedSerialize>>);
