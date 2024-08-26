#[cynic::schema("mina")]
mod schema {}

#[derive(cynic::QueryVariables, Debug)]
pub struct QueryBalanceVariables {
    pub public_key: PublicKey,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "query", schema = "mina")]
pub struct QueryNetworkId {
    #[cynic(rename = "networkID")]
    pub network_id: String,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "query", schema = "mina")]
pub struct QueryGenesisBlockIdentifier {
    pub genesis_block: Block,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "query", schema = "mina")]
pub struct QueryNetworkStatus {
    #[arguments(maxLength: 1)]
    pub best_chain: Option<Vec<Block2>>,
    pub daemon_status: DaemonStatus,
    pub sync_status: SyncStatus,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "query", variables = "QueryBalanceVariables", schema = "mina")]
pub struct QueryBalance {
    #[arguments(publicKey: $public_key)]
    pub account: Option<Account>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "mina")]
pub struct DaemonStatus {
    pub peers: Vec<Peer>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "mina")]
pub struct Peer {
    pub peer_id: String,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "mina")]
pub struct Block {
    pub state_hash: StateHash,
    pub protocol_state: ProtocolState,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "mina")]
pub struct ProtocolState {
    pub consensus_state: ConsensusState,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Block", schema = "mina")]
pub struct Block2 {
    pub state_hash: StateHash,
    pub protocol_state: ProtocolState2,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "ProtocolState", schema = "mina")]
pub struct ProtocolState2 {
    pub blockchain_state: BlockchainState,
    pub consensus_state: ConsensusState,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "mina")]
pub struct ConsensusState {
    pub block_height: Length,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "mina")]
pub struct BlockchainState {
    pub utc_date: BlockTime,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "mina")]
pub struct Account {
    pub balance: AnnotatedBalance,
    pub nonce: Option<AccountNonce>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "mina")]
pub struct AnnotatedBalance {
    pub block_height: Length,
    pub state_hash: Option<StateHash>,
    pub liquid: Option<Balance>,
    pub total: Balance,
}

#[derive(cynic::Enum, Clone, Copy, Debug)]
#[cynic(schema = "mina")]
pub enum SyncStatus {
    Connecting,
    Listening,
    Offline,
    Bootstrap,
    Synced,
    Catchup,
}

#[derive(cynic::Scalar, Debug, Clone)]
pub struct AccountNonce(pub String);

#[derive(cynic::Scalar, Debug, Clone)]
pub struct Balance(pub String);

#[derive(cynic::Scalar, Debug, Clone)]
pub struct BlockTime(pub String);

#[derive(cynic::Scalar, Debug, Clone)]
pub struct Length(pub String);

#[derive(cynic::Scalar, Debug, Clone)]
pub struct PublicKey(pub String);

#[derive(cynic::Scalar, Debug, Clone)]
pub struct StateHash(pub String);
