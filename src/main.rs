mod util;
mod structs;

#[macro_use]
extern crate lazy_static;

use axum::{
    http::StatusCode,
    Json,
    response::IntoResponse,
    Router,
    routing::{get, post}
};
use ethers::{
    core::types::Bytes as EthBytes,
    core::types::U256,
};
use reqwest;
use serde_json::json;
use std::fs;
use std::net::SocketAddr;
use toml;
use tower_http::trace::TraceLayer;
use tracing::{error, info, instrument};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use structs::{BalanceRequestPayload, Config, EvmResponse, EvmRpcRequest, RpcError, TransactionRequest};
use tracing_flame::FlameLayer;


// Constants
lazy_static! {
    static ref CONFIG: Config = load_config();
}

fn load_config() -> Config {
    let config_str = fs::read_to_string("config.toml")
        .expect("Failed to read config.toml");
    toml::from_str(&config_str).expect("Failed to parse config.toml")
}
// TODO support multiple chains in code
// TODO utoipa and OpenApi docs


#[tokio::main]
async fn main() {
    // initialize tracing (aka logging)
    if CONFIG.flametrace_performance {
        setup_global_subscriber();
        info!("default tracing setup with flametrace performance ENABLED");
    } else {
        tracing_subscriber::registry()
            .with(tracing_subscriber::fmt::layer())
            .init();
        info!("default tracing setup with flametrace performance DISABLED");
    }

    let app = Router::new()
        .route("/send_funding_and_user_signed_txns", post(send_funding_and_user_signed_txns))
        .route("/get_balance_for_account", get(get_balance_for_account))
        // See https://docs.rs/tower-http/0.1.1/tower_http/trace/index.html for more details.
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3030));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}


fn setup_global_subscriber() -> impl Drop {
    let fmt_layer = tracing_subscriber::fmt::Layer::default();

    let (flame_layer, _guard) = FlameLayer::with_file("./tracing.folded").unwrap();

    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(flame_layer)
        .init();
    _guard
}


#[instrument]
async fn send_funding_and_user_signed_txns(Json(payload): Json<TransactionRequest>) -> impl IntoResponse {
    info!("Received /send_funding_and_user_signed_txns request: {payload:#?}");

    let funding_txn_raw: EthBytes = EthBytes::from(payload.raw_transactions[0].clone().into_bytes());
    let user_txn_raw: EthBytes = EthBytes::from(payload.raw_transactions[1].clone().into_bytes());

    // Send the first transaction (funding)
    let evm_funding_request = EvmRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "eth_sendRawTransaction".to_string(),
        params: vec![funding_txn_raw.to_string()],
        id: 1,  // if needed change id
    };
    info!("Sending Funding Transaction: {evm_funding_request:#?}");

    let client = reqwest::Client::new();
    let evm_funding_http_response = match client.post(CONFIG.bsc_testnet_rpc_url.clone())
        .json(&evm_funding_request)
        .send()
        .await {
        Ok(res) => res,
        Err(_) => {
            error!("Failed to parse EVM funding response");
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to send EVM funding request").into_response();
        },
    };

    let evm_funding_response: EvmResponse = match evm_funding_http_response.json().await {
        Ok(res) => res,
        Err(_) => {
            error!("Failed to parse funding response");
            return (StatusCode::BAD_REQUEST, "Failed to parse funding response").into_response();
        },
    };
    info!("Funding Response: {evm_funding_response:#?}");
    if evm_funding_response.error.is_some() {
        let result: RpcError = evm_funding_response.error.unwrap();
        let result_str = json!(result).to_string();
        return (StatusCode::BAD_REQUEST, result_str).into_response()
    }

    // Send the second transaction (actual user txn)
    let evm_user_txn_request = EvmRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "eth_sendRawTransaction".to_string(),
        params: vec![user_txn_raw.to_string()],
        id: 1,  // if needed change id
    };
    info!("Sending User Transaction: {evm_user_txn_request:#?}");

    let client = reqwest::Client::new();
    let response = match client.post(CONFIG.bsc_testnet_rpc_url.clone())
        .json(&evm_user_txn_request)
        .send()
        .await {
        Ok(res) => res,
        Err(_) => {
            error!("Failed to parse user foreign chain txn response");
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to send user foreign chain txn request").into_response();
        },
    };

    let evm_user_txn_response: EvmResponse = match response.json().await {
        Ok(res) => res,
        Err(_) => {
            error!("Failed to parse user foreign chain txn response");
            return (StatusCode::BAD_REQUEST, "Failed to parse user foreign chain txn response").into_response();
        },
    };
    info!("User Foreign Chain Txn Response: {evm_user_txn_response:#?}");
    return if evm_user_txn_response.result.is_some() {
        let result: String = evm_user_txn_response.result.unwrap();
        (StatusCode::OK, result).into_response()
    } else {
        let result: RpcError = evm_user_txn_response.error.unwrap();
        let result_str = json!(result).to_string();
        (StatusCode::BAD_REQUEST, result_str).into_response()
    }

}

#[instrument]
async fn get_balance_for_account(Json(payload): Json<BalanceRequestPayload>) -> impl IntoResponse {
    let address = payload.address;
    let evm_balance_request = EvmRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "eth_getBalance".to_string(),
        params: vec![address.clone(), "latest".to_string()],
        id: 1,  // if needed change id
    };
    info!("Balance Request: {evm_balance_request:#?}");

    let client = reqwest::Client::new();
    let evm_balance_http_response = match client.post(CONFIG.bsc_testnet_rpc_url.clone())
        .json(&evm_balance_request)
        .send()
        .await {
        Ok(res) => res,
        Err(_) => {
            error!("Failed to parse EVM balance response");
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to send EVM balance request").into_response();
        },
    };

    let evm_balance_response: EvmResponse = match evm_balance_http_response.json().await {
        Ok(res) => res,
        Err(_) => {
            error!("Failed to parse response");
            return (StatusCode::BAD_REQUEST, "Failed to parse response").into_response();
        },
    };
    // default to 0 balance if not found
    let hex_str: String = evm_balance_response.result.unwrap_or("0x0".to_string());
    let balance: U256 = util::convert_hex_to_u256(&hex_str).unwrap();
    info!("balance: {balance:#?} for account: {address:#?}");

    (StatusCode::OK, balance.to_string()).into_response()
}
