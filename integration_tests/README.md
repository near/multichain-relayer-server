quick and dirty integration tests for the multichain relayer system in python


# Run
1. Update the transaction details of the EVM transaction you want to send in `generate_rlp_evm_txn.py` then run the script and save the RLP hex string output.
   
2. Ensure the [gas station indexer](https://github.com/near/gas-station-event-indexer/tree/main) is running locally
3. Ensure the multichain server is configured correctly (`config.toml`) and running: `cargo run`

4. Construct the signed transaction using the [near-cli-rs](https://github.com/near/near-cli-rs).
   The receiver account ID should be the gas station contract.
   You will need 2 actions if you want the gas station to cover your gas cost on the foreign chain:
   1 to send the NEAR equivalent and 1 function call to the gas station.
   You should transfer the amount of NEAR that's needed to cover gas both on NEAR and on the foreign chain.
   You also need to paste in the RLP generated hex for the EVM transaction you want on the other chain generated in step 1.
   When it asks you to send or display, choose send.
   Example below:
```shell
near contract call-function as-transaction canhazgas.testnet create_transaction json-args '{"transaction_rlp_hex":"eb80851bf08eb000825208947b965bdb7f0464843572eb2b8c17bdf27b720b14872386f26fc1000080808080","use_paymaster":true}' prepaid-gas '100.000 TeraGas' attached-deposit '0.5 NEAR' sign-as nomnomnom.testnet network-config testnet sign-with-keychain
```
5. Call sign_next 2x TODO