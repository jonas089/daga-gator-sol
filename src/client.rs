use crate::types::{Block, SolanaEpoch};
use anyhow::Result;
use jsonrpsee_core::client::ClientT;
use jsonrpsee_http_client::{HeaderMap, HeaderValue, HttpClient, HttpClientBuilder};
use serde_json::{json, Value};

pub const RPC_ENDPOINT: &str = "https://api.devnet.solana.com";

#[derive(Debug)]
pub struct JsonRpcClient {
    client: HttpClient,
}

impl JsonRpcClient {
    pub fn new(rpc_endpoint: &str) -> Result<JsonRpcClient> {
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));

        let client = HttpClientBuilder::default()
            .set_headers(headers)
            .build(rpc_endpoint)?;

        Ok(JsonRpcClient { client })
    }

    pub async fn post(&self, method: &str, params: Vec<Value>) -> Result<Value> {
        Ok(self.client.request(method, params).await?)
    }

    pub async fn get_current_epoch(&self) -> Result<SolanaEpoch> {
        let response = self.post("getEpochInfo", vec![]).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn get_current_era_blocks(&self, epoch: SolanaEpoch) -> Result<Vec<u32>> {
        let response = self
            .post(
                "getBlocks",
                vec![
                    Value::from(epoch.absolute_slot - epoch.slot_index),
                    Value::from(epoch.absolute_slot - epoch.slot_index + epoch.slots_in_epoch),
                ],
            )
            .await?;
        Ok(serde_json::from_value(response).unwrap())
    }
}

#[tokio::test]
async fn test_get_epoch() {
    let client: JsonRpcClient =
        JsonRpcClient::new(RPC_ENDPOINT).expect("Failed to construct Client");
    println!("Epoch: {:?}", client.get_current_epoch().await);
}

#[tokio::test]
async fn test_get_blocks() {
    let client: JsonRpcClient =
        JsonRpcClient::new(RPC_ENDPOINT).expect("Failed to construct Client");
    let epoch: SolanaEpoch = client.get_current_epoch().await.unwrap();
    let blocks = client.get_current_era_blocks(epoch).await;
    println!("Blocks: {:?}", &blocks);
}

#[tokio::test]
async fn test_get_block_metadata() {
    let client: JsonRpcClient =
        JsonRpcClient::new(RPC_ENDPOINT).expect("Failed to construct Client");
    let epoch: SolanaEpoch = client.get_current_epoch().await.unwrap();
    let finalized_block_id: u32 = client
        .get_current_era_blocks(epoch)
        .await
        .expect("Failed to get Blocks for current Epoch")[0];
    let response = client
        .post(
            "getBlock",
            vec![
                Value::from(finalized_block_id),
                Value::from(json!(      {
                  "encoding": "json",
                  "maxSupportedTransactionVersion":0,
                  "transactionDetails":"full",
                  "rewards":false
                })),
            ],
        )
        .await;
    let block: Value = serde_json::from_value(response.unwrap()).unwrap();
    // exclude 0 index since that is the metadata of the transactions blob
    let transaction_signature: String = serde_json::from_value(block["transactions"].get(1).unwrap()["transaction"]["signatures"][0].clone()).unwrap();
    println!("Transaction Signature: {:?}", &transaction_signature);
    // 52C9SsvGcMy6j6MMFtD2YNFkHn99HpqgnVPVNZQDTDWv3p9gAJU1gkfQwNycPEDy6KXP7k6UveLnqivTBo3Tbnqg
}

#[tokio::test]
async fn test_get_transaction_from_signature(){
    // todo
}