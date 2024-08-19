#[cynic::schema("archive")]
mod schema {}

#[derive(cynic::QueryVariables, Debug)]
pub struct SomeDocVariables<'a> {
    pub address: &'a str,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "SomeDocVariables", schema = "archive")]
pub struct SomeDoc {
    #[arguments(input: { address: $address, from: 0, to: 1000 })]
    pub events: Vec<Option<EventOutput>>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "archive")]
pub struct EventOutput {
    pub block_info: Option<BlockInfo>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "archive")]
pub struct BlockInfo {
    pub chain_status: String,
    pub timestamp: String,
    pub state_hash: String,
    pub global_slot_since_hardfork: i32,
}

