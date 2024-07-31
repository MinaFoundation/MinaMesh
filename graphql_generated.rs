use cynic;

#[cynic::schema("archive")]
mod schema {}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query")]
pub struct AnotherArchiveQuery {
  #[arguments(input: { address: "0x0", from: 0, to: 100 })]
  pub events: Vec<Option<EventOutput>>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct EventOutput {
  pub block_info: Option<BlockInfo>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct BlockInfo {
  pub height: i32,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query")]
pub struct SomethingArchiveQuery {
  #[arguments(input: { address: "0x0", from: 0, to: 100 })]
  pub events: Vec<Option<EventOutput>>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct EventOutput {
  pub block_info: Option<BlockInfo>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct BlockInfo {
  pub height: i32,
}
