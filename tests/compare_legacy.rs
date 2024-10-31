use anyhow::Result;
use mina_mesh::{
  models::{
    AccountBalanceRequest, AccountBalanceResponse, AccountIdentifier, NetworkIdentifier, PartialBlockIdentifier,
  },
  MinaMesh, MinaMeshConfig, MinaMeshError,
};
use pretty_assertions::assert_eq;
use reqwest::Client;

const LEGACY_ENDPOINT: &str = "https://rosetta-online-mainnet.minaprotocol.network";
const SOME_ACCOUNT: &str = "B62qkYHGYmws5CYa3phYEKoZvrENTegEhUJYMhzHUQe5UZwCdWob8zv";

struct CompareContext {
  client: Client,
  mina_mesh: MinaMesh,
}

// macro_rules! create_legacy_cmp {
//   ($name:ident, $url:literal, $request_type:ty) => {
//     async {
//       let (a, b) = tokio::join(
//         mina_mesh.$name(req).await,
//       )
//     }
//     paste! {
//       async fn [<handle _ $name>](mina_mesh: State<Arc<MinaMesh>>, Json(req):
// Json<coinbase_mesh::models::$request_type>) -> impl IntoResponse {
//         Wrapper(mina_mesh.$name(req).await)
//       }
//     }
//   };
//   // ($name:ident) => {
//   //   paste! {
//   //     async fn [<handle _ $name>](mina_mesh: State<Arc<MinaMesh>>) -> impl
// IntoResponse {   //       Wrapper(mina_mesh.$name().await)
//   //     }
//   //   }
//   // };
// }

impl CompareContext {
  fn new(mina_mesh: MinaMesh) -> Self {
    Self { client: Client::new(), mina_mesh }
  }

  async fn assert_bodies_eq(&self, subpath: &str, req: &AccountBalanceRequest) -> Result<()> {
    let (mina_mesh_result, legacy_result) =
      tokio::try_join!(self.mina_mesh.account_balance(req.clone()), self.legacy_req(req, subpath))?;
    assert_eq!(mina_mesh_result, legacy_result);
    Ok(())
  }

  async fn legacy_req(
    &self,
    req: &AccountBalanceRequest,
    subpath: &str,
  ) -> Result<AccountBalanceResponse, MinaMeshError> {
    Ok(
      self
        .client
        .post(format!("{LEGACY_ENDPOINT}{subpath}"))
        .json(req)
        .send()
        .await?
        .json::<AccountBalanceResponse>()
        .await?,
    )
  }
}

#[tokio::test]
async fn main() -> Result<()> {
  tracing_subscriber::fmt::init();
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;
  let compare_legacy = CompareContext::new(mina_mesh);
  compare_legacy
    .assert_bodies_eq("/account/balance", &AccountBalanceRequest {
      account_identifier: Box::new(AccountIdentifier {
        address: SOME_ACCOUNT.into(),
        sub_account: None,
        metadata: None,
      }),
      block_identifier: Some(Box::new(PartialBlockIdentifier { index: Some(6265), hash: None })),
      currencies: None,
      network_identifier: Box::new(NetworkIdentifier {
        blockchain: "mina".into(),
        network: "mainnet".into(),
        sub_network_identifier: None,
      }),
    })
    .await?;
  Ok(())
}
