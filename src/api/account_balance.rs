use coinbase_mesh::models::{
  AccountBalanceRequest, AccountBalanceResponse, AccountIdentifier, Amount, BlockIdentifier, PartialBlockIdentifier,
};
use cynic::QueryBuilder;

use crate::{
  create_currency,
  graphql::{
    Account3, AccountNonce, AnnotatedBalance, Balance, Length, QueryBalance, QueryBalanceVariables, StateHash, TokenId,
  },
  util::Wrapper,
  MinaMesh, MinaMeshError,
};

/// https://github.com/MinaProtocol/mina/blob/985eda49bdfabc046ef9001d3c406e688bc7ec45/src/app/rosetta/lib/account.ml#L11
impl MinaMesh {
  pub async fn account_balance(&self, req: AccountBalanceRequest) -> Result<AccountBalanceResponse, MinaMeshError> {
    self.validate_network(&req.network_identifier).await?;
    let AccountIdentifier { address, metadata, .. } = *req.account_identifier;
    match req.block_identifier {
      Some(block_identifier) => self.block_balance(address, metadata, *block_identifier).await,
      None => self.frontier_balance(address).await,
    }
  }

  async fn block_balance(
    &self,
    public_key: String,
    metadata: Option<serde_json::Value>,
    partial_block_id: PartialBlockIdentifier,
  ) -> Result<AccountBalanceResponse, MinaMeshError> {
    let index = partial_block_id.index;
    let hash = partial_block_id.hash;
    let block = sqlx::query_file!("sql/queries/maybe_block.sql", index, hash)
      .fetch_optional(&self.pg_pool)
      .await?
      .ok_or(MinaMeshError::BlockMissing(index, hash.clone()))?;
    let maybe_account_balance_info = sqlx::query_file!(
      "sql/queries/maybe_account_balance_info.sql",
      public_key,
      block.height.ok_or(MinaMeshError::ChainInfoMissing)?,
      Wrapper(metadata).token_id_or_default()?
    )
    .fetch_optional(&self.pg_pool)
    .await?;
    match maybe_account_balance_info {
      None => Ok(AccountBalanceResponse {
        block_identifier: Box::new(BlockIdentifier {
          hash: match block.state_hash {
            Some(state_hash) => state_hash,
            None => return Err(MinaMeshError::BlockMissing(index, hash.clone())),
          },
          index: match block.height {
            Some(height) => height,
            None => return Err(MinaMeshError::BlockMissing(index, hash)),
          },
        }),
        balances: vec![Amount {
          currency: Box::new(create_currency(None)),
          value: "0".to_string(),
          metadata: Some(serde_json::json!({
            "locked_balance": 0,
            "liquid_balance": 0,
            "total_balance": 0
          })),
        }],
        metadata: Some(serde_json::json!({
          "created_via_historical_lookup": true,
          "nonce": "0"
        })),
      }),
      Some(account_balance_info) => {
        let token_id = account_balance_info.token_id;
        let nonce = account_balance_info.nonce;
        let last_relevant_command_balance = account_balance_info.balance.parse::<u64>()?;
        let timing_info = sqlx::query_file!("sql/queries/timing_info.sql", account_balance_info.timing_id)
          .fetch_optional(&self.pg_pool)
          .await?;
        let liquid_balance = match timing_info {
          Some(timing_info) => {
            let incremental_balance = incremental_balance_between_slots(
              account_balance_info.block_global_slot_since_genesis.ok_or(MinaMeshError::ChainInfoMissing)? as u32,
              block.global_slot_since_genesis.ok_or(MinaMeshError::ChainInfoMissing)? as u32,
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
        Ok(AccountBalanceResponse {
          block_identifier: Box::new(BlockIdentifier {
            hash: match block.state_hash {
              Some(state_hash) => state_hash,
              None => return Err(MinaMeshError::BlockMissing(index, hash.clone())),
            },
            index: match block.height {
              Some(height) => height,
              None => return Err(MinaMeshError::BlockMissing(index, hash)),
            },
          }),
          balances: vec![Amount {
            currency: Box::new(create_currency(Some(&token_id))),
            value: liquid_balance.to_string(),
            metadata: Some(serde_json::json!({
              "locked_balance": locked_balance,
              "liquid_balance": liquid_balance,
              "total_balance": total_balance
            })),
          }],
          metadata: Some(serde_json::json!({
            "created_via_historical_lookup": true,
            "nonce": format!("{}", nonce)
          })),
        })
      }
    }
  }

  async fn frontier_balance(&self, public_key: String) -> Result<AccountBalanceResponse, MinaMeshError> {
    let result = self
      .graphql_client
      .send(QueryBalance::build(QueryBalanceVariables { public_key: public_key.clone().into() }))
      .await?;
    if let QueryBalance {
      account:
        Some(Account3 {
          balance:
            AnnotatedBalance {
              block_height: Length(index_raw),
              state_hash: Some(StateHash(hash)),
              liquid: Some(Balance(liquid_raw)),
              total: Balance(total_raw),
            },
          nonce: Some(AccountNonce(nonce)),
          token_id: TokenId(token_id),
        }),
    } = result
    {
      let total = total_raw.parse::<u64>()?;
      let liquid = liquid_raw.parse::<u64>()?;
      let index = index_raw.parse::<i64>()?;
      Ok(AccountBalanceResponse {
        block_identifier: Box::new(BlockIdentifier { hash, index }),
        balances: vec![Amount {
          currency: Box::new(create_currency(Some(&token_id))),
          value: total_raw,
          metadata: Some(serde_json::json!({
            "locked_balance": (total - liquid),
            "liquid_balance": liquid,
            "total_balance": total
          })),
        }],
        metadata: Some(serde_json::json!({
          "created_via_historical_lookup": false,
          "nonce": format!("{}", nonce)
        })),
      })
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
