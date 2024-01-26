// use alloy_json_rpc::error::RpcError;
// use alloy_json_rpc::packet::{RequestPacket, ResponsePacket};
// use reqwest::Client;
// use std::future::Future;
// use std::pin::Pin;
// use std::task::{Context, Poll};
// use tower::Service;
// use crate::BSC_RPC_URL;
//
// struct ReqwestTransport {
//     client: Client,
// }
//
// impl ReqwestTransport {
//     pub fn new(client: Client) -> Self {
//         ReqwestTransport { client }
//     }
// }
//
// impl Service<RequestPacket> for ReqwestTransport {
//     type Response = ResponsePacket;
//     type Error = RpcError;
//     type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;
//
//     fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
//         Poll::Ready(Ok(())) // Always ready to send a request
//     }
//
//     fn call(&mut self, req: RequestPacket) -> Self::Future {
//         let client = self.client.clone();
//         let request_body = serde_json::to_string(&req).expect("Failed to serialize request");
//
//         Box::pin(async move {
//             let response = client.post(BSC_RPC_URL)
//                 .body(request_body)
//                 .send()
//                 .await;
//
//             match response {
//                 Ok(resp) => {
//                     let status = resp.status();
//                     if status.is_success() {
//                         let body = resp.text().await.expect("Failed to read response text");
//                         let response_packet: ResponsePacket = serde_json::from_str(&body)
//                             .expect("Failed to deserialize response");
//                         Ok(response_packet)
//                     } else {
//                         Err(RpcError::TransportError(format!("HTTP error: {}", status)))
//                     }
//                 },
//                 Err(e) => Err(RpcError::TransportError(format!("Reqwest error: {}", e))),
//             }
//         })
//     }
// }
