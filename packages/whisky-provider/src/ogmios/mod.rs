mod submitter;
use whisky_common::*;

use reqwest::RequestBuilder;
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct Ogmios {
    http_client: reqwest::Client,
    pub base_url: String,
}

impl Ogmios {
    pub fn new(base_url: String) -> Self {
        let http_client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(300))
            .build()
            .expect("Failed to create HTTP client");

        Ogmios {
            http_client,
            base_url,
        }
    }

    async fn send_request(
        &self,
        req: RequestBuilder,
        response_body: &mut String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let req = req.header("Accept", "application/json").build()?;

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
            .map_err(WError::from_err("ogmios - get - send_request"))?;
        Ok(response_body)
    }

    pub async fn post<T: Serialize>(&self, url: &str, body: T) -> Result<String, WError> {
        let json_body =
            serde_json::to_string(&body).map_err(WError::from_err("ogmios - post - json_body"))?;

        let req = self
            .http_client
            .post(format!("{}{}", &self.base_url, url))
            .header("Content-Type", "application/json")
            .body(json_body);

        let mut response_body = String::new();
        self.send_request(req, &mut response_body)
            .await
            .map_err(WError::from_err("ogmios - post - send_request"))?;
        Ok(response_body)
    }
}

#[derive(Clone, Debug)]
pub struct OgmiosProvider {
    pub ogmios_client: Ogmios,
}

impl OgmiosProvider {
    pub fn new(base_url: &str) -> OgmiosProvider {
        let ogmios_client = Ogmios::new(base_url.to_string());
        OgmiosProvider { ogmios_client }
    }
}
