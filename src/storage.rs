use crate::types::{Block, Transaction};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
pub struct MemoryDB {
    pub blocks: HashMap<u64, Block>,
    pub transactions: HashMap<String, (Transaction, u64)>,
    pub block_idx: u64,
}
pub type SharedMemoryDB = Arc<RwLock<MemoryDB>>;

impl MemoryDB {
    pub fn insert_block(&mut self, height: u64, block: Block) {
        self.blocks.insert(height, block);
        self.block_idx = height
    }
    pub fn insert_transaction(&mut self, tx_hash: String, tx: Transaction, height: u64) {
        self.transactions.insert(tx_hash, (tx, height));
    }
    pub fn get_block_by_height(&self, height: u64) -> &Block {
        self.blocks.get(&height).expect("Failed to get Block")
    }
    pub fn get_transaction_by_hash(&self, tx_hash: &str) -> &(Transaction, u64) {
        self.transactions
            .get(tx_hash)
            .expect("Failed to get Transaction")
    }
    pub fn get_last_block(&self) -> &Block {
        self.blocks
            .get(&self.block_idx)
            .expect("Failed to get Block")
    }
}

pub async fn async_insert_block(db: &mut SharedMemoryDB, height: u64, block: Block) {
    let mut db = db.write().await;
    db.insert_block(height, block);
}

pub async fn async_insert_transaction(
    db: &mut SharedMemoryDB,
    tx_hash: String,
    tx: Transaction,
    height: u64,
) {
    let mut db = db.write().await;
    db.insert_transaction(tx_hash, tx, height);
}

pub async fn async_get_block_by_height(db: &SharedMemoryDB, height: u64) -> Block {
    let db = db.read().await;
    db.get_block_by_height(height).clone()
}

pub async fn async_get_transaction_by_hash(
    db: &SharedMemoryDB,
    tx_hash: String,
    height: u64,
) -> (Transaction, u64) {
    let db = db.read().await;
    db.get_transaction_by_hash(&tx_hash).clone()
}

pub async fn async_get_last_block(db: &SharedMemoryDB) -> Block {
    let db = db.read().await;
    db.get_last_block().clone()
}
