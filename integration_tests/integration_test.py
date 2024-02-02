import requests
import eth_utils
import rlp
from eth_utils import to_hex
from typing import List


def get_account_balance(address: str):
    url = "http://localhost:3030/get_balance_for_account"
    payload = {"address": address}
    response = requests.post(url, json=payload)
    if response.status_code == 200:
        return int(response.json()["balance"])
    else:
        raise Exception("Failed to fetch account balance.")


def send_transaction_to_near_rpc(transaction_rlp_hex: str, use_paymaster: bool, chain_id: str):
    near_rpc_url = "https://rpc.testnet.near.org"
    payload = {
        # TODO Update payload structure according to the actual NEAR RPC documentation
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

# Step 2: Prepare and encode RLP transaction
# TODO
transaction_rlp_hex = "TODO_rlp_encoded_transaction_here"

# Step 3: Use NEAR RPC to submit the transaction
near_rpc_result = send_transaction_to_near_rpc(
    transaction_rlp_hex=transaction_rlp_hex,
    use_paymaster=True,
    chain_id="97"
)

# Step 4: Send the raw_transactions to localhost
send_raw_transactions(near_rpc_result["raw_transactions"], near_rpc_result["chain_id"])

# Step 5: Get updated balance
updated_balance = get_account_balance("0x48cf9dA81d8c9FE834093eCE58ea7221aBc19DB2")

# Step 6: Compare balances and assert the condition
assert 0.01 < (initial_balance - updated_balance) / 10**18 < 0.015, "The balance difference does not match expected."

print("Transaction processed successfully. Balance difference is within the expected range.")
