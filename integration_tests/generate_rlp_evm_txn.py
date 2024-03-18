from eth_account._utils.legacy_transactions import encode_transaction, serializable_unsigned_transaction_from_dict
import rlp
from rlp.sedes import List, BigEndianInt, Binary, binary
from web3 import Web3


def generate_rlp_encoded_transaction(is_eip_1559: bool = False):
    # BSC Testnet parameters
    # This example assumes a network that supports EIP-1559.
    chain_id = 97  # Update accordingly
    nonce = 0  # Increment accordingly
    to = '0x7b965bDB7F0464843572Eb2B8c17BdF27B720b14'  # Recipient address
    value = Web3.to_wei(0.01, 'ether')  # Amount to send

    if is_eip_1559:
        # EIP-1559 transaction parameters
        max_priority_fee_per_gas = Web3.to_wei(2, 'gwei')  # Tip for the miner
        max_fee_per_gas = Web3.to_wei(100, 'gwei')  # Max fee per gas
        gas_limit = 21000  # Standard gas limit for a simple transaction
        data = b''  # No data for a simple transfer

        tx_fields = {
            'chainId': chain_id,
            'nonce': nonce,
            'maxPriorityFeePerGas': max_priority_fee_per_gas,
            'maxFeePerGas': max_fee_per_gas,
            'gas': gas_limit,
            'to': to,
            'value': value,
            'data': data,
            'type': '0x2',  # Indicates an EIP-1559 transaction
            'accessList': [],  # Optional access list
        }

        # Init the sedes for the transaction
        tx_sedes = List([
            BigEndianInt(),  # chainId
            BigEndianInt(),  # nonce
            BigEndianInt(),  # maxPriorityFeePerGas
            BigEndianInt(),  # maxFeePerGas
            BigEndianInt(),  # gas
            Binary.fixed_length(20),  # to address, fixed length of 20 bytes
            BigEndianInt(),  # value
            Binary(),  # data, variable length
            List([]),  # accessList
        ])

        # Assuming the rest of your transaction data is correctly defined in tx_list
        tx_list = [
            tx_fields['chainId'],
            tx_fields['nonce'],
            tx_fields['maxPriorityFeePerGas'],
            tx_fields['maxFeePerGas'],
            tx_fields['gas'],
            Web3.to_bytes(hexstr=tx_fields['to']),
            tx_fields['value'],
            tx_fields['data'],
            tx_fields['accessList'],
        ]

        # Now encode with the corrected sedes
        encoded_tx = rlp.encode(tx_list, sedes=tx_sedes)
        print(encoded_tx)

        # Convert to hex
        encoded_tx_hex = '0x' + encoded_tx.hex()

        # Output the hex RLP-encoded transaction
        print(f'RLP-Encoded Transaction: {encoded_tx_hex}')

    else:
        # Legacy transaction parameters
        gas_price = Web3.to_wei(120, 'gwei')
        gas_limit = 21000
        data = ''  # No data for a simple transfer

        tx_dict = {
            'nonce': nonce,
            'gasPrice': gas_price,
            'gas': gas_limit,
            'to': to,
            'value': value,
            'data': data,
            'chainId': chain_id,
        }

        # Create a serializable unsigned transaction
        unsigned_tx = serializable_unsigned_transaction_from_dict(tx_dict)

        # RLP-encode the transaction (EIP-1559 transactions require passing the correct transaction type to the encoder)
        rlp_encoded_tx = encode_transaction(unsigned_tx, vrs=(0, 0, 0))

        # Output the hex RLP-encoded transaction
        print(f'RLP-Encoded Transaction: {rlp_encoded_tx.hex()}')


if __name__ == '__main__':
    generate_rlp_encoded_transaction(is_eip_1559=True)
