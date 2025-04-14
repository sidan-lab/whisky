use async_trait::async_trait;
use whisky_common::{Submitter, WError};

use super::MaestroProvider;

#[async_trait]
impl Submitter for MaestroProvider {
    async fn submit_tx(&self, tx_hex: &str) -> Result<String, WError> {
        let url = "/txmanager";
        let maestro_client = &self.maestro_client;

        let tx_binary = hex::decode(tx_hex).map_err(WError::from_err("Invalid hex data"))?;

        let req = maestro_client
            .http_client
            .post(format!("{}{}", &maestro_client.base_url, url))
            .header("Content-Type", "application/cbor")
            .body(tx_binary);

        let mut response_body = String::new();
        self.maestro_client
            .send_request(req, &mut response_body)
            .await
            .map_err(WError::from_err("Maestro - submit_tx"))?;

        Ok(response_body)
    }
}
