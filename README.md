# Data-gator, a Solana Block Scraper
I am using a free Helius node with a rate limit of 10 requests per second. This is a hard limitation that I cannot bypass without purchasing a plan (or running my own node ;=)

To start aggregating data, run (assuming you have Rust installed):
```
cargo run
```
![teaser](https://github.com/jonas089/solforge-interview-task/blob/master/resources/teaser.png)

Data-gator will immediately start fetching all Blocks for the current `epoch`, storing both `Blocks` and all the `Transactions` in those Blocks in seperate tables.
For now the tables are an in-memory DB (please forgive me).


