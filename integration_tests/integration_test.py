import subprocess
import json
import time
import re


def run_command(command):
    process = subprocess.Popen(command, stdout=subprocess.PIPE, stderr=subprocess.PIPE, shell=True)
    output, error = process.communicate()
    return output.decode('utf-8'), error.decode('utf-8')


def initialize_account(account_id):
    print(f"Initializing account {account_id}...")

    # Storage deposit
    cmd = f"""near contract call-function as-transaction v2.nft.kagi.testnet \
      storage_deposit json-args {{}} prepaid-gas '100.0 Tgas' attached-deposit '1 NEAR' \
      sign-as {account_id}.testnet network-config testnet sign-with-keychain send"""
    output, error = run_command(cmd)
    print("Storage deposit output:", output)

    # Mint NFT
    cmd = f"""near contract call-function as-transaction v2.nft.kagi.testnet \
      mint json-args {{}} prepaid-gas '100.0 Tgas' attached-deposit '0 NEAR' \
      sign-as {account_id}.testnet network-config testnet sign-with-keychain send"""
    output, error = run_command(cmd)
    print("Mint NFT output:", output)

    # Extract token_id from output
    token_id_match = re.search(r'"token_id": "(\d+)"', output)
    if token_id_match:
        token_id = token_id_match.group(1)
        print(f"Extracted token_id: {token_id}")
    else:
        raise Exception("Failed to extract token_id from mint output")

    # Approve Gas Station for this Token
    cmd = f"""near contract call-function as-transaction v2.nft.kagi.testnet \
      ckt_approve_call json-args '{{"token_id":"{token_id}","account_id":"canhazgas.testnet","msg":""}}' \
      prepaid-gas '100.0 Tgas' attached-deposit '0 NEAR' \
      sign-as {account_id}.testnet network-config testnet sign-with-keychain send"""
    output, error = run_command(cmd)
    print("Approve Gas Station output:", output)


def get_paymaster_info(account_id, chain_id):
    print(f"Getting paymaster info for chain {chain_id}...")
    cmd = f"""near contract call-function as-transaction canhazgas.testnet \
     get_paymasters json-args '{{"chain_id": "{chain_id}"}}' \
     prepaid-gas '100.0 Tgas' attached-deposit '0 NEAR' \
     sign-as {account_id}.testnet network-config testnet sign-with-keychain send"""
    output, error = run_command(cmd)
    print("Paymaster info:", output)
    return json.loads(output)


def set_nonce(account_id, chain_id, nonce):
    print(f"Setting nonce for chain {chain_id} to {nonce}...")
    cmd = f"""near contract call-function as-transaction canhazgas.testnet \
     set_nonce json-args '{{"chain_id": "{chain_id}", "nonce": {nonce}}}' \
     prepaid-gas '100.000 Tgas' attached-deposit '0 NEAR' \
     sign-as {account_id}.testnet network-config testnet sign-with-keychain send"""
    output, error = run_command(cmd)
    print("Set nonce output:", output)


def create_transaction(account_id, rlp_hex):
    print("Creating transaction...")
    cmd = f"""near contract call-function as-transaction canhazgas.testnet \
      create_transaction json-args '{{"transaction_rlp_hex":"{rlp_hex}","use_paymaster":true}}' \
      prepaid-gas '100.000 TeraGas' attached-deposit '0.5 NEAR' \
      sign-as {account_id}.testnet network-config testnet sign-with-keychain send"""
    output, error = run_command(cmd)
    print("Create transaction output:", output)
    return json.loads(output)


def sign_next(account_id, transaction_id):
    print(f"Signing transaction {transaction_id}...")
    cmd = f"""near contract call-function as-transaction canhazgas.testnet \
      sign_next json-args '{{"id":"{transaction_id}"}}' \
      prepaid-gas '300.0 Tgas' attached-deposit '0 NEAR' \
      sign-as {account_id}.testnet network-config testnet sign-with-keychain send"""
    output, error = run_command(cmd)
    print("Sign next output:", output)


def main():
    account_id = input("Enter your account ID (without .testnet): ")
    chain_id = input("Enter the chain ID: ")
    rlp_hex = input("Enter the RLP hex for the EVM transaction: ")

    # Initialize account
    initialize_account(account_id)

    # Get paymaster info
    paymaster_info = get_paymaster_info(account_id, chain_id)

    # Set nonce if needed
    current_nonce = paymaster_info[0]['nonce']
    set_nonce_input = input(f"Current nonce is {current_nonce}. Do you want to set a new nonce? (y/n): ")
    if set_nonce_input.lower() == 'y':
        new_nonce = int(input("Enter the new nonce: "))
        set_nonce(account_id, chain_id, new_nonce)

    # Create transaction
    transaction = create_transaction(account_id, rlp_hex)
    transaction_id = transaction['id']

    # Sign transaction twice
    for _ in range(2):
        sign_next(account_id, transaction_id)
        time.sleep(5)  # Wait a bit between signings

    print("Integration test completed. Check the gas station event indexer and multichain relayer server outputs.")


if __name__ == "__main__":
    main()
