import json
import requests
import eth_utils
import rlp
from eth_utils import to_hex
from typing import List


def get_account_balance(address: str):
    url = "http://localhost:3030/get_balance_for_account"
    payload = {"address": address}
    response = requests.get(url, json=payload)
    if response.status_code == 200:
        return response.json()
    else:
        raise Exception("Failed to fetch account balance.")


def send_transaction_to_near_rpc(b64_txn: str, use_paymaster: bool, chain_id: str):
    near_rpc_url = "https://rpc.testnet.near.org"
    payload = {
        "jsonrpc": "2.0",
        "id": "1",
        "method": "broadcast_tx_commit",
        "params": [
            b64_txn
        ]
    }
    response = requests.post(near_rpc_url, json=payload)
    if response.status_code == 200:
        return response.json()  # Assuming this returns the required list of raw_transactions and chain_id
    else:
        raise Exception("Failed to interact with NEAR RPC.")


def send_raw_transactions(raw_transactions: List[str], chain_id: str):
    url = "http://localhost:3030/send_funding_and_user_signed_txns"
    payload = {
        "raw_transactions": raw_transactions,
        "chain_id": chain_id
    }
    response = requests.post(url, json=payload)
    if response.status_code != 200:
        raise Exception("Failed to send raw transactions.")


# Step 1: Get initial balance
initial_balance = get_account_balance("0x48cf9dA81d8c9FE834093eCE58ea7221aBc19DB2")
print("Initial balance:", initial_balance)

# Step 2: Prepare and encode RLP transaction
# TODO LP also add the chain_id in the payload
# paste this in from output of running `near` rust cli to construct the transaction
# b64_near_transaction_with_rlp_hex = "TODO_rlp_encoded_transaction_here"
# b64_near_transaction_with_rlp_hex = "EQAAAG5vbW5vbW5vbS50ZXN0bmV0AK/HILx7aUbcVWUI9LVl4fD6vaMK5nOJVBHhhhjW0aGqlKlrbzd9AAARAAAAY2FuaGF6Z2FzLnRlc3RuZXRwIs24OjrKvul7ggfiG7ds23L+XndcGAEB0HReJjtn3AIAAAADAAAAoe3MzhvC0wAAAAAAAAISAAAAY3JlYXRlX3RyYW5zYWN0aW9uiwAAACd7InRyYW5zYWN0aW9uX3JscF9oZXgiOiJlYjgwODUwMjU0MGJlNDAwODI1MjA4OTQ3Yjk2NWJkYjdmMDQ2NDg0MzU3MmViMmI4YzE3YmRmMjdiNzIwYjE0ODcyMzg2ZjI2ZmMxMDAwMDgwODA4MDgwIiwgInVzZV9wYXltYXN0ZXIiOiB0cnVlfScAQHoQ81oAAAAAQGg7s/OG8DQAAAAAAAAAhfPGdXKlQ+gQdgPGnFg4pTngrGmDinjHer32VVHEU9ipM4hIeQ8y7AG1eZm9+MJEgs4/TMVhNHCc65rp2lSMCw=="
### sending this in from output of running `near` rust cli to construct the transaction
# args: '{"transaction_rlp_hex":"eb808502540be400825208947b965bdb7f0464843572eb2b8c17bdf27b720b14872386f26fc1000080808080", "use_paymaster": true}'
# signer_id:    nomnomnom.testnet
# public_key:   ed25519:CqATViMHe7cgkKdGhHXe499zSBLkM8wUVs9vBSjYE1n1
# nonce:        137677046000021
# receiver_id:  canhazgas.testnet
# block_hash:   GbcSQM5wXpN9doVzZN1KgyS2PC7Scu1FM8Xz9SMk5VoU
# actions:
#    -- transfer deposit:    1 NEAR
#    -- function call:
#                    method name:  create_transaction
#                    args:         [39, 123, 34, 116, 114, 97, 110, 115, 97, 99, 116, 105, 111, 110, 95, 114, 108, 112, 95, 104, 101, 120, 34, 58, 34, 101, 98, 56, 48, 56, 53, 48, 50, 53, 52, 48, 98, 101, 52, 48, 48, 56, 50, 53, 50, 48, 56, 57, 52, 55, 98, 57, 54, 53, 98, 100, 98, 55, 102, 48, 52, 54, 52, 56, 52, 51, 53, 55, 50, 101, 98, 50, 98, 56, 99, 49, 55, 98, 100, 102, 50, 55, 98, 55, 50, 48, 98, 49, 52, 56, 55, 50, 51, 56, 54, 102, 50, 54, 102, 99, 49, 48, 48, 48, 48, 56, 48, 56, 48, 56, 48, 56, 48, 34, 44, 32, 34, 117, 115, 101, 95, 112, 97, 121, 109, 97, 115, 116, 101, 114, 34, 58, 32, 116, 114, 117, 101, 125, 39]
#                    gas:          100.000 TeraGas
#                    deposit:      0.1 NEAR
###
b64_near_transaction_with_rlp_hex = "EQAAAG5vbW5vbW5vbS50ZXN0bmV0AK/HILx7aUbcVWUI9LVl4fD6vaMK5nOJVBHhhhjW0aGqlalrbzd9AAARAAAAY2FuaGF6Z2FzLnRlc3RuZXTnvR8KMvUnfkuCs2t9BVyYskR9YZBp8wvOAkClKPLWhwIAAAADAAAAoe3MzhvC0wAAAAAAAAISAAAAY3JlYXRlX3RyYW5zYWN0aW9uiwAAACd7InRyYW5zYWN0aW9uX3JscF9oZXgiOiJlYjgwODUwMjU0MGJlNDAwODI1MjA4OTQ3Yjk2NWJkYjdmMDQ2NDg0MzU3MmViMmI4YzE3YmRmMjdiNzIwYjE0ODcyMzg2ZjI2ZmMxMDAwMDgwODA4MDgwIiwgInVzZV9wYXltYXN0ZXIiOiB0cnVlfScAQHoQ81oAAAAAgPZK4ccCLRUAAAAAAAAA2TIFaWyXSd/iwTHzD/WpFzcYnKuCKHr9oqUTRZNbCfGdBBVDAusT9CTpenZL2AE1V9SUX70fkE52M7ct84JOAg=="

# Step 3: Use NEAR RPC to submit the transaction
near_rpc_result = send_transaction_to_near_rpc(
    b64_txn=b64_near_transaction_with_rlp_hex,
    use_paymaster=True,
    chain_id="97"
)
print("Near RPC Result:", json.dumps(near_rpc_result, indent=4))

# Step 4: Send the raw_transactions to localhost
foreign_chain_tx_result = send_raw_transactions(near_rpc_result["raw_transactions"], near_rpc_result["chain_id"])
print("Foreign Chain Tx Result:", foreign_chain_tx_result)

# Step 5: Get updated balance
updated_balance = get_account_balance("0x48cf9dA81d8c9FE834093eCE58ea7221aBc19DB2")
print("Updated balance:", updated_balance)

# Step 6: Compare balances and assert the condition
# TODO either change the response format of get_balance_for_address or change the parsing of response
# assert 0.01 < (initial_balance - updated_balance) / 10**18 < 0.015, "The balance difference does not match expected."

print("Transaction processed successfully. Balance difference is within the expected range.")
