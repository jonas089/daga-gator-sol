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
    #[serde(rename = "blockHeight")]
    pub block_height: Value,
    #[serde(rename = "blockTime")]
    pub block_time: Option<Value>,
    pub blockhash: String,
    #[serde(rename = "parentSlot")]
    pub parent_slot: Value,
    #[serde(rename = "previousBlockHash")]
    pub previous_block_hash: Option<String>,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn get_transaction_signatures(&self) -> Vec<String> {
        let mut signatures: Vec<String> = Vec::new();
        for transaction in self.transactions.clone() {
            signatures.push(transaction.transaction.signatures[0].clone());
        }
        signatures
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct RawTransaction {
    pub meta: TransactionMeta,
    pub slot: Value,
    pub transaction: TransactionData,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Transaction {
    pub meta: BlockMeta,
    pub transaction: TransactionData,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct BlockMeta {
    pub err: Option<Value>,
    pub fee: Value,
    #[serde(rename = "innerInstructions")]
    pub inner_instructions: Vec<Value>,
    #[serde(rename = "logMessages")]
    pub log_messages: Vec<Value>,
    #[serde(rename = "postBalances")]
    pub post_balances: Vec<Value>,
    #[serde(rename = "postTokenBalances")]
    pub post_token_balances: Vec<Value>,
    #[serde(rename = "preBalances")]
    pub pre_balances: Vec<Value>,
    #[serde(rename = "preTokenBalances")]
    pub pre_token_balances: Vec<Value>,
    pub rewards: Option<Vec<Value>>,
    pub status: Status,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct TransactionMeta {
    pub err: Option<Value>,
    pub fee: Value,
    #[serde(rename = "innerInstructions")]
    pub inner_instructions: Vec<Value>,
    #[serde(rename = "postBalances")]
    pub post_balances: Vec<Value>,
    #[serde(rename = "postTokenBalances")]
    pub post_token_balances: Vec<Value>,
    #[serde(rename = "preBalances")]
    pub pre_balances: Vec<Value>,
    #[serde(rename = "preTokenBalances")]
    pub pre_token_balances: Vec<Value>,
    pub rewards: Option<Vec<Value>>,
    pub status: Status,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Status {
    #[serde(rename = "Ok")]
    pub ok: Option<Value>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct TransactionData {
    pub message: Message,
    pub signatures: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Message {
    #[serde(rename = "accountKeys")]
    pub account_keys: Vec<String>,
    pub header: Header,
    pub instructions: Option<Vec<Instruction>>,
    #[serde(rename = "recentBlockhash")]
    pub recent_blockhash: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Header {
    #[serde(rename = "numReadonlySignedAccounts")]
    pub num_readonly_signed_accounts: Value,
    #[serde(rename = "numReadonlyUnsignedAccounts")]
    pub num_readonly_unsigned_accounts: Value,
    #[serde(rename = "numRequiredSignatures")]
    pub num_required_signatures: Value,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Instruction {
    pub accounts: Vec<Value>,
    pub data: String,
    #[serde(rename = "programIdIndex")]
    pub program_id_index: Value,
}
