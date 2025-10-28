use crate::*;
use async_trait::async_trait;

#[async_trait]
pub trait Submitter: Send + Sync {
    async fn submit_tx(&self, tx_hex: &str) -> Result<String, WError>;
}
