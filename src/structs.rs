use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct Config {
    pub(crate) flametrace_performance: bool,
    pub(crate) chains: HashMap<String, ChainConfig>,
}

#[derive(Deserialize)]
pub struct ChainConfig {
    #[allow(dead_code)] // may want to use this in the future
    pub(crate) name: String,
    pub(crate) rpc_url: String,
    pub(crate) supported: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TransactionRequest {
    pub(crate) signed_transactions: [String; 2],
    pub(crate) foreign_chain_id: String,
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
