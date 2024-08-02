mod graphql_generated;

use clap::{Parser, Subcommand};
use cynic::http::ReqwestExt;
use cynic::{GraphQlResponse, QueryBuilder, QueryFragment};
use graphql_generated::archive;
use tokio;

#[derive(Debug, Parser)]
#[command(name = "mina-mesh", version, about = "A Mesh-compliant Server for Mina", propagate_version = true, author)]
struct MinaMeshArgs {
  #[arg(long, short = 'v', default_value = "false", global = true)]
  verbose: bool,
  #[command(subcommand)]
  command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
  #[command(about = "Start the Mina Mesh Server")]
  Start {
    #[arg(long, short = 'p', default_value = "8686")]
    port: u16,
    #[arg(long, default_value = "https://api.minascan.io/node/devnet/v1/graphql")]
    mina_proxy_url: String,
    #[arg(long, default_value = "https://api.minascan.io/archive/devnet/v1/graphql")]
    archive_url: String,
  },
}

#[tokio::main]
async fn main() {
  let args = MinaMeshArgs::parse();
  let MinaMeshArgs { command, .. } = args;
  match command.unwrap() {
    Commands::Start { archive_url, mina_proxy_url, .. } => {
      println!("Starting server.");
      let client = GraphQLClient { archive_url, mina_proxy_url, inner: reqwest::Client::new() };
      let operation = archive::SomethingArchiveQuery::build(());
      let response = client.archive(operation).await;
      println!("{:?}", response);
    }
  }
}

struct GraphQLClient {
  pub inner: reqwest::Client,
  pub mina_proxy_url: String,
  pub archive_url: String,
}

impl GraphQLClient {
  pub async fn archive<ResponseData, Vars>(
    self,
    operation: cynic::Operation<ResponseData, Vars>,
  ) -> GraphQlResponse<ResponseData>
  where
    Vars: serde::Serialize,
    ResponseData: serde::de::DeserializeOwned + 'static,
  {
    self.inner.post(self.archive_url).run_graphql(operation).await.unwrap()
  }
}
