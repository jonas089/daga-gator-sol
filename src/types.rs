use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Block {}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Transaction {}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct SolanaEpoch{
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