use serde_json::Value;
use jsonrpsee_http_client::{HttpClientBuilder, HttpClient, HeaderValue, HeaderMap};
use jsonrpsee_core::client::ClientT;
use crate::types::SolanaEpoch;
use anyhow::Result;

pub const RPC_ENDPOINT: &str = "https://api.devnet.solana.com";

#[derive(Debug)]
pub struct JsonRpcClient{
    client: HttpClient
}

impl JsonRpcClient{
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

    pub async fn get_current_epoch(&self) -> Result<SolanaEpoch>{
        let response = self.post("getEpochInfo", vec![]).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn get_current_era_blocks(&self, epoch: SolanaEpoch) -> Result<Vec<u32>>{
        let response = self.post("getBlocks", vec![Value::from(epoch.absolute_slot - epoch.slot_index), Value::from(epoch.absolute_slot - epoch.slot_index + epoch.slots_in_epoch)]).await?;
        Ok(serde_json::from_value(response).unwrap())
    }
}

#[tokio::test]
async fn test_get_epoch(){
    let client: JsonRpcClient = JsonRpcClient::new(RPC_ENDPOINT).expect("Failed to construct Client");
    println!("Epoch: {:?}", client.get_current_epoch().await);
}

#[tokio::test]
async fn test_get_blocks(){
    let client: JsonRpcClient = JsonRpcClient::new(RPC_ENDPOINT).expect("Failed to construct Client");
    let epoch: SolanaEpoch = client.get_current_epoch().await.unwrap();
    let blocks = client.get_current_era_blocks(epoch).await;
    println!("Blocks: {:?}", &blocks);
}