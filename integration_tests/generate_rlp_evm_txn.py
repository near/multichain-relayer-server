from eth_account._utils.legacy_transactions import encode_transaction, serializable_unsigned_transaction_from_dict
from web3 import Web3


def generate_rlp_encoded_transaction():
    # BSC Testnet parameters
    # NOTE: you should update these for your tests
    chain_id = 97  # BSC testnet chain ID
    nonce = 0  # Increment accordingly
    gas_price = Web3.to_wei(10, 'gwei')  # Adjust based on current conditions
    gas_limit = 21000  # Standard gas limit for a simple transaction
    to = '0x7b965bDB7F0464843572Eb2B8c17BdF27B720b14'  # Recipient address
    value = Web3.to_wei(0.01, 'ether')  # Amount to send
    data = ''  # No data for a simple BNB transfer

    # Create a dictionary of the transaction components
    tx_dict = {
        'nonce': nonce,
        'gasPrice': gas_price,
        'gas': gas_limit,
        'to': to,
        'value': value,
        'data': data,
        'chainId': chain_id
    }

    # Create a serializable unsigned transaction
    unsigned_tx = serializable_unsigned_transaction_from_dict(tx_dict)

    # RLP-encode the transaction
    rlp_encoded_tx = encode_transaction(unsigned_tx, vrs=(0, 0, 0))

    # Output the RLP-encoded transaction
    print(f'RLP-Encoded Transaction: {rlp_encoded_tx.hex()}')


if __name__ == '__main__':
    generate_rlp_encoded_transaction()
