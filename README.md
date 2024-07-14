# Data-gator, a Solana Block Scraper
I am using a free `Helius` Node with a rate limit of `10 requests per second`. This is a hard limitation that I cannot bypass without purchasing a plan (or running my own node)

To start aggregating data, run (assuming you have Rust installed):
```
cargo run
```
![teaser](https://github.com/jonas089/solforge-interview-task/blob/master/resources/teaser.png)

Data-gator will immediately start fetching all `Blocks` for the current `epoch`, storing both `Blocks` and all the `Transactions` in those `Blocks` in separate tables.
For now the tables are those of a memory DB (which implements insert and get similar to what one would see when migrating to SQL).

Simultaneously, this will start an axum server that serves routes such as `127.0.0.1:8080/ping`.

## API routes
`/blocks`: Query for all Blocks that are in the (memory) DB
![blocks](https://github.com/jonas089/solforge-interview-task/blob/master/resources/api-blocks.png)

`/transactions`: Query for all Transactions that are in the (memory) DB
![transactions](https://github.com/jonas089/solforge-interview-task/blob/master/resources/api-transactions.png)

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
`Superblocks`. Either way for the scope of this project I will only support `Blocks` that are aligned with the latest [Solana RPC documentation](https://solana.com/docs/rpc/http/getblock).

Similarily `Transactions` are fit into a `Transaction` struct:
```rust
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Transaction {
    pub meta: BlockMeta,
    pub transaction: TransactionData,
}
```

# Design decisions
When I began working on data-gator, I first studied the Solana RPC Api endpoints looking for how to fetch `Blocks` for a given `Epoch` and then extract `Transactions` from those `Blocks`.
That was my main focus and my goal was to get the minimum requirements in place as quick as possible. Moving forward I queried those endpoints and used the RPC documentation to come up
with Rust structs that fit the return values.

Due to the time constraints I decided to design a memory DB with an API that mocks that of an actual SQL library. To migrate to SQL one would have to setup tables and queries and then replace the
memory DB. This is definitely feasible but would have taken a substantial amount of time considering the scope of this exercise. Therefore I decided to go with a memory DB and focus on writing clean
and reusable code that spawns both the API service and the gator-loop efficiently, sharing state between them and locking whenever a write on the DB occurs.

# Changes on July 13 (post submission)
Noticed that I stored the Transaction data found in Blocks instead of `RawTransaction`s. The consequence is that the pre- and post- Balances are not returned by `127.0.0.1:8080/transactions`. Changed this to store raw Transactions and serve the relevant data. Due to additional queries and rate-limits this comes at a performance cost. 

# Another Repo that I would like to share (conveys my idea of design principles)
Anonymous Github GPG voting with ZK [jonas089/cypher-poll](https://github.com/jonas089/cypher-poll)

