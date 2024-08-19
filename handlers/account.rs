use super::Context;
use crate::graphql_generated::mina::{
  Account, AnnotatedBalance, Balance, BalanceQuery, BalanceQueryVariables, Length, PublicKey, StateHash,
};
use anyhow::{Context as AnyhowContext, Result};
use cynic::{http::ReqwestExt, QueryBuilder};
use mesh::models::{
  AccountBalanceRequest, AccountBalanceResponse, AccountIdentifier, Amount, BlockIdentifier, Currency,
  PartialBlockIdentifier,
};

/// https://github.com/MinaProtocol/mina/blob/985eda49bdfabc046ef9001d3c406e688bc7ec45/src/app/rosetta/lib/account.ml#L11
pub async fn balance(request: AccountBalanceRequest) -> Result<AccountBalanceResponse> {
  let context = Context::from_env().await?;
  match request.block_identifier {
    Some(block_identifier) => block_balance(&context, &*request.account_identifier, *block_identifier).await,
    None => frontier_balance(&context, &*request.account_identifier).await,
  }
}

async fn block_balance(
  context: &Context,
  account_identifier: &AccountIdentifier,
  block_identifier: PartialBlockIdentifier,
) -> Result<AccountBalanceResponse> {
  let rec = sqlx::query!(
    r#"
      SELECT COUNT(*) FROM blocks
      WHERE height = $1
      AND chain_status = 'canonical'
    "#,
    block_identifier.index,
  )
  .fetch_one(&context.pool)
  .await?;
  if let Some(count) = rec.count {
    if count > 0 {
      // has canonical height
      // do we really need to do a different query?
      if let Some(serde_json::Value::Object(map)) = &account_identifier.metadata {
        let token_id = map.get("token_id").context("token not in map")?.as_str().context("token value not string")?;
        let rec = sqlx::query!(
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
          account_identifier.address,
          block_identifier.index,
          token_id
        );
        let result = rec.fetch_one(&context.pool).await?;
        let _height = result.height;
        let _state_hash = block_identifier.hash;
        // height,
        // block_global_slot_since_genesis,
        // balance,
        // nonce,
        // timing_id,
      } else {
        unimplemented!();
      };
    } else {
      // query pending chain as well
    }
  }
  unimplemented!()
}

async fn frontier_balance(context: &Context, address: &AccountIdentifier) -> Result<AccountBalanceResponse> {
  let operation = BalanceQuery::build(BalanceQueryVariables { public_key: PublicKey(address.address.clone()) });
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

#[tokio::test]
async fn first() {
  use mesh::models::NetworkIdentifier;
  let x = balance(AccountBalanceRequest {
    account_identifier: Box::new(AccountIdentifier {
      // address: "B62qrQKS9ghd91shs73TCmBJRW9GzvTJK443DPx2YbqcyoLc56g1ny9".into(),
      address: "B62qooos8xGyqtJGpT7eaoyGrABCf4vcAnzCtxPLNrf26M7FwAxHg1i".into(),
      sub_account: None,
      metadata: None,
    }),
    block_identifier: None,
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
