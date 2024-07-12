# Data-gator, a Solana Block Scraper
I am using a free Helius node with a rate limit of 10 requests per second. This is a hard limitation that I cannot bypass without purchasing a plan (or running my own node ;=)

To start aggregating data, run (assuming you have Rust installed):
```
cargo run
```
![teaser](https://github.com/jonas089/solforge-interview-task/blob/master/resources/teaser.png)

Data-gator will immediately start fetching all Blocks for the current `epoch`, storing both `Blocks` and all the `Transactions` in those Blocks in seperate tables.
For now the tables are an in-memory DB (please forgive me).

Additionally, this will start an axum server that serves routes such as `127.0.0.1:8080/ping`.

## API routes

work in progress

## Data Types
See `types.rs`. When fetching the Blocks they are fit into this struct:

```rust
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

```
using `serde-json`. Blocks that don't fit into this struct are considered malformed or odd and therefore dropped.
This is what one can expect to encouter:
![malformed](https://github.com/jonas089/solforge-interview-task/blob/master/resources/malformed.png)
These errors indicate that a `Block` was encountered that doesn't fit into my `Block` struct.
It might be that these `Blocks` are actually malformed or that they adhere to a different format for reasons such as 
`Superblocks`. Either way for the scope of this project I will only support `Blocks` that are aligned with the latest Solana RPC documentation.

Similarily `Transactions` are fit into a `Transaction` struct:
```rust
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Transaction {
    pub meta: BlockMeta,
    pub transaction: TransactionData,
}
```
