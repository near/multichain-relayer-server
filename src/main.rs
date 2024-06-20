mod structs;
mod util;

#[macro_use]
extern crate lazy_static;

use axum::{http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use serde_json::json;
use std::collections::HashSet;
use std::fs;
use std::net::SocketAddr;
use structs::{Config, EvmResponse, EvmRpcRequest, RpcError, TransactionRequest};
use tower_http::trace::TraceLayer;
use tracing::{error, info, instrument};
use tracing_flame::FlameLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// Constants
lazy_static! {
    static ref CONFIG: Config = load_config();
    static ref SUPPORTED_CHAINS: HashSet<String> = {
        CONFIG
            .chains
            .iter()
            .filter_map(|(key, value)| {
                if value.supported {
                    Some(key.clone())
                } else {
                    None
                }
            })
            .collect()
    };
}

fn load_config() -> Config {
    let config_str = fs::read_to_string("config.toml").expect("Failed to read config.toml");
    toml::from_str(&config_str).expect("Failed to parse config.toml")
}
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
    info!("Configured support for {} chain(s)", SUPPORTED_CHAINS.len());

    let app = Router::new()
        .route(
            "/send_funding_and_user_signed_txns",
            post(send_funding_and_user_signed_txns),
        )
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
async fn send_funding_and_user_signed_txns(
    Json(payload): Json<TransactionRequest>,
) -> impl IntoResponse {
    info!("Received /send_funding_and_user_signed_txns request: {payload:#?}");

    let funding_txn_str: String = payload.signed_transactions[0].clone();
    let user_txn_str: String = payload.signed_transactions[1].clone();
    let chain_id: String = payload.foreign_chain_id.clone();
    if !SUPPORTED_CHAINS.contains(&chain_id) {
        let error_msg = format!("Unsupported chain_id: {chain_id}");
        error!("{error_msg}");
        return (StatusCode::BAD_REQUEST, error_msg).into_response();
    }
    let rpc_url: String = CONFIG.chains.get(&chain_id).unwrap().rpc_url.clone();

    // Send the first transaction (funding)
    let evm_funding_request = EvmRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "eth_sendRawTransaction".to_string(),
        params: vec![funding_txn_str.clone()],
        id: 1, // if needed change id
    };
    info!("Sending Funding Transaction: {evm_funding_request:#?}");

    let client = reqwest::Client::new();
    let evm_funding_http_response = match client
        .post(rpc_url.clone())
        .json(&evm_funding_request)
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => {
            error!("Failed to parse EVM funding response");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to send EVM funding request",
            )
                .into_response();
        }
    };

    let evm_funding_response: EvmResponse = match evm_funding_http_response.json().await {
        Ok(res) => res,
        Err(_) => {
            error!("Failed to parse funding response");
            return (StatusCode::BAD_REQUEST, "Failed to parse funding response").into_response();
        }
    };
    info!("Funding Response: {evm_funding_response:#?}");
    if evm_funding_response.error.is_some() {
        let result: RpcError = evm_funding_response.error.unwrap();
        let result_str = json!(result).to_string();
        return (StatusCode::BAD_REQUEST, result_str).into_response();
    }

    // Send the second transaction (actual user txn)
    let evm_user_txn_request = EvmRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "eth_sendRawTransaction".to_string(),
        params: vec![user_txn_str.clone()],
        id: 1, // if needed change id
    };
    info!("Sending User Transaction: {evm_user_txn_request:#?}");

    let client = reqwest::Client::new();
    let response = match client
        .post(rpc_url.clone())
        .json(&evm_user_txn_request)
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => {
            error!("Failed to parse user foreign chain txn response");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to send user foreign chain txn request",
            )
                .into_response();
        }
    };

    let evm_user_txn_response: EvmResponse = match response.json().await {
        Ok(res) => res,
        Err(_) => {
            error!("Failed to parse user foreign chain txn response");
            return (
                StatusCode::BAD_REQUEST,
                "Failed to parse user foreign chain txn response",
            )
                .into_response();
        }
    };
    info!("User Foreign Chain Txn Response: {evm_user_txn_response:#?}");
    return if evm_user_txn_response.result.is_some() {
        let result: String = evm_user_txn_response.result.unwrap();
        (StatusCode::OK, result).into_response()
    } else {
        let result: RpcError = evm_user_txn_response.error.unwrap();
        let result_str = json!(result).to_string();
        (StatusCode::BAD_REQUEST, result_str).into_response()
    };
}
