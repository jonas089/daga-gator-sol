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
pub struct Block {}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Transaction {
    pub meta: Meta,
    pub slot: u64,
    pub transaction: TransactionData,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Meta {
    pub err: Option<Value>,
    pub fee: u64,
    #[serde(rename = "innerInstructions")]
    pub inner_instructions: Vec<Value>,
    #[serde(rename = "postBalances")]
    pub post_balances: Vec<u64>,
    #[serde(rename = "postTokenBalances")]
    pub post_token_balances: Vec<u64>,
    #[serde(rename = "preBalances")]
    pub pre_balances: Vec<u64>,
    #[serde(rename = "preTokenBalances")]
    pub pre_token_balances: Vec<u64>,
    pub rewards: Vec<Value>,
    pub status: Status,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Status {
    #[serde(rename = "Ok")]
    pub ok: Option<Value>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct TransactionData {
    message: Message,
    signatures: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Message {
    #[serde(rename = "accountKeys")]
    pub account_keys: Vec<String>,
    pub header: Header,
    pub instructions: Vec<Instruction>,
    #[serde(rename = "recentBlockhash")]
    pub recent_blockhash: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Header {
    #[serde(rename = "numReadonlySignedAccounts")]
    pub num_readonly_signed_accounts: u64,
    #[serde(rename = "numReadonlyUnsignedAccounts")]
    pub num_readonly_unsigned_accounts: u64,
    #[serde(rename = "numRequiredSignatures")]
    pub num_required_signatures: u64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Instruction {
    pub accounts: Vec<u64>,
    pub data: String,
    #[serde(rename = "programIdIndex")]
    pub program_id_index: u64,
}
