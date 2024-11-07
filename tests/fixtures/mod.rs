use erased_serde::Serialize as ErasedSerialize;

mod account_balance;
mod block;
mod network_status;

pub type CompareGroup<'a> = (&'a str, Vec<(String, Box<dyn ErasedSerialize>)>);

pub fn groups<'a>() -> Vec<CompareGroup<'a>> {
  vec![account_balance::account_balance(), block::block(), network_status::network_status()]
}
