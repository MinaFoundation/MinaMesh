use super::Context;
use crate::graphql_generated::mina::{
  Account, AnnotatedBalance, Balance, BalanceQuery, BalanceQueryVariables, Length, PublicKey, StateHash,
};
use anyhow::Result;
use cynic::{http::ReqwestExt, QueryBuilder};
use mesh::models::{
  AccountBalanceRequest, AccountBalanceResponse, AccountIdentifier, Amount, BlockIdentifier, Currency,
  PartialBlockIdentifier,
};

/// https://github.com/MinaProtocol/mina/blob/985eda49bdfabc046ef9001d3c406e688bc7ec45/src/app/rosetta/lib/account.ml#L11
pub async fn balance(request: AccountBalanceRequest) -> Result<AccountBalanceResponse> {
  let context = Context::from_env().await?;
  let account: MinaAccountIdentifier = (*request.account_identifier).into();
  match request.block_identifier {
    Some(block_identifier) => block_balance(&context, &account, *block_identifier).await,
    None => frontier_balance(&context, &account).await,
  }
}

const DEFAULT_TOKEN_ID: &str = "wSHV2S4qX9jFsLjQo8r1BsMLH2ZRKsZx6EJd1sbozGPieEC4Jf";

async fn block_balance(
  context: &Context,
  account_identifier: &MinaAccountIdentifier,
  block_identifier: PartialBlockIdentifier,
) -> Result<AccountBalanceResponse> {
  // Get block data from the database

  let maybe_block = sqlx::query!(
    r#"
      SELECT height, state_hash, global_slot_since_genesis FROM blocks
      WHERE height = $1
      AND chain_status = 'canonical'
    "#,
    block_identifier.index,
  )
  .fetch_optional(&context.pool)
  .await?;
  match maybe_block {
    Some(block) => {
      // has canonical height
      // do we really need to do a different query?
      let account_balance_info = sqlx::query!(
        r#"
          SELECT b.height, b.global_slot_since_genesis AS block_global_slot_since_genesis, balance, nonce, timing_id

          FROM blocks b
          INNER JOIN accounts_accessed ac ON ac.block_id = b.id
          INNER JOIN account_identifiers ai on ai.id = ac.account_identifier_id
          INNER JOIN public_keys pks ON ai.public_key_id = pks.id
          INNER JOIN tokens t ON ai.token_id = t.id

          WHERE pks.value = $1
          AND b.height <= $2
          AND b.chain_status = 'canonical'
          AND t.value = $3

          ORDER BY (b.height) DESC
          LIMIT 1
        "#,
        account_identifier.public_key,
        block_identifier.index,
        account_identifier.token_id,
      );
      let result = account_balance_info.fetch_one(&context.pool).await?;
      println!("HERE");
      // println!("{:?}", result);
      let timing_info =
        sqlx::query!("SELECT * FROM timing_info WHERE id = $1", result.timing_id).fetch_optional(&context.pool).await?;
      println!("REACHES HERE");
      match timing_info {
        Some(timing_info) => {
          let incremental_balance = incremental_balance_between_slots(
            result.block_global_slot_since_genesis as u32,
            block.global_slot_since_genesis as u32,
            timing_info.cliff_time as u32,
            timing_info.cliff_amount.parse::<u64>().unwrap(),
            timing_info.vesting_period as u32,
            timing_info.vesting_increment.parse::<u64>().unwrap(),
            timing_info.initial_minimum_balance.parse::<u64>().unwrap(),
          );
          let balance = result.balance.parse::<u64>()? + incremental_balance;
          let balance_string = result.balance.to_string();
          return Ok(AccountBalanceResponse::new(
            BlockIdentifier { hash: block.state_hash, index: block.height },
            vec![Amount {
              currency: Box::new(Currency {
                symbol: "MINA".into(), // TODO: Use actual currency symbol / custom tokens
                decimals: 9,
                metadata: None,
              }),
              value: balance_string,
              metadata: Some(serde_json::json!({
                "locked_balance": "".to_string(),
                "liquid_balance": balance_string,
                "total_balance": balance.to_string()
              })),
            }],
          ));
        }
        None => {
          let balance_string = result.balance.to_string();
          return Ok(AccountBalanceResponse::new(
            BlockIdentifier { hash: block.state_hash, index: block.height },
            vec![Amount {
              currency: Box::new(Currency {
                symbol: "MINA".into(), // TODO: Use actual currency symbol / custom tokens
                decimals: 9,
                metadata: None,
              }),
              value: result.balance,
              metadata: Some(serde_json::json!({
                "locked_balance": 0.to_string(),
                "liquid_balance": balance_string,
                "total_balance": balance_string
              })),
            }],
          ));
        }
      };
    }
    None => {
      unimplemented!();
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

// Note: The `min_balance_at_slot` function is not provided in the original OCaml code,
// so we'll declare it here as a separate function that needs to be implemented.

async fn frontier_balance(context: &Context, address: &MinaAccountIdentifier) -> Result<AccountBalanceResponse> {
  let operation = BalanceQuery::build(BalanceQueryVariables { public_key: PublicKey(address.public_key.clone()) });
  let result = context.client.post(&context.config.mina_proxy_url).run_graphql(operation).await?;
  if let Some(BalanceQuery {
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
      None => Some(DEFAULT_TOKEN_ID.to_string()), // TODO: return actual default token ID
      _ => unimplemented!(),
    }
    .unwrap(); // TODO: handle unwrap
    MinaAccountIdentifier { public_key: self.address, token_id }
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
    block_identifier: Some(Box::new(PartialBlockIdentifier { index: Some(371513), hash: None })),
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
