use alloy_consensus::transaction::eip1559::{TxEip1559, AccessList, TransactionPayload};
use alloy_json_rpc::{Request, Client, Method};
use alloy_primitives::{address, Address, Signature};
use alloy_signer::{LocalWallet, Signer, SignerSync};

fn create_eip1559_transaction(account_id: Address, amount: &str, chain_id: u64) -> TxEip1559 {
    TxEip1559 {
        chain_id: Some(chain_id), // Replace with BSC chain ID
        nonce: YOUR_TREASURY_ACCOUNT_NONCE,     // TODO replace with actual value
        max_priority_fee_per_gas: PRIORITY_FEE,  // TODO replace with actual value
        max_fee_per_gas: MAX_FEE,  // TODO replace with actual value
        gas_limit: GAS_LIMIT,  // TODO replace with actual value
        action: TransactionAction::Call(account_id), // User account address
        value: amount, // Amount of Native Gas to transfer
        data: TransactionPayload::None,
        access_list: AccessList::default(),
    }
}

async fn create_evm_signature(chain_id: u64) -> Signature {
    // TODO replace with actual signature
    // https://github.com/alloy-rs/alloy/tree/main/crates/signer
    Signature::default()
}

async fn fund_user_account(account_id: &str, amount: &str, chain_id: u64) {
    let signed_funding_txn = create_eip1559_transaction(
        address!(account_id),
        amount,
        chain_id,
    ).into_signed(
        create_evm_signature(chain_id.clone())
    );

    send_funding_transaction(signed_funding_txn).await;
}

async fn send_funding_transaction(signed_tx: SignedTransaction) {
    let client = Client::new(BSC_JSON_RPC_URL); // Replace with the BSC JSON-RPC URL
    let raw_tx = signed_tx.raw(); // Serialize the signed transaction

    let request = Request::new(Method::EthSendRawTransaction, vec![raw_tx.into()]);
    let response = client.execute(request).await.expect("Failed to send transaction");

    println!("Transaction sent: {:?}", response);
}
