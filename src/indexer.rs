// std dependencies
use std::collections::{HashMap, HashSet};

// external dependencies
use tokio::sync::mpsc;
use tracing::info;

// near lake dependencies
use near_lake_framework::near_indexer_primitives;


/// The main listener function the will be reading the stream of blocks `StreamerMessage`
/// and perform necessary checks
pub async fn listen_blocks(
    mut stream: mpsc::Receiver<near_indexer_primitives::StreamerMessage>,
    watching_list: Vec<near_indexer_primitives::types::AccountId>,
) {
    eprintln!("listen_blocks");
    // This will be a map of correspondence between transactions and receipts
    let mut tx_receipt_ids = HashMap::<String, String>::new();
    // This will be a list of receipt ids we're following
    let mut wanted_receipt_ids = HashSet::<String>::new();

    // Boilerplate code to listen the stream
    while let Some(streamer_message) = stream.recv().await {
        // eprintln!("Block height: {}", streamer_message.block.header.height);
        for shard in streamer_message.shards {
            let chunk = if let Some(chunk) = shard.chunk {
                chunk
            } else {
                continue;
            };

            for transaction in chunk.transactions {
                // Check if transaction sender id is one of the list we are interested in
                if is_tx_sender_watched(&transaction, &watching_list) {
                    // extract receipt_id transaction was converted into
                    let converted_into_receipt_id = transaction
                        .outcome
                        .execution_outcome
                        .outcome
                        .receipt_ids
                        .first()
                        .expect("`receipt_ids` must contain one Receipt Id")
                        .to_string();
                    // add `converted_into_receipt_id` to the list of receipt ids we are interested in
                    wanted_receipt_ids.insert(converted_into_receipt_id.clone());
                    // add key value pair of transaction hash and in which receipt id it was converted for further lookup
                    tx_receipt_ids.insert(
                        converted_into_receipt_id,
                        transaction.transaction.hash.to_string(),
                    );
                }
            }

            for execution_outcome in shard.receipt_execution_outcomes {
                if let Some(receipt_id) =
                    wanted_receipt_ids.take(&execution_outcome.receipt.receipt_id.to_string())
                {
                    // log the tx because we've found it
                    info!(
                        target: "indexer_example",
                        "Transaction hash {:?} related to {} executed with status {:?}",
                        tx_receipt_ids.get(receipt_id.as_str()),
                        &execution_outcome.receipt.predecessor_id,
                        execution_outcome.execution_outcome.outcome.status
                    );
                    if let near_indexer_primitives::views::ReceiptEnumView::Action {
                        signer_id,
                        ..
                    } = &execution_outcome.receipt.receipt
                    {
                        eprintln!("{}", signer_id);
                    }

                    // remove tx from hashmap
                    tx_receipt_ids.remove(receipt_id.as_str());
                }
            }
        }
    }
}

fn is_tx_sender_watched(
    tx: &near_indexer_primitives::IndexerTransactionWithOutcome,
    watching_list: &[near_indexer_primitives::types::AccountId],
) -> bool {
    watching_list.contains(&tx.transaction.signer_id)
}
