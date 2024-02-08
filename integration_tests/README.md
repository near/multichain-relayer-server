quick and dirty integration tests for the multichain relayer system in python

# Install
Create a new virtual environment. Activate it and install dependencies by running: 
```shell
pip install eth-account eth-rlp eth-utils requests web3
```

# Run
1. Update the transaction details of the EVM transaction you want to send in `generate_rlp_evm_txn.py` then run the script and save the RLP hex string output.
   
2. Construct the signed transaction using the [near-cli-rs](https://github.com/near/near-cli-rs). 
The receiver account ID should be the gas station contract. 
You will need 2 actions if you want the gas station to cover your gas cost on the foreign chain: 
1 to send the NEAR equivalent and 1 function call to the gas station. 
You should transfer the amount of NEAR that's needed to cover gas both on NEAR and on the foreign chain. 
You also need to paste in the RLP generated hex for the EVM transaction you want on the other chain generated in step 1. 
When it asks you to send or display, choose display as you'll need to paste the b64 encoded output for step 3. 
Example below:
```shell
near-cli contract call-function as-transaction canhazgas.testnet create_transaction json-args '{"transaction_rlp_hex":"eb80851bf08eb000825208947b965bdb7f0464843572eb2b8c17bdf27b720b14872386f26fc1000080808080","use_paymaster":true}' prepaid-gas '100.000 TeraGas' attached-deposit '0.5 NEAR' sign-as nomnomnom.testnet network-config testnet sign-with-keychain
```
3. Paste the base64 encoded transaction as the value of `b64_near_transaction_with_rlp_hex` var in `integration_test.py`

4. Run the integration test:
```shell
python3 integration_test.py
```