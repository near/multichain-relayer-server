import asyncio
import base64
import json
import requests
import time

from typing import List


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
    return latest_final_block - 100000  # Subtract 10 blocks for margin of error


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
            "method_name": "list_signed_transaction_sequences_after",
            "args_base64": base64.b64encode(
                bytes(
                    str(
                        f'{{"block_height":"{latest_block}"}}'
                    ).encode('utf-8')
                )
            ).decode('utf-8')
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
    result: List[dict] = response.json()["result"]
    # expected result:
    #     [
    #         {
    #             "created_by_account_id": "b807806adcb73f6aecb5ed98bb8bd7bbe7bbf8ed342596ab700ef6b050abc4c3",
    #             "foreign_chain_id": "0",
    #             "signed_transactions": [
    #                 "02f86a80808204d28204d282520894c89663ac6d169bc3e2e0a99d9fe96f2e82bcc307840316d52080c080a055af9f924460637e7b5f2f2aacbac7ce04e19aa36f22c3de9e068eee1b5aaa01a069e6a3e6545edaecaad5a14427120442e9b1a876df968f74477792b1a3df1e7b",
    #                 "02f86a80821e618204d28204d2825208947b965bdb7f0464843572eb2b8c17bdf27b720b148204d280c080a0ad790ca092035f70f34c1124eaaa3bb082258601525bad9ff1f015d8146a0ea0a02e96d79f778eb7b4d2eefb24dfed9b01d6b109163591ba231a3db6c250986153"
    #             ]
    #         },
    # ...
    #         {
    #             "created_by_account_id": "b807806adcb73f6aecb5ed98bb8bd7bbe7bbf8ed342596ab700ef6b050abc4c3",
    #             "foreign_chain_id": "97",
    #             "signed_transactions": [
    #                 "0x02f873610185174876e80085174876e80082520894c89663ac6d169bc3e2e0a99d9fe96f2e82bcc307870eebe0b40e800080c001a0ff19fe769246de8483b986e5aeaa3360bfb74f238e2a91ea353dac9aad9e24a0a020485dcd2c64172b9bc058b7813646dafbf2f27d51aae388b074e514fdb6de05",
    #                 "0x02f873618085174876e80085174876e80082520894ef55a8bdf4498ea0af88bc54efb29608bb25e130872e2f6e5e14800080c001a0dac67c383e8de3211f3c5d360cc2e9a21d160711fc3f80113ac525169317e2eca07140a1d0d1528b6eaf9fac4bb1bd44c1c4f63bb956292b0211a0dad1748e2eea"
    #             ]
    #         }
    #     ]
    print(f"Response get_latest_signed_transactions call: {result}")
    if len(result) >= 0:
        # assuming the last element is the latest
        return result[-1]["signed_transactions"]
    else:
        return []


async def check_for_new_signed_txns_from_gas_station():
    latest_final_block = fetch_latest_block()
    print(f"Latest Final Block: {latest_final_block}")
    latest_signed_txns = get_latest_signed_transactions(latest_final_block)
    if len(latest_signed_txns) == 0:
        print("No new signed transactions found")
        return

    # call the multichain relayer with the signed txns
    payload = {
        "foreign_chain_id": "97",
        "raw_transactions": latest_signed_txns,
    }
    response = requests.post(
        url="0.0.0.0:3030/send_funding_and_user_signed_txns",
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
