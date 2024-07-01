import argparse
import copy
import json
import re
import requests
import subprocess
import time


def run_command(command, verbose=False):
    process = subprocess.Popen(command, stdout=subprocess.PIPE, stderr=subprocess.PIPE, shell=True)
    output, error = process.communicate()
    output_str = output.decode('utf-8')
    error_str = error.decode('utf-8')
    if verbose:
        print("Command output:")
        print(output_str)
        if error_str:
            print("Command error:")
            print(error_str)
    return output_str, error_str


def initialize_account(account_id, verbose=False):
    print(f"Initializing account {account_id}...")

    # Storage deposit
    cmd = f"""near contract call-function as-transaction v2.nft.kagi.testnet \
      storage_deposit json-args {{}} prepaid-gas '100.0 Tgas' attached-deposit '1 NEAR' \
      sign-as {account_id}.testnet network-config testnet sign-with-keychain send"""
    output, error = run_command(cmd, verbose)
    print("Storage deposit completed.")

    # Mint NFT
    cmd = f"""near contract call-function as-transaction v2.nft.kagi.testnet \
      mint json-args {{}} prepaid-gas '100.0 Tgas' attached-deposit '0 NEAR' \
      sign-as {account_id}.testnet network-config testnet sign-with-keychain send"""
    output, error = run_command(cmd, verbose)
    print("Mint NFT completed.")

    # Extract token_id from output
    token_id: str = copy.deepcopy(output)
    token_id = token_id.strip()
    print(f"Extracted token_id: {token_id}")

    # Approve Gas Station for this Token
    cmd = f"""near contract call-function as-transaction v2.nft.kagi.testnet \
      ckt_approve_call json-args '{{"token_id":"{token_id}","account_id":"canhazgas.testnet","msg":""}}' \
      prepaid-gas '100.0 Tgas' attached-deposit '1 yoctoNEAR' \
      sign-as {account_id}.testnet network-config testnet sign-with-keychain send"""
    output, error = run_command(cmd, verbose)
    print("Approve Gas Station completed.")

    return token_id


def get_paymaster_info(account_id, chain_id, verbose=False):
    print(f"Getting paymaster info for chain {chain_id}...")
    cmd = f"""near contract call-function as-transaction canhazgas.testnet \
     get_paymasters json-args '{{"chain_id": "{chain_id}"}}' \
     prepaid-gas '100.0 Tgas' attached-deposit '0 NEAR' \
     sign-as {account_id}.testnet network-config testnet sign-with-keychain send"""
    output, error = run_command(cmd, verbose)
    return json.loads(output)


def set_nonce(account_id, chain_id, nonce, verbose=False):
    print(f"Setting nonce for chain {chain_id} to {nonce}...")
    cmd = f"""near contract call-function as-transaction canhazgas.testnet \
     set_nonce json-args '{{"chain_id": "{chain_id}", "nonce": {nonce}}}' \
     prepaid-gas '100.000 Tgas' attached-deposit '0 NEAR' \
     sign-as {account_id}.testnet network-config testnet sign-with-keychain send"""
    output, error = run_command(cmd, verbose)
    print("Nonce set successfully.")


def create_transaction(account_id, rlp_hex, token_id, near_deposit, verbose=False):
    print("Creating transaction...")
    cmd = f"""near contract call-function as-transaction canhazgas.testnet \
      create_transaction json-args '{{"transaction_rlp_hex":"{rlp_hex}","use_paymaster":true,"token_id":"{token_id}"}}' \
      prepaid-gas '100.000 TeraGas' attached-deposit '{near_deposit} NEAR' \
      sign-as {account_id}.testnet network-config testnet sign-with-keychain send"""
    output, error = run_command(cmd, verbose)
    return json.loads(output)


def sign_next(account_id, transaction_id, verbose=False):
    print(f"Signing transaction {transaction_id}...")
    cmd = f"""near contract call-function as-transaction canhazgas.testnet \
      sign_next json-args '{{"id":"{transaction_id}"}}' \
      prepaid-gas '300.0 Tgas' attached-deposit '0 NEAR' \
      sign-as {account_id}.testnet network-config testnet sign-with-keychain send"""
    output, error = run_command(cmd, verbose)
    print("Transaction signed.")
    return output


def extract_event_data(output):
    event_json_match = re.search(r'EVENT_JSON:(.*)', output)
    if event_json_match:
        event_json_str = event_json_match.group(1)
        event_json = json.loads(event_json_str)
        return event_json.get('data', {})
    return None


def send_to_endpoint(data):
    url = "http://localhost:3030/send_funding_and_user_signed_txns"
    payload = {
        "foreign_chain_id": data["foreign_chain_id"],
        "signed_transactions": data["signed_transactions"]
    }
    response = requests.post(url, json=payload)
    print(f"Endpoint response status code: {response.status_code}")
    print(f"Endpoint response content: {response.text}")


def main():
    parser = argparse.ArgumentParser(description="Run integration tests for the multichain relayer system.")
    parser.add_argument("--verbose", action="store_true", help="Print subprocess output")
    args = parser.parse_args()

    account_id = input("Enter your account ID (without .testnet): ")
    chain_id = input("Enter the chain ID: ")
    rlp_hex = input("Enter the RLP hex for the EVM transaction (exclude 0x prefix): ")
    token_id = input("Enter the token ID (leave blank if you don't have one): ")
    skip_indexer: bool = input("Do you want to skip the indexer and send data directly to the endpoint? (y/n): ").lower().strip() == 'y'
    near_deposit: float = float(
        input("Enter the amount of NEAR to attach to the gas station transaction as payment for foreign chain gas: ")
    )

    if token_id == "":
        # Initialize account
        token_id = initialize_account(account_id, args.verbose)
    else:
        print(f"Using provided token_id: {token_id}")

    # Get paymaster info
    paymaster_info = get_paymaster_info(account_id, chain_id, args.verbose)

    # Set nonce if needed
    current_nonce = paymaster_info[0]['nonce']
    set_nonce_input = input(f"Current nonce is {current_nonce}. Do you want to set a new nonce? (y/n): ")
    if set_nonce_input.lower() == 'y':
        new_nonce = int(input("Enter the new nonce: "))
        set_nonce(account_id, chain_id, new_nonce, args.verbose)

    # Create transaction
    transaction = create_transaction(account_id, rlp_hex, token_id, near_deposit, args.verbose)
    transaction_id = transaction['id']

    # Sign transaction twice
    for i in range(2):
        output = sign_next(account_id, transaction_id, args.verbose)
        if i == 1 and skip_indexer:  # After the second sign_next call
            event_data = extract_event_data(output)
            if event_data:
                send_to_endpoint(event_data)
            else:
                print("Failed to extract event data from output.")
        time.sleep(5)  # Wait a bit between signings

    print("Integration test completed. Check the gas station event indexer and multichain relayer server outputs.")


if __name__ == "__main__":
    main()
