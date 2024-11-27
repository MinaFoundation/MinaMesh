use erased_serde::Serialize as ErasedSerialize;

mod account_balance;
mod block;
mod search_transactions;

#[allow(unused_imports)]
pub use account_balance::*;
#[allow(unused_imports)]
pub use block::*;
pub use search_transactions::*;

pub type CompareGroup<'a> = (&'a str, Vec<Box<dyn ErasedSerialize>>);
