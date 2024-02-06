quick and dirty integration tests for the multichain relayer system in python

# Install
Create a new virtual environment. Activate it and run: 
```shell
pip install requests eth-rlp eth-utils
```

# Run
1. Construct the signed transaction using the [near-cli-rs](https://github.com/near/near-cli-rs). The receiver account ID should be the gas station contract. You will need 2 actions if you want the gas station to cover your gas cost on the foreign chain: 1 to send the NEAR equivalent and 1 function call to the gas station. You should transfer the amount of NEAR that's needed to cover gas both on NEAR and on the foreign chain. You also need to paste in the RLP generated hex for the EVM transaction you want on the other chain. Example below.
```shell
awit@Anthonys-MacBook-Pro ~ % near
✔  What are you up to? (select one of the options with the up-down arrows on your keyboard and press Enter) · transaction - Operate transactions
✔ Сhoose action for transaction · construct-transaction  - Construct a new transaction
What is the sender account ID?: nomnomnom.testnet
What is the receiver account ID?: canhazgas.testnet
✔ Select an action that you want to add to the action: · add-action   - Select a new action
✔ Select an action that you want to add to the action: · send-near            - The transfer is carried out in NEAR tokens
How many NEAR Tokens do you want to transfer? (example: 10NEAR or 0.5near or 10000yoctonear): 1NEAR
✔ Select an action that you want to add to the action: · add-action   - Select a new action
✔ Select an action that you want to add to the action: · call-function        - Execute function (contract method)
What is the name of the function?: create_transaction
Enter arguments to this function: {transaction_rlp_hex="TODO", use_paymaster=true}

Enter gas for function call: 100 TeraGas

Enter deposit for a function call (example: 10NEAR or 0.5near or 10000yoctonear).: 0.5 NEAR
✔ Select an action that you want to add to the action: · skip         - Skip adding a new action
✔ What is the name of the network? · testnet
✔  Select a tool for signing the transaction · sign-with-keychain               - Sign the transaction with a key saved in legacy keychain (compatible with the old near CLI)

Your transaction was signed successfully.
Signed transaction:

signer_id:    nomnomnom.testnet
public_key:   ed25519:CqATViMHe7cgkKdGhHXe499zSBLkM8wUVs9vBSjYE1n1
nonce:        137677046000020
receiver_id:  canhazgas.testnet
block_hash:   2TU3Hua4STKzQgrqomwg74LnuBK2VVqACg5NS8inFV8E
actions:
   -- transfer deposit:    1 NEAR
   -- function call:
                   method name:  create_transaction
                   args:         [123, 116, 114, 97, 110, 115, 97, 99, 116, 105, 111, 110, 95, 114, 108, 112, 95, 104, 101, 120, 61, 34, 34, 44, 32, 117, 115, 101, 95, 112, 97, 121, 109, 97, 115, 116, 101, 114, 61, 116, 114, 117, 101, 125]
                   gas:          100.000 TeraGas
                   deposit:      0.5 NEAR

✔ How would you like to proceed · display   - Print only base64 encoded transaction for JSON RPC input and exit

Serialize_to_base64:
EQAAAG5vbW5vbW5vbS50ZXN0bmV0AK/HILx7aUbcVWUI9LVl4fD6vaMK5nOJVBHhhhjW0aGqlKlrbzd9AAARAAAAY2FuaGF6Z2FzLnRlc3RuZXQVo1yT6YycpPs6QLyeLszfV3W3NXkxZAly7jkFKu41cwIAAAADAAAAoe3MzhvC0wAAAAAAAAISAAAAY3JlYXRlX3RyYW5zYWN0aW9uLAAAAHt0cmFuc2FjdGlvbl9ybHBfaGV4PSIiLCB1c2VfcGF5bWFzdGVyPXRydWV9AEB6EPNaAAAAAIDQdmbnDeFpAAAAAAAAAHNXJoOqRCl3j+6egSajbWv/qHtf7kkbALPH7f4PMn5rruill+Akpy4DmMLcCs/ObBaqn9FQDxRFdtLuTpgw0A0=
Your console command:
near-cli transaction construct-transaction nomnomnom.testnet canhazgas.testnet add-action send-near '1 NEAR' skip
```
2. Paste the base64 encoded transaction as the value of `b64_near_transaction_with_rlp_hex` var in `integration_test.py`

3. Run the integration test:
```shell
python3 integration_test.py
```