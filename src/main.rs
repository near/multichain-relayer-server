// std dependencies
use std::str::FromStr;
use std::collections::{HashMap, HashSet};

// external dependencies
use clap::Parser;
use tokio::sync::mpsc;
use tracing::info;

// near lake dependencies
use near_lake_framework::near_indexer_primitives;
use near_lake_framework::LakeConfig;

// local dependencies
use configs::{init_logging, Opts};
mod configs;
mod indexer;

// TODO change to testnet relayer contract
// TODO listen for logs in format:
/*
            xchain_id: self.xchain_id.clone(),
            sender: transaction.from().map(|a| to_checksum(a, None)),
            payload_id: Some(hex::encode(txid.0)),
            payload: rlp_signed_hex,
            request_tokens_for_gas,
 */
// TODO in next branch use alloy-rs to interact with the xchain
// TODO after that, update txn status in redis


/// Assuming we want to watch for transactions where a receiver account id is one of the provided in a list
/// We pass the list of account ids (or contracts it is the same) via argument ``--accounts``
/// We want to catch all *successfull* transactions sent to one of the accounts from the list.
/// In the demo we'll just look for them and log them but it might and probably should be extended based on your needs.

#[tokio::main]
async fn main() -> Result<(), tokio::io::Error> {
    init_logging();

    let opts: Opts = Opts::parse();

    let config: LakeConfig = opts.clone().into();

    let (_, stream) = near_lake_framework::streamer(config);

    let watching_list = opts
        .accounts
        .split(',')
        .map(|elem| {
            near_indexer_primitives::types::AccountId::from_str(elem).expect("AccountId is invalid")
        })
        .collect();

    indexer::listen_blocks(stream, watching_list).await;

    Ok(())
}
