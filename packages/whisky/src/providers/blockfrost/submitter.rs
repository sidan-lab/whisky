use async_trait::async_trait;
use whisky_common::{Submitter, WError};

use super::BlockfrostProvider;

#[async_trait]
impl Submitter for BlockfrostProvider {
    async fn submit_tx(&self, tx_hex: &str) -> Result<String, WError> {
        let url = "/tx/submit";
        let blockfrost_client = &self.blockfrost_client;

        let tx_binary = hex::decode(tx_hex).map_err(WError::from_err("Invalid hex data"))?;
        let req = blockfrost_client
            .http_client
            .post(format!("{}{}", &blockfrost_client.base_url, url))
            .header("Content-Type", "application/cbor")
            .body(tx_binary);

        let mut response_body = String::new();
        self.blockfrost_client
            .send_request(req, &mut response_body)
            .await
            .map_err(WError::from_err("Blockfrost - submit_tx"))?;

        let tx_hash = response_body.trim_matches('"').to_string();

        Ok(tx_hash)
    }
}
