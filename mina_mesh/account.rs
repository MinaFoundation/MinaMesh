use crate::graphql_generated::mina::{
  Account, AnnotatedBalance, Balance, Length, PublicKey, QueryBalance, QueryBalanceVariables, StateHash,
};
use crate::MinaMeshContext;
use anyhow::Result;
use cynic::{http::ReqwestExt, QueryBuilder};
use mesh::models::{
  AccountBalanceRequest, AccountBalanceResponse, AccountIdentifier, Amount, BlockIdentifier, Currency,
  PartialBlockIdentifier,
};

/// https://github.com/MinaProtocol/mina/blob/985eda49bdfabc046ef9001d3c406e688bc7ec45/src/app/rosetta/lib/account.ml#L11
pub async fn balance(request: AccountBalanceRequest) -> Result<AccountBalanceResponse> {
  let context = MinaMeshContext::from_env().await?;
  context.network_health_check(*request.network_identifier).await?;
  let account: MinaAccountIdentifier = (*request.account_identifier).into();
  match request.block_identifier {
    Some(block_identifier) => block_balance(&context, &account, *block_identifier).await,
    None => frontier_balance(&context, &account).await,
  }
}

const DEFAULT_TOKEN_ID: &str = "wSHV2S4qX9jFsLjQo8r1BsMLH2ZRKsZx6EJd1sbozGPieEC4Jf";

async fn block_balance(
  context: &MinaMeshContext,
  account_identifier: &MinaAccountIdentifier,
  block_identifier: PartialBlockIdentifier,
) -> Result<AccountBalanceResponse> {
  // Get block data from the database
  let maybe_block = sqlx::query_file!("sql/maybe_block.sql", block_identifier.index)
    .fetch_optional(&context.pool)
    .await?;
  match maybe_block {
    Some(block) => {
      // has canonical height
      // do we really need to do a different query?
      let maybe_account_balance_info = sqlx::query_file!(
        "sql/maybe_account_balance_info.sql",
        account_identifier.public_key,
        block_identifier.index.unwrap(),
        account_identifier.token_id,
      )
      .fetch_optional(&context.pool)
      .await?;
      match maybe_account_balance_info {
        None => {
          return Ok(AccountBalanceResponse::new(
            BlockIdentifier {
              hash: block.state_hash,
              index: block.height,
            },
            vec![Amount {
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
            }],
          ));
        }
        Some(account_balance_info) => {
          println!("B");
          let last_relevant_command_balance = account_balance_info.balance.parse::<u64>()?;
          let timing_info = sqlx::query_file!("sql/timing_info.sql", account_balance_info.timing_id)
            .fetch_optional(&context.pool)
            .await?;
          let liquid_balance = match timing_info {
            Some(timing_info) => {
              let incremental_balance = incremental_balance_between_slots(
                account_balance_info.block_global_slot_since_genesis as u32,
                block.global_slot_since_genesis as u32,
                timing_info.cliff_time as u32,
                timing_info.cliff_amount.parse::<u64>().unwrap(),
                timing_info.vesting_period as u32,
                timing_info.vesting_increment.parse::<u64>().unwrap(),
                timing_info.initial_minimum_balance.parse::<u64>().unwrap(),
              );
              last_relevant_command_balance + incremental_balance
            }
            None => last_relevant_command_balance,
          };
          let total_balance = last_relevant_command_balance;
          let locked_balance = total_balance - liquid_balance;
          Ok(AccountBalanceResponse::new(
            BlockIdentifier {
              hash: block.state_hash,
              index: block.height,
            },
            vec![Amount {
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
            }],
          ))
        }
      }
    }
    None => {
      // TODO: return proper error message
      anyhow::bail!("Block not found")
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
  let min_balance_at_end_slot = min_balance_at_slot(
    end_slot,
    cliff_time,
    cliff_amount,
    vesting_period,
    vesting_increment,
    initial_minimum_balance,
  );
  min_balance_at_start_slot.saturating_sub(min_balance_at_end_slot)
}

// Note: The `min_balance_at_slot` function is not provided in the original OCaml code,
// so we'll declare it here as a separate function that needs to be implemented.

async fn frontier_balance(
  context: &MinaMeshContext,
  address: &MinaAccountIdentifier,
) -> Result<AccountBalanceResponse> {
  let operation = QueryBalance::build(QueryBalanceVariables {
    public_key: PublicKey(address.public_key.clone()),
  });
  let result = context
    .client
    .post(&context.config.mina_proxy_url)
    .run_graphql(operation)
    .await?;
  if let Some(QueryBalance {
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
  }) = result.data
  {
    let total = total_raw.parse::<u64>()?;
    let liquid = liquid_raw.parse::<u64>()?;
    let index = index_raw.parse::<i64>()?;
    return Ok(AccountBalanceResponse::new(
      BlockIdentifier { hash, index },
      vec![Amount {
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
      }],
    ));
  } else {
    anyhow::bail!("Failed to get balance: {:?}", result.errors.or(Some(vec![])))
  }
}

#[derive(Debug)]
pub struct MinaAccountIdentifier {
  pub public_key: String,
  pub token_id: String,
}

impl Into<MinaAccountIdentifier> for AccountIdentifier {
  fn into(self) -> MinaAccountIdentifier {
    let token_id = match self.metadata {
      Some(serde_json::Value::Object(map)) => map.get("token_id").map(|v| v.as_str().unwrap().to_string()),
      None => Some(DEFAULT_TOKEN_ID.to_string()),
      _ => unimplemented!(),
    }
    .unwrap(); // TODO: handle unwrap
    MinaAccountIdentifier {
      public_key: self.address,
      token_id,
    }
  }
}

#[tokio::test]
async fn first() {
  use mesh::models::NetworkIdentifier;
  let x = balance(AccountBalanceRequest {
    account_identifier: Box::new(AccountIdentifier {
      // address: "B62qrQKS9ghd91shs73TCmBJRW9GzvTJK443DPx2YbqcyoLc56g1ny9".into(),
      // address: "B62qooos8xGyqtJGpT7eaoyGrABCf4vcAnzCtxPLNrf26M7FwAxHg1i".into(),
      address: "B62qmjJeM4Fd4FVghfhgwoE1fkEexK2Rre8WYKMnbxVwB5vtKUwvgMv".into(),
      sub_account: None,
      metadata: None,
    }),
    block_identifier: Some(Box::new(PartialBlockIdentifier {
      index: Some(371513),
      hash: None,
    })),
    currencies: None,
    network_identifier: Box::new(NetworkIdentifier {
      blockchain: "mina".into(),
      network: "mainnet".into(),
      sub_network_identifier: None,
    }),
  })
  .await;
  println!("{:?}", x);
}

pub fn coins() {}
