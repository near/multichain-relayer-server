use ethers_core::types::{TransactionRequest, U256};

// from https://github.com/near/multichain-gas-station-contract/blob/3e544fa5f4fa383f8026d528c7a7c6c7c12432f8/tests/tests.rs#L160-L173
#[test]
//#[ignore = "generate a payload signable by the contract"]
fn generate_eth_rlp_hex() {
    let eth_transaction = TransactionRequest {
        chain_id: Some(97.into()),
        from: None,
        to: Some("0x7b965bDB7F0464843572Eb2B8c17BdF27B720b14".into()),
        data: None,
        gas: Some(21000.into()),
        gas_price: Some(120.into()),
        value: Some(U256::from(10000000000000000_i64)),  // 0.01 BNB
        nonce: Some(0.into()),
    };

    println!("{}", hex::encode(eth_transaction.rlp()));
}