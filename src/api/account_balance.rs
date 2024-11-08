use coinbase_mesh::models::{
  AccountBalanceRequest, AccountBalanceResponse, AccountIdentifier, Amount, BlockIdentifier, Currency,
  PartialBlockIdentifier,
};
use cynic::QueryBuilder;

use crate::{
  graphql::{Account, AnnotatedBalance, Balance, Length, QueryBalance, QueryBalanceVariables, StateHash},
  util::Wrapper,
  MinaMesh, MinaMeshError, MinaNetwork,
};

/// https://github.com/MinaProtocol/mina/blob/985eda49bdfabc046ef9001d3c406e688bc7ec45/src/app/rosetta/lib/account.ml#L11
impl MinaMesh {
  pub async fn account_balance(
    &self,
    AccountBalanceRequest { account_identifier, block_identifier: maybe_block_identifier, network_identifier, .. }: AccountBalanceRequest,
  ) -> Result<AccountBalanceResponse, MinaMeshError> {
    let network: MinaNetwork = network_identifier.try_into()?;
    let AccountIdentifier { address, metadata, .. } = *account_identifier;
    match maybe_block_identifier {
      Some(block_identifier) => self.block_balance(&network, address, metadata, *block_identifier).await,
      None => self.frontier_balance(&network, address).await,
    }
  }

  // TODO: can we get the block via the hash and not the index?
  async fn block_balance(
    &self,
    network: &MinaNetwork,
    public_key: String,
    metadata: Option<serde_json::Value>,
    PartialBlockIdentifier { index, .. }: PartialBlockIdentifier,
  ) -> Result<AccountBalanceResponse, MinaMeshError> {
    let pool = self.pool(network).await?;
    let block = sqlx::query_file!("sql/queries/maybe_block.sql", index)
      .fetch_optional(&pool)
      .await?
      .ok_or(MinaMeshError::BlockMissing(index.unwrap().to_string()))?;
    // has canonical height / do we really need to do a different query?
    let maybe_account_balance_info = sqlx::query_file!(
      "sql/queries/maybe_account_balance_info.sql",
      public_key,
      index,
      Wrapper(metadata).to_token_id()?
    )
    .fetch_optional(&pool)
    .await?;
    match maybe_account_balance_info {
      None => {
        Ok(AccountBalanceResponse::new(BlockIdentifier { hash: block.state_hash, index: block.height }, vec![Amount {
          currency: Box::new(Currency {
            symbol: "MINA".into(), // TODO: Use actual currency symbol / custom tokens
            decimals: 9,
            metadata: None,
          }),
          value: "0".to_string(),
          metadata: Some(serde_json::json!({
            "locked_balance": "0".to_string(),
            "liquid_balance": "0".to_string(),
            "total_balance": "0".to_string()
          })),
        }]))
      }
      Some(account_balance_info) => {
        println!("B");
        let last_relevant_command_balance = account_balance_info.balance.parse::<u64>()?;
        let timing_info = sqlx::query_file!("sql/queries/timing_info.sql", account_balance_info.timing_id)
          .fetch_optional(&pool)
          .await?;
        let liquid_balance = match timing_info {
          Some(timing_info) => {
            let incremental_balance = incremental_balance_between_slots(
              account_balance_info.block_global_slot_since_genesis as u32,
              block.global_slot_since_genesis as u32,
              timing_info.cliff_time as u32,
              timing_info.cliff_amount.parse::<u64>()?,
              timing_info.vesting_period as u32,
              timing_info.vesting_increment.parse::<u64>()?,
              timing_info.initial_minimum_balance.parse::<u64>()?,
            );
            last_relevant_command_balance + incremental_balance
          }
          None => last_relevant_command_balance,
        };
        let total_balance = last_relevant_command_balance;
        let locked_balance = total_balance - liquid_balance;
        Ok(AccountBalanceResponse::new(BlockIdentifier { hash: block.state_hash, index: block.height }, vec![Amount {
          currency: Box::new(Currency {
            symbol: "MINA".into(), // TODO: Use actual currency symbol / custom tokens
            decimals: 9,
            metadata: None,
          }),
          value: liquid_balance.to_string(),
          metadata: Some(serde_json::json!({
            "locked_balance": locked_balance.to_string(),
            "liquid_balance": liquid_balance.to_string(),
            "total_balance": total_balance.to_string()
          })),
        }]))
      }
    }
  }

  async fn frontier_balance(
    &self,
    network: &MinaNetwork,
    public_key: String,
  ) -> Result<AccountBalanceResponse, MinaMeshError> {
    let result = self
      .graphql_client
      .send(network, QueryBalance::build(QueryBalanceVariables { public_key: public_key.clone().into() }))
      .await?;
    if let QueryBalance {
      account:
        Some(Account {
          balance:
            AnnotatedBalance {
              block_height: Length(index_raw),
              state_hash: Some(StateHash(hash)),
              liquid: Some(Balance(liquid_raw)),
              total: Balance(total_raw),
            },
          ..
        }),
    } = result
    {
      let total = total_raw.parse::<u64>()?;
      let liquid = liquid_raw.parse::<u64>()?;
      let index = index_raw.parse::<i64>()?;
      Ok(AccountBalanceResponse::new(BlockIdentifier { hash, index }, vec![Amount {
        currency: Box::new(Currency {
          symbol: "MINA".into(), // TODO: Use actual currency symbol / custom tokens
          decimals: 9,
          metadata: None,
        }),
        value: total_raw,
        metadata: Some(serde_json::json!({
          "locked_balance": (total - liquid).to_string(),
          "liquid_balance": liquid.to_string(),
          "total_balance": total.to_string()
        })),
      }]))
    } else {
      Err(MinaMeshError::AccountNotFound(public_key))
    }
  }
}

fn min_balance_at_slot(
  global_slot: u32,
  cliff_time: u32,
  cliff_amount: u64,
  vesting_period: u32,
  vesting_increment: u64,
  initial_minimum_balance: u64,
) -> u64 {
  if global_slot < cliff_time {
    initial_minimum_balance
  } else if vesting_period == 0 {
    0
  } else {
    let min_balance_past_cliff = initial_minimum_balance.saturating_sub(cliff_amount);
    if min_balance_past_cliff == 0 {
      0
    } else {
      let num_periods = (global_slot - cliff_time) / vesting_period;
      let vesting_decrement = if (u64::MAX / num_periods as u64) < vesting_increment {
        u64::MAX
      } else {
        num_periods as u64 * vesting_increment
      };
      min_balance_past_cliff.saturating_sub(vesting_decrement)
    }
  }
}

fn incremental_balance_between_slots(
  start_slot: u32,
  end_slot: u32,
  cliff_time: u32,
  cliff_amount: u64,
  vesting_period: u32,
  vesting_increment: u64,
  initial_minimum_balance: u64,
) -> u64 {
  if end_slot <= start_slot {
    return 0;
  }
  let min_balance_at_start_slot = min_balance_at_slot(
    start_slot,
    cliff_time,
    cliff_amount,
    vesting_period,
    vesting_increment,
    initial_minimum_balance,
  );
  let min_balance_at_end_slot =
    min_balance_at_slot(end_slot, cliff_time, cliff_amount, vesting_period, vesting_increment, initial_minimum_balance);
  min_balance_at_start_slot.saturating_sub(min_balance_at_end_slot)
}

// Note: The `min_balance_at_slot` function is not provided in the original
// OCaml code, so we'll declare it here as a separate function that needs to be
// implemented.
