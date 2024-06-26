## Prerequisites
You need to have the following installed:
- Python >=3.10
- Rust
- [NEAR CLI-RS](https://github.com/near/near-cli-rs).
    - Make sure to configure it with the correct network and account.

# Run
From root of multichain-relayer-server repo directory run: 
```
python3 integration_tests/integration_test.py
```
with the optional `--verbose` flag to print subprocess output.

In separate terminals, run the following:
- [gas station indexer](https://github.com/near/gas-station-event-indexer/tree/main) locally with the correct values in the `config.toml` file: `make run`
- multichain server (this repo) is configured correctly (`config.toml`) and run: `cargo run`

## Manual Steps
Instructions on how to manually perform end-to-end tests on the entire multichain relayer system including the gas station contract, indexer, and relayer server.

1.  The following instructions in 1.i-iii are only need to be called once to initialize the account on the gas station. Make sure to replace the `<account_id>` (string) with the account you want to initialize and `<token_id>` (integer) with the token id of the NFT you minted in step ii:
    1. Registration / Storage Deposit:
    ```shell
    near contract call-function as-transaction v2.nft.kagi.testnet \
      storage_deposit json-args {} prepaid-gas '100.0 Tgas' attached-deposit '1 NEAR' \ 
      sign-as <account_id>.testnet network-config testnet sign-with-keychain send
    ```
    2. Mint NFT - make sure to save the token id from the logs of this call
    ```shell
    near contract call-function as-transaction v2.nft.kagi.testnet \ 
      mint json-args {} prepaid-gas '100.0 Tgas' attached-deposit '0 NEAR' \
      sign-as <account_id>.testnet network-config testnet sign-with-keychain send
    ```
    3. Approve the Gas Station for this Token
    ```shell
    near contract call-function as-transaction v2.nft.kagi.testnet \
      ckt_approve_call json-args '{"token_id":"<token_id>","account_id":"canhazgas.testnet","msg":""}' \
      prepaid-gas '100.0 Tgas' attached-deposit '0 NEAR' \
      sign-as <account_id>.testnet network-config testnet sign-with-keychain send
    ```
2. Get paymaster info for the chain you want to send to from the gas station contract, then optionally manually set nonces:
    ```shell
    near contract call-function as-transaction canhazgas.testnet \
     get_paymasters json-args '{"chain_id": "<chain_id>"}' \
     prepaid-gas '100.0 Tgas' attached-deposit '0 NEAR' \
     sign-as <account_id>.testnet network-config testnet sign-with-keychain send
    ```
   which  returns something like:
    ```
    --- Result -------------------------
    [
      {
        "foreign_address": "0x98c6C99176911AD4E7fc7413eDF09956B2D7F0F8",
        "minimum_available_balance": "99886599999948172000",
        "nonce": 28,
        "token_id": "1"
      }
    ]
    ------------------------------------
    ```
   1. You may need to manually set the nonce for the paymaster to be able to send the transaction:
   ```shell
   near contract call-function as-transaction canhazgas.testnet \
    get_paymasters json-args '{"chain_id": "<chain_id>"}' \
    prepaid-gas '100.000 Tgas' \
    attached-deposit '0 NEAR' \
    sign-as <account_id>.testnet \
    network-config testnet \
    sign-with-keychain send
    ```
3. Update the transaction details of the EVM transaction you want to send in `generate_rlp_evm_txn.py` then run the script and save the RLP hex string output. 
   1. If that doesn't work, try running the `generate_eip1559_rlp_hex()` test in tests/tests.rs - python and rust output different hex rlp excoded txns :/  
   
4. Ensure the [gas station indexer](https://github.com/near/gas-station-event-indexer/tree/main) is running locally with the correct values in the `config.toml` file
5. Ensure the multichain server is configured correctly (`config.toml`) and running: `cargo run`

6. Construct the signed transaction using the [near-cli-rs](https://github.com/near/near-cli-rs).
   The receiver account ID should be the gas station contract.
   You will need 2 actions if you want the gas station to cover your gas cost on the foreign chain:
   1 to send the NEAR equivalent and 1 function call to the gas station.
   You should transfer the amount of NEAR that's needed to cover gas both on NEAR and on the foreign chain.
   You also need to paste in the RLP generated hex for the EVM transaction you want on the other chain generated in step 1.
   When it asks you to send or display, choose send.
   Example below:
    ```shell
    near contract call-function as-transaction canhazgas.testnet \
      create_transaction json-args '{"transaction_rlp_hex":"eb80851bf08eb000825208947b965bdb7f0464843572eb2b8c17bdf27b720b14872386f26fc1000080808080","use_paymaster":true,"token_id":"<token_id>"}' \
      prepaid-gas '100.000 TeraGas' attached-deposit '0.5 NEAR' \
      sign-as <account_id>.testnet \
      network-config testnet sign-with-keychain send
    ```
   which returns something like:
    ```
     --- Result -------------------------
      {
        "id": "29",
        "pending_signature_count": 2
      }
      ------------------------------------
     ```
7. Get the `"id"` from the receipts from the result in the previous step, and use that to call `sign_next` twice:
    ```shell
    near contract call-function as-transaction canhazgas.testnet \
      sign_next json-args '{"id":"<id>"}' \
      prepaid-gas '300.0 Tgas' \
      attached-deposit '0 NEAR' \
      sign-as <account_id>.testnet \
      network-config testnet \
      sign-with-keychain send
    ```
   
NOTE: this step will be updated soon, as support for yield/resume calls is implemented on MPC contract. 
8. Watch the output of the gas station event indexer to see the transactions being emitted by the gas station contract.
9. Watch the output of the multichain relayer server to see the transactions being sent to the foreign chain.


#### Optional for testing purposes:
- Instead of creating the signed txn and calling the gas station contract to sign the transaction, you can get the recently signed transactions by calling: 
  - `near contract call-function as-read-only canhazgas.testnet list_signed_transaction_sequences_after json-args '{"block_height":"157111000"}' network-config testnet now` replacing the blockheight with a more recent blockheight
  - This will return something like the output below. Take an individual entry in the list of jsons and send that as the payload of a POST request to the `/send_funding_and_user_signed_txns` endpoint:
```json
[
  {
    "created_by_account_id": "b807806adcb73f6aecb5ed98bb8bd7bbe7bbf8ed342596ab700ef6b050abc4c3",
    "foreign_chain_id": "97",
    "signed_transactions": [
      "0x02f873610385174876e80085174876e80082520894c89663ac6d169bc3e2e0a99d9fe96f2e82bcc307870eebe0b40e800080c080a0712d44ba4cd7567764231e21f054c5e7d008055222820e9d5ba148ede48755f7a06e8b812d37047593fc51fce7254ea7aef89927cada729bc903cd36fa9659dce4",
      "0x02f873618085174876e80085174876e80082520894ef55a8bdf4498ea0af88bc54efb29608bb25e130872aa1efb94e000080c080a017d7024fe9e32ad8da1181729fac7e6a45311c47bf59f2b5a8b5e9fe002c0617a04ad725b362cf12c6e066c5b0b7ecbbf08f5e4d0a240337e6ddc8076f0528e3e5"
    ]
  },
...
  {
    "created_by_account_id": "b807806adcb73f6aecb5ed98bb8bd7bbe7bbf8ed342596ab700ef6b050abc4c3",
    "foreign_chain_id": "97",
    "signed_transactions": [
      "0x02f873610185174876e80085174876e80082520894c89663ac6d169bc3e2e0a99d9fe96f2e82bcc307870eebe0b40e800080c001a0ff19fe769246de8483b986e5aeaa3360bfb74f238e2a91ea353dac9aad9e24a0a020485dcd2c64172b9bc058b7813646dafbf2f27d51aae388b074e514fdb6de05",
      "0x02f873618085174876e80085174876e80082520894ef55a8bdf4498ea0af88bc54efb29608bb25e130872e2f6e5e14800080c001a0dac67c383e8de3211f3c5d360cc2e9a21d160711fc3f80113ac525169317e2eca07140a1d0d1528b6eaf9fac4bb1bd44c1c4f63bb956292b0211a0dad1748e2eea"
    ]
  }
]
```
