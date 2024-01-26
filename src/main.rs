use axum::{
    Json,
    response::IntoResponse,
    routing::{post},
    Router
};
// TODO use custom middleware https://github.com/gakonst/ethers-rs/blob/88095ba47eb6a3507f0db1767353b387b27a6e98/ethers-providers/src/middleware.rs#L18
// example below:
/* from https://docs.rs/ethers/latest/ethers/middleware/trait.Middleware.html#required-associated-types
use ethers_providers::{Middleware, MiddlewareError};
use ethers_core::types::{U64, TransactionRequest, U256, transaction::eip2718::TypedTransaction, BlockId};
use thiserror::Error;
use async_trait::async_trait;

#[derive(Debug)]
struct MyMiddleware<M>(M);

#[derive(Error, Debug)]
pub enum MyError<M: Middleware> {
    #[error("{0}")]
    MiddlewareError(M::Error),

    // Add your middleware's specific errors here
}

impl<M: Middleware> MiddlewareError for MyError<M> {
    type Inner = M::Error;

    fn from_err(src: M::Error) -> MyError<M> {
        MyError::MiddlewareError(src)
    }

    fn as_inner(&self) -> Option<&Self::Inner> {
        match self {
            MyError::MiddlewareError(e) => Some(e),
            _ => None,
        }
    }
}

#[async_trait]
impl<M> Middleware for MyMiddleware<M>
where
    M: Middleware,
{
    type Error = MyError<M>;
    type Provider = M::Provider;
    type Inner = M;

    fn inner(&self) -> &M {
        &self.0
    }

    /// Overrides the default `get_block_number` method to always return 0
    async fn get_block_number(&self) -> Result<U64, Self::Error> {
        Ok(U64::zero())
    }

    /// Overrides the default `estimate_gas` method to log that it was called,
    /// before forwarding the call to the next layer.
    async fn estimate_gas(&self, tx: &TypedTransaction, block: Option<BlockId>) -> Result<U256, Self::Error> {
        println!("Estimating gas...");
        self.inner().estimate_gas(tx, block).await.map_err(MiddlewareError::from_err)
    }
}
 */


use ethers::{
    core::types::Bytes as EthBytes,
    providers::{Http, Provider::send_raw_transaction, JsonRpcClient, JsonRpcClientWrapper},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::net::SocketAddr;
use std::str::FromStr;
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
async fn handle_send_funding_and_user_signed_txns(Json(payload): Json<TransactionRequest>) -> impl IntoResponse {
    info!("Received /handle_send_funding_and_user_signed_txns request: {payload:#?}");
    // TODO convert to global RPC client
    let url = Url::parse(BSC_RPC_URL).unwrap();
    let provider = Http::new(url);

    let funding_txn_raw: EthBytes = EthBytes::from(payload.raw_transactions[0].into_bytes());
    let user_txn_raw: EthBytes = EthBytes::from(payload.raw_transactions[1].into_bytes());

    // Send the first transaction (funding)
    info!("Sending Funding Transaction: {funding_txn_raw:#?}");
    let tx1_response = provider.send_raw_transaction(
        funding_txn_raw,
    ).await?;
    // let tx1_response = JsonRpcClient::request(
    //     &provider,
    //     "eth_sendRawTransaction",
    //     funding_txn_raw,
    // ).await?;
    info!("Funding Transaction Response: {tx1_response:#?}");

    // TODO: Wait for the transaction to be finalized and error handling

    // Send the second transaction
    info!("Sending User Foreign Chain Transaction: {user_txn_raw:#?}");
    let tx2_response = provider.send_raw_transaction(
        user_txn_raw,
    ).await?;
    // let tx2_response = JsonRpcClient::request(
    //     &provider,
    //     "eth_sendRawTransaction",
    //     user_txn_raw,
    // ).await?;
    info!("User Foreign Chain Transaction Response: {tx2_response:#?}");

    tx2_response.into_response()
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
