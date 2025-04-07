use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use whisky_common::*;

pub struct FetcherOptions {
    pub max_page: Option<usize>,
    pub order: Option<FetchOrder>,
    pub additional_options: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum FetchOrder {
    Asc,
    Desc,
}
use std::{collections::HashMap, error::Error};

#[async_trait]
pub trait Fetcher: Send + Sync {
    async fn fetch_account_info(&self, address: &str) -> Result<AccountInfo, WError>;
    async fn fetch_address_utxos(
        &self,
        address: &str,
        asset: Option<&str>,
    ) -> Result<Vec<UTxO>, WError>;
    async fn fetch_address_txs(
        &self,
        address: &str,
        options: Option<FetcherOptions>,
    ) -> Result<Vec<TransactionInfo>, WError>;
    async fn fetch_asset_addresses(&self, asset: &str) -> Result<Vec<(String, String)>, WError>;
    async fn fetch_asset_metadata(
        &self,
        asset: &str,
    ) -> Result<HashMap<String, serde_json::Value>, WError>;
    async fn fetch_block_info(&self, hash: &str) -> Result<BlockInfo, WError>;
    async fn fetch_collection_assets(
        &self,
        policy_id: &str,
        cursor: Option<String>,
    ) -> Result<(Vec<(String, String)>, Option<String>), WError>;
    async fn fetch_protocol_parameters(&self, epoch: Option<u32>) -> Result<Protocol, WError>;
    async fn fetch_tx_info(&self, hash: &str) -> Result<TransactionInfo, WError>;
    async fn fetch_utxos(&self, hash: &str, index: Option<u32>) -> Result<Vec<UTxO>, WError>;
    async fn get(&self, url: &str) -> Result<serde_json::Value, WError>;
}
