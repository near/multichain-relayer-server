import asyncio
import base64
import json
import time

import requests


def fetch_latest_block() -> int:
    # Define the RPC endpoint for the NEAR network
    url = "https://rpc.testnet.near.org"

    # Define the payload for fetching the latest block
    payload = json.dumps({
        "jsonrpc": "2.0",
        "id": "dontcare",
        "method": "block",
        "params": {
            "finality": "final"
        }
    })

    # Define the headers for the HTTP request
    headers = {
        'Content-Type': 'application/json'
    }

    # Send the HTTP request to the NEAR RPC endpoint
    response = requests.request("POST", url, headers=headers, data=payload)

    # Parse the JSON response to get the latest final block height
    latest_final_block = int(response.json()["result"]["header"]["height"])

    # TODO change back to -10 after testing
    return latest_final_block - 10  # Subtract 10 blocks for margin of error


def get_latest_signed_transactions(latest_block: int):
    # Define the RPC endpoint for the NEAR network
    url = "https://rpc.testnet.near.org"

    # Define the payload for fetching the latest block
    payload = json.dumps({
        "jsonrpc": "2.0",
        "id": "dontcare",
        "method": "query",
        "params": {
            "request_type": "call_function",
            "finality": "final",
            "account_id": "canhazgas.testnet",
            "method_name": "get_signed_transactions_after",
            "args_base64": base64.b64encode(latest_block.to_bytes(32, 'big')).decode('utf-8')
        }
    })
    print(f"Payload get_latest_signed_transactions call: {payload}")

    # Define the headers for the HTTP request
    headers = {
        'Content-Type': 'application/json'
    }

    # Send the HTTP request to the NEAR RPC endpoint
    response = requests.request("POST", url, headers=headers, data=payload)

    # Parse the JSON response to get the signed transactions
    result = response.json()["result"]
    print(f"Response get_latest_signed_transactions call: {result}")
    # TODO: Parse the signed transactions

    return result


async def check_for_new_signed_txns_from_gas_station():
    latest_final_block = fetch_latest_block()
    print(f"Latest Final Block: {latest_final_block}")
    latest_signed_txns = get_latest_signed_transactions(latest_final_block)
    # TODO make sure the formatting is correct

    # call the multichain relayer with the signed txns
    payload = {
        "foreign_chain_id": "97",
        "raw_transactions": latest_signed_txns,
    }
    response = requests.post(
        url="localhost:3030/send_funding_and_user_signed_txns",
        json=payload,
    )
    if response.status_code not in {200, 201}:
        print(f"Error: calling localhost:3030/send_funding_and_user_signed_txns: {response.text}")
    else:
        print(f"Response from localhost:3030/send_funding_and_user_signed_txns: {response.text}")


async def main():
    while True:
        await check_for_new_signed_txns_from_gas_station()
        time.sleep(5)  # wait 5 seconds


if __name__ == "__main__":
    loop = asyncio.get_event_loop()
    loop.run_until_complete(main())
