mod evaluator;
mod fetcher;
pub mod models;
pub mod utils;
use whisky_common::*;

use maestro_rust_sdk::models::transactions::RedeemerEvaluation;
use reqwest::RequestBuilder;
use serde::Serialize;

#[derive(Serialize)]
pub struct EvaluateTx {
    cbor: String,
    additional_utxos: Vec<AdditionalUtxo>,
}

#[derive(Serialize)]
pub struct AdditionalUtxo {
    pub tx_hash: String,
    pub index: u32,
    pub txout_cbor: String,
}

#[derive(Debug, Clone)]
pub struct Maestro {
    api_key: String,
    http_client: reqwest::Client,
    pub base_url: String,
}

impl Maestro {
    pub fn new(api_key: String, network: String) -> Self {
        let base_url = format!("https://{}.gomaestro-api.org/v1", &network,);
        let http_client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(300))
            .build()
            .expect("Failed to create HTTP client");

        Maestro {
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
            .map_err(WError::from_err("Maestro - get - send_request"))?;
        Ok(response_body)
    }

    pub async fn post<T: Serialize>(&self, url: &str, body: T) -> Result<String, WError> {
        let json_body =
            serde_json::to_string(&body).map_err(WError::from_err("Maestro - post - json_body"))?;

        let req = self
            .http_client
            .post(format!("{}{}", &self.base_url, url))
            .header("Content-Type", "application/json")
            .body(json_body);

        let mut response_body = String::new();
        self.send_request(req, &mut response_body)
            .await
            .map_err(WError::from_err("Maestro - post - send_request"))?;
        Ok(response_body)
    }

    pub async fn evaluate_tx(
        &self,
        tx_cbor: &str,
        additional_utxos: Vec<AdditionalUtxo>,
    ) -> Result<Vec<RedeemerEvaluation>, WError> {
        let url = "/transactions/evaluate";
        let body = EvaluateTx {
            cbor: tx_cbor.to_string(),
            additional_utxos,
        };
        let resp = self
            .post(url, &body)
            .await
            .map_err(WError::from_err("Maestro - evaluate_tx - post"))?;
        let redeemer_evaluations =
            serde_json::from_str(&resp).map_err(WError::from_err("Maestro - evaluate_tx"))?;
        Ok(redeemer_evaluations)
    }
}

#[derive(Clone, Debug)]
pub struct MaestroProvider {
    pub maestro_client: Maestro,
}

impl MaestroProvider {
    pub fn new(api_key: &str, network: &str) -> MaestroProvider {
        let maestro_client = Maestro::new(api_key.to_string(), network.to_string());
        MaestroProvider { maestro_client }
    }
}
