use erased_serde::Serialize as ErasedSerialize;

mod account_balance;
mod block;

type CompareGroup<'a> = (&'a str, Vec<Box<dyn ErasedSerialize>>);

pub fn groups<'a>() -> Vec<CompareGroup<'a>> {
  vec![("/account/balance", account_balance::requests()), ("/block", block::requests())]
}
