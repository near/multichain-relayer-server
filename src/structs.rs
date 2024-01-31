use std::collections::HashMap;
use serde::{Deserialize, Serialize};


#[derive(Deserialize)]
pub struct Config {
    // TODO get rid of this post ETHDenver
    pub(crate) bsc_testnet_rpc_url: String,
    pub(crate) flametrace_performance: bool,
    pub(crate) chains: HashMap<String, ChainConfig>,
}

#[derive(Deserialize)]
pub struct ChainConfig {
    pub(crate) name: String,
    pub(crate) rpc_url: String,
}


#[derive(Clone, Debug, Deserialize)]
pub struct TransactionRequest {
    pub(crate) raw_transactions: [String; 2],
    pub(crate) chain_id: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct TransactionResponse {
    pub(crate) status: String,
    pub(crate) transaction_hash: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BalanceRequestPayload {
    pub(crate) address: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EvmRpcRequest {
    pub jsonrpc: String,
    pub(crate) method: String,
    pub(crate) params: Vec<String>,
    pub(crate) id: u32,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct RpcError {
    pub(crate) code: i32,
    pub(crate) message: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EvmResponse {
    pub(crate) jsonrpc: String,
    pub(crate) id: u32,
    pub(crate) result: Option<String>,
    pub(crate) error: Option<RpcError>,
}
