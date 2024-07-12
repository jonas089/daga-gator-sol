pub mod client;
pub mod storage;
pub mod types;
use colored::*;
use std::collections::HashMap;
use storage::{MemoryDB, SharedMemoryDB};
use tokio::sync::RwLock;

fn main() {
    println!(
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
    // todo: start aggregating data and store it in the db
    // todo: launch the api as a child process
}
