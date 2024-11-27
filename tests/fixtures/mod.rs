use erased_serde::Serialize as ErasedSerialize;

mod account_balance;
mod block;

pub use account_balance::*;
pub use block::*;

pub type CompareGroup<'a> = (&'a str, Vec<Box<dyn ErasedSerialize>>);
