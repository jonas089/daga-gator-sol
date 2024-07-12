use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct SolanaEpoch {
    #[serde(rename = "absoluteSlot")]
    pub absolute_slot: u64,
    #[serde(rename = "blockHeight")]
    pub block_height: u64,
    pub epoch: u64,
    #[serde(rename = "slotIndex")]
    pub slot_index: u64,
    #[serde(rename = "slotsInEpoch")]
    pub slots_in_epoch: u64,
    #[serde(rename = "transactionCount")]
    pub transaction_count: u64,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Block {

}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Transaction {

}