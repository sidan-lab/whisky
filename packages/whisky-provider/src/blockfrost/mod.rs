mod evaluator;
mod fetcher;
pub mod models;
mod submitter;
pub mod utils;
use std::collections::HashMap;

use whisky_common::*;

use reqwest::RequestBuilder;
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct Blockfrost {
    project_id: String,
    http_client: reqwest::Client,
    pub base_url: String,
}

impl Blockfrost {
    pub fn new(project_id: String, network: String) -> Self {
        let base_url = format!("https://cardano-{}.blockfrost.io/api/v0", &network);
        let http_client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(300))
            .build()
            .expect("Failed to create HTTP client");

        Blockfrost {
            project_id,
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
            .header("project_id", &self.project_id)
            .build()?;

        let response = self.http_client.execute(req).await?;

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

    async fn fetch_specific_script(&self, script_hash: &str) -> Result<models::Script, WError> {
        let url = format!("/scripts/{}", script_hash);

        let resp = self
            .get(&url)
            .await
            .map_err(WError::from_err("blockfrost::fetch_specific_script"))?;

        let script: models::Script = serde_json::from_str(&resp).map_err(WError::from_err(
            "blockfrost::fetch_specific_script type error",
        ))?;

        Ok(script)
    }
    async fn fetch_plutus_script_cbor(&self, script_hash: &str) -> Result<String, WError> {
        let url = format!("/scripts/{}/cbor", script_hash);

        let resp = self
            .get(&url)
            .await
            .map_err(WError::from_err("blockfrost::fetch_plutus_script_cbor"))?;

        let script_cbor: HashMap<String, String> = serde_json::from_str(&resp).map_err(
            WError::from_err("blockfrost::fetch_plutus_script_cbor type error"),
        )?;
        let cbor = script_cbor["cbor"].clone();

        Ok(cbor)
    }

    async fn fetch_native_script_json(
        &self,
        script_hash: &str,
    ) -> Result<serde_json::Value, WError> {
        let url = format!("/scripts/{}/json", script_hash);

        let resp = self
            .get(&url)
            .await
            .map_err(WError::from_err("blockfrost::fetch_native_script_json"))?;

        let script_json: HashMap<String, serde_json::Value> = serde_json::from_str(&resp).map_err(
            WError::from_err("blockfrost::fetch_native_script_json type error"),
        )?;
        let json = script_json["json"].clone();

        Ok(json)
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
