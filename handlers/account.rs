use super::Context;
use mesh::models::{
  AccountBalanceRequest, AccountCoinsRequest, AccountIdentifier, NetworkIdentifier, PublicKey,
};

/// https://github.com/MinaProtocol/mina/blob/985eda49bdfabc046ef9001d3c406e688bc7ec45/src/app/rosetta/lib/account.ml#L11
fn balance(context: Context, public_key: PublicKey) -> Result<(), envy::Error> {
  let context = Context::from_env()?;
  Ok(())
}

fn coins() {}
