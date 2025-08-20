use async_trait::async_trait;
use whisky_common::{Submitter, WError};

use super::OgmiosProvider;

#[async_trait]
impl Submitter for OgmiosProvider {
    async fn submit_tx(&self, tx_hex: &str) -> Result<String, WError> {
        let body = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "submitTransaction",
            "params": {
                "transaction": {
                    "cbor": tx_hex,
                }
            }
        });

        let resp = self
            .ogmios_client
            .post("/", body)
            .await
            .map_err(WError::from_err("Ogmios - submit_tx - post"))?;

        let v: serde_json::Value = serde_json::from_str(&resp)
            .map_err(WError::from_err("Ogmios - submit_tx - parse response"))?;

        if let Some(err) = v.get("error") {
            return Err(WError::new(
                "Ogmios - submit_tx",
                &format!("{}", err),
            ));
        }

        let tx_id = v
            .get("result")
            .ok_or_else(WError::from_opt(
                "Ogmios - submit_tx",
                "Missing transaction id in response",
            ))?;

        Ok(tx_id.to_string())
    }
}
