use erased_serde::Serialize as ErasedSerialize;

mod account_balance;
mod block;

pub type CompareGroup<'a> = (&'a str, Vec<(String, Box<dyn ErasedSerialize>)>);

pub fn groups<'a>() -> Vec<CompareGroup<'a>> {
  vec![account_balance::account_balance(), block::block()]
}
