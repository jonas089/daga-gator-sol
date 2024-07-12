use serde_json::{from_value, json, Error, Value};
use jsonrpsee_http_client::{HttpClientBuilder, HttpClient, HeaderValue, HeaderMap};
use jsonrpsee_core::client::ClientT;
use crate::types::{self, SolanaEpoch};
use anyhow::Result;

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
}

#[tokio::test]
async fn test_epoch_response(){
    use types::SolanaEpoch;
    let rpc_endpoint: &str = "https://api.devnet.solana.com";
    let client: JsonRpcClient = JsonRpcClient::new(rpc_endpoint).expect("Failed to construct Client");
    println!("Epoch: {:?}", client.get_current_epoch().await);
}