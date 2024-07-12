pub mod client;
pub mod storage;
pub mod types;
use client::{JsonRpcClient, RPC_ENDPOINT};
use colored::*;
use std::collections::HashMap;
use storage::{MemoryDB, SharedMemoryDB};
use tokio::sync::RwLock;
use types::SolanaEpoch;

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
        "says Hello".blue()
    );
    let memory_db: SharedMemoryDB = SharedMemoryDB::new(RwLock::new(MemoryDB {
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
    for block in epoch_blocks {}

    // todo: start aggregating data and store it in the db
    // todo: launch the api as a child process
}
