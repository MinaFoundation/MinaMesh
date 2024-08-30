// This file is generated by `build.rs`. Do not edit it directly.

#[cynic::schema("mina")]
mod schema {}

#[derive(cynic::QueryVariables, Debug)]
pub struct QueryBalanceVariables {
    pub public_key: PublicKey,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct QueryMempoolTransactionsVariables<'a> {
    pub hashes: Option<Vec<&'a str>>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct QueryBlockTransactionsVariables<'a> {
    pub state_hash: Option<&'a str>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "query", schema = "mina")]
pub struct QueryNetworkId {
    #[cynic(rename = "networkID")]
    pub network_id: String,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "query", variables = "QueryMempoolTransactionsVariables", schema = "mina")]
pub struct QueryMempoolTransactions {
    pub initial_peers: Vec<String>,
    pub daemon_status: DaemonStatus,
    #[arguments(hashes: $hashes)]
    pub pooled_user_commands: Vec<UserCommand>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "query", schema = "mina")]
pub struct QueryMempool {
    pub initial_peers: Vec<String>,
    pub daemon_status: DaemonStatus2,
    #[arguments(publicKey: null)]
    pub pooled_user_commands: Vec<UserCommand2>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "UserCommand", schema = "mina")]
pub struct UserCommand2 {
    pub hash: TransactionHash,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "query", schema = "mina")]
pub struct QueryGenesisBlockIdentifier {
    pub genesis_block: Block,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "query", variables = "QueryBlockTransactionsVariables", schema = "mina")]
pub struct QueryBlockTransactions {
    #[arguments(stateHash: $state_hash)]
    pub block: Block2,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "query", schema = "mina")]
pub struct QueryNetworkStatus {
    #[arguments(maxLength: 1)]
    pub best_chain: Option<Vec<Block3>>,
    pub daemon_status: DaemonStatus3,
    pub sync_status: SyncStatus,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "query", variables = "QueryBalanceVariables", schema = "mina")]
pub struct QueryBalance {
    #[arguments(publicKey: $public_key)]
    pub account: Option<Account>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "DaemonStatus", schema = "mina")]
pub struct DaemonStatus3 {
    pub peers: Vec<Peer>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "mina")]
pub struct Peer {
    pub peer_id: String,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "mina")]
pub struct DaemonStatus {
    pub chain_id: String,
    pub peers: Vec<Peer2>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Peer", schema = "mina")]
pub struct Peer2 {
    pub host: String,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "DaemonStatus", schema = "mina")]
pub struct DaemonStatus2 {
    pub chain_id: String,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Block", schema = "mina")]
pub struct Block2 {
    pub transactions: Transactions,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "mina")]
pub struct Transactions {
    pub user_commands: Vec<UserCommand>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "mina")]
pub struct UserCommand {
    pub amount: Amount,
    pub fee: Fee,
    pub source: Account2,
    pub fee_token: TokenId,
    pub hash: TransactionHash,
    pub kind: UserCommandKind,
    pub memo: String,
    pub nonce: i32,
    pub receiver: Account2,
    pub token: TokenId,
    pub valid_until: Globalslot,
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
pub struct Block3 {
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
#[cynic(graphql_type = "Account", schema = "mina")]
pub struct Account2 {
    pub public_key: PublicKey,
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
pub struct Amount(pub String);

#[derive(cynic::Scalar, Debug, Clone)]
pub struct Balance(pub String);

#[derive(cynic::Scalar, Debug, Clone)]
pub struct BlockTime(pub String);

#[derive(cynic::Scalar, Debug, Clone)]
pub struct Fee(pub String);

#[derive(cynic::Scalar, Debug, Clone)]
pub struct Globalslot(pub String);

#[derive(cynic::Scalar, Debug, Clone)]
pub struct Length(pub String);

#[derive(cynic::Scalar, Debug, Clone)]
pub struct PublicKey(pub String);

#[derive(cynic::Scalar, Debug, Clone)]
pub struct StateHash(pub String);

#[derive(cynic::Scalar, Debug, Clone)]
pub struct TokenId(pub String);

#[derive(cynic::Scalar, Debug, Clone)]
pub struct TransactionHash(pub String);

#[derive(cynic::Scalar, Debug, Clone)]
pub struct UserCommandKind(pub String);

