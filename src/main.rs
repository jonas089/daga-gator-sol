pub mod client;
pub mod storage;
pub mod types;
use client::{JsonRpcClient, RPC_ENDPOINT};
use colored::*;
use indicatif::ProgressBar;
use std::collections::HashMap;
use storage::{async_insert_block, async_insert_transaction};
use storage::{MemoryDB, SharedMemoryDB};
use tokio::sync::RwLock;
use types::{Block, SolanaEpoch};

#[tokio::main]
async fn main() {
    println!(
        "{}",
        r#"
           .-._   _ _ _ _ _ _ _ _
.-''-.__.-'00  '-' ' ' ' ' ' ' ' '-.
'.___ '    .   .--_'-' '-' '-' _'-' '._
 V: V 'vv-'   '_   '.       .'  _..' '.'.
   '=.____.=_.--'   :_.__.__:_   '.   : :
           (((____.-'        '-.  /   : :
                             (((-'\ .' /
                           _____..'  .'
                          '-._____.-'"#
            .green()
            .bold()
    );
    println!(
        "{}{} {}!",
        "Data".green(),
        "Gator".yellow(),
        "says Hello".blue().italic()
    );
    let mut shared_memory_db: SharedMemoryDB = SharedMemoryDB::new(RwLock::new(MemoryDB {
        blocks: HashMap::new(),
        transactions: HashMap::new(),
        block_idx: 0,
    }));
    let client: JsonRpcClient =
        JsonRpcClient::new(RPC_ENDPOINT).expect("[Error] Failed to init RPC Client");
    let current_epoch: SolanaEpoch = client
        .get_current_epoch()
        .await
        .expect("[Error] Failed to get current Epoch");
    let epoch_blocks: Vec<u32> = client
        .get_current_era_blocks(current_epoch)
        .await
        .expect("[Error] Failed to get Blocks for ongoing Epoch");
    let progress_bar: ProgressBar = ProgressBar::new(epoch_blocks.len() as u64);
    println!("Epoch Blocks: {:?}", &epoch_blocks.len());
    for block_slot in epoch_blocks {
        // No fault tolerance for now
        let block: Option<Block> = match client.get_block_by_id(block_slot).await {
            Ok(block) => Some(block),
            Err(err) => {
                println!("Malformed or non-standard Block {}", &err.to_string().red());
                None
            }
        };
        match block {
            Some(block) => {
                let block_height = block.block_height.as_u64().unwrap();
                for transaction in block.transactions.clone() {
                    async_insert_transaction(
                        &mut shared_memory_db,
                        transaction.transaction.signatures[0].clone(),
                        transaction,
                        block_height,
                    )
                    .await;
                }
                async_insert_block(&mut shared_memory_db, block_height, block).await;
            }
            None => {}
        }
        progress_bar.inc(1);
        //sleep(Duration::from_millis(10));
    }
    progress_bar.finish_with_message("Done fetching Blocks for Epoch!");
    /*
        Since I am implementing this with a MemoryDB, there is no fallback options for cases where
        the service crashes. I wrote the code with SQL databases in mind though, therefore it is quite
        easy to setup a schema and replace MemoryDB. This is a common practice in Prototyping and POC architecture that
        I am personally a big fan of.

        If I had 1 week + to work on this project (and were paid for it), then I would consider setting up a Database schema and move away from
        in-memory storage. I think that for the scope of this project this is a fair assumption.
    */
}
