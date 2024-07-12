use crate::types::{Block, Transaction};
use std::collections::HashMap;
pub struct MemoryDB {
    pub blocks: HashMap<u64, Block>,
    pub transactions: HashMap<String, (Transaction, u64)>,
    pub block_idx: u64,
}

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
