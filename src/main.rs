// mod alloy_transport_implementation;

use axum::{
    Json,
    response::IntoResponse,
    routing::{post},
    Router
};
// TODO import and use alloy-json-rpc functions properly
use alloy_rpc_client::RpcClient as AlloyRpcClient;
use alloy_transport::Transport;
use alloy_transport_http::Http as AlloyHttp;
use reqwest::Client as ReqwestClient;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing::{debug, error, info, instrument};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tracing_flame::FlameLayer;
use url::Url;

// TODO refactor along with 3030 Port into config.toml and load from there
// Constants
const BSC_RPC_URL: &str = "https://data-seed-prebsc-1-s1.binance.org:8545";
const FLAMETRACE_PERFORMANCE: &bool = &true;

// TODO utoipa and OpenApi docs
// TODO error handling


#[derive(Clone, Debug, Deserialize)]
struct TransactionRequest {
    raw_transactions: [String; 2],
}

#[derive(Clone, Debug, Serialize)]
struct TransactionResponse {
    status: String,
    transaction_hash: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
struct BalanceRequest {
    account_id: String,
}

#[derive(Clone, Debug, Serialize)]
struct BalanceResponse {
    balance: String,
}



#[tokio::main]
async fn main() {
    // initialize tracing (aka logging)
    if *FLAMETRACE_PERFORMANCE {
        setup_global_subscriber();
        info!("default tracing setup with flametrace performance ENABLED");
    } else {
        tracing_subscriber::registry()
            .with(tracing_subscriber::fmt::layer())
            .init();
        info!("default tracing setup with flametrace performance DISABLED");
    }

    let app = Router::new()
        .route("/send_funding_and_user_signed_txns", post(handle_send_funding_and_user_signed_txns))
        //.route("/get_balance_for_account", post(handle_get_balance_for_account))
        // See https://docs.rs/tower-http/0.1.1/tower_http/trace/index.html for more details.
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
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


//#[instrument]
async fn handle_send_funding_and_user_signed_txns(Json(payload): Json<TransactionRequest>) -> Result<Json<TransactionResponse>, impl IntoResponse> {
    info!("Received /handle_send_funding_and_user_signed_txns request: {payload:#?}");
    // TODO convert to global RPC client
    let reqwest_client: ReqwestClient = ReqwestClient::new();
    let url: Url = Url::parse(BSC_RPC_URL)?;
    let transport_http: AlloyHttp<ReqwestClient> = AlloyHttp::new(
        url,
    );
    // let reqwest_transport = ReqwestTransport::new(reqwest_client.clone());
    let client: AlloyRpcClient<ReqwestClient> = AlloyRpcClient::new(
        transport_http.client().clone(),
        // reqwest_transport,
        true,
    );

    // Send the first transaction (funding)
    let tx1_request = client.prepare(
        "eth_sendRawTransaction",
        vec![json!(payload.raw_transactions[0])],
    );
    info!("Sending Funding Transaction: {tx1_request:#?}");
    let tx1_response = tx1_request.await.unwrap();
    info!("Funding Transaction Response: {tx1_response:#?}");

    // TODO: Wait for the transaction to be finalized and error handling

    // Send the second transaction
    let tx2_request = client.prepare(
        "eth_sendRawTransaction",
        vec![json!(payload.raw_transactions[1])],
    );
    info!("Sending User Foreign Chain Transaction: {tx2_request:#?}");
    let tx2_response = tx2_request.await.unwrap();
    info!("User Foreign Chain Transaction Response: {tx2_response:#?}");

    Ok(Json(TransactionResponse {
        status: "Success".into(),
        transaction_hash: Some(tx2_response),
    }))
}

// #[instrument]
// async fn handle_get_balance_for_account(Json(payload): Json<BalanceRequest>) -> Result<Json<BalanceResponse>, impl IntoResponse> {
//     let mut client = AlloyRpcClient::new(BSC_RPC_URL.to_string());  // TODO convert to global thread-safe RPC client
//
//     let balance_request = RPCRequest::new("eth_getBalance".to_string(), vec![json!(payload.account_id), json!("latest")]);
//     let balance_response: RPCResponse<String> = client.send_request(&balance_request).await.unwrap();
//
//     Ok(Json(BalanceResponse {
//         balance: balance_response.result,
//     }))
// }
