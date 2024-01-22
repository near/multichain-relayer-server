# multichain-relayer-server
Pagoda Implementation of Multichain Relayer Server facilitating cross chain transactions 

## Description
This server interacts with RPCs on other chains sending both presigned funding transactions to cover gas and the actual presigned transaction once the funding is done. The goal is to package this as a library that can be called on the client side of the wallet. This server relies heavily on:
 - EVM RPCs will be interacted with using the [alloy](https://github.com/alloy-rs/) crate. Initially we will connect to public RPCs
 - TODO Solana crate and RPC providers.

## Functionality
1. Funding the user's xchain account with gas from the paymaster treasury account, which is provided as a raw signed transaction
2. Sending the user's raw signed transaction (in hexadecimal in EVM case)


## Technical Design
The Technical Design is detailed in https://docs.google.com/document/d/1ZvfiVDXSykYsdH8v816G-V5OizAUtpw9WAh1ihwz908/edit

Below is a Design Diagram of the entire multichain relayer system. 
This server repo focuses on the purple/blue Multichain Relayer Core Backend Services Box in the middle and the connections to the XChain systems in the red box via RPCs.  
![img.png](img.png)
