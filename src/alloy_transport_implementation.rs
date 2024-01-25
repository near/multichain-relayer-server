// use alloy_transport::Transport;
// use alloy_transport::r#trait::private::Sealed;
// use alloy_json_rpc::error::RpcError;
// use alloy_json_rpc::packet::{RequestPacket, ResponsePacket};
// use reqwest::Client;
// use std::future::Future;
// use std::pin::Pin;
//
// // This is your custom wrapper around ReqwestClient
// struct ReqwestTransport {
//     client: Client,
// }
//
// impl ReqwestTransport {
//     pub fn new(client: Client) -> Self {
//         ReqwestTransport { client }
//     }
//
//     // Implement other necessary methods here
// }
//
// impl Sealed for ReqwestTransport {}
//
// impl Transport for ReqwestTransport {
//     type Error = RpcError; // You need to define the error type properly
//     type Future = Pin<Box<dyn Future<Output = Result<ResponsePacket, Self::Error>>>>;
//     type Response = ResponsePacket;
//
//     fn call(&mut self, request: RequestPacket) -> Self::Future {
//         // Convert RequestPacket to a format suitable for Reqwest
//         // Make the actual HTTP call using ReqwestClient
//         // Convert the response back to ResponsePacket
//         // Handle errors appropriately
//
//         // Example, you'll need to adapt this to your specific case
//         Box::pin(async move {
//             let response = self.client.post(/* URL and request details here */).send().await;
//             match response {
//                 Ok(resp) => {
//                     // Convert resp to ResponsePacket and return
//                 },
//                 Err(e) => {
//                     // Convert e to RpcError and return
//                 }
//             }
//         })
//     }
// }
//
// // Usage
// let reqwest_client = Client::new();
// let transport = ReqwestTransport::new(reqwest_client);
// let rpc_client = AlloyRpcClient::new(transport, true);
