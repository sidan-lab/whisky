mod fetcher;
pub mod models;
pub mod utils;
use whisky_common::*;

use reqwest::RequestBuilder;
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct Blockfrost {
    api_key: String,
    http_client: reqwest::Client,
    pub base_url: String,
}

impl Blockfrost {
    pub fn new(api_key: String, network: String) -> Self {
        let base_url = format!("https://cardano-{}.blockfrost.io/api/v0", &network);
        let http_client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(300))
            .build()
            .expect("Failed to create HTTP client");

        Blockfrost {
            api_key,
            http_client,
            base_url,
        }
    }

    async fn send_request(
        &self,
        req: RequestBuilder,
        response_body: &mut String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let req = req
            .header("Accept", "application/json")
            .header("api-key", &self.api_key)
            .build()?;

        println!("req: {:?}", req);

        let response = self.http_client.execute(req).await?;

        println!("response: {:?}", response);

        if response.status().is_success() {
            *response_body = response.text().await?;
            Ok(())
        } else {
            Err(format!("Error: {}", response.status()).into())
        }
    }

    pub async fn get(&self, url: &str) -> Result<String, WError> {
        let req = self.http_client.get(format!("{}{}", &self.base_url, url));
        let mut response_body = String::new();
        self.send_request(req, &mut response_body)
            .await
            .map_err(WError::from_err("Blockfrost - get - send_request"))?;
        Ok(response_body)
    }

    pub async fn post<T: Serialize>(&self, url: &str, body: T) -> Result<String, WError> {
        let json_body = serde_json::to_string(&body)
            .map_err(WError::from_err("Blockfrost - post - json_body"))?;

        let req = self
            .http_client
            .post(format!("{}{}", &self.base_url, url))
            .header("Content-Type", "application/json")
            .body(json_body);

        let mut response_body = String::new();
        self.send_request(req, &mut response_body)
            .await
            .map_err(WError::from_err("Blockfrost - post - send_request"))?;
        Ok(response_body)
    }
}

#[derive(Clone, Debug)]
pub struct BlockfrostProvider {
    pub blockfrost_client: Blockfrost,
}

impl BlockfrostProvider {
    pub fn new(api_key: &str, network: &str) -> BlockfrostProvider {
        let blockfrost_client = Blockfrost::new(api_key.to_string(), network.to_string());
        BlockfrostProvider { blockfrost_client }
    }
}
