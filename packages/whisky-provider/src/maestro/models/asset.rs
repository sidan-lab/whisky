use super::*;
use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Standards {
    pub cip25_metadata: Option<HashMap<String, serde_json::Value>>,
    pub cip68_metadata: Option<HashMap<String, serde_json::Value>>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct AssetInformation {
    pub asset_name: String,
    pub asset_name_ascii: String,
    pub asset_standards: Standards,
    pub burn_tx_count: i64,
    pub fingerprint: String,
    pub first_mint_tx: HashMap<String, serde_json::Value>,
    pub latest_mint_tx_metadata: Option<HashMap<String, serde_json::Value>>,
    pub mint_tx_count: i64,
    pub token_registry_metadata: Option<HashMap<String, serde_json::Value>>,
    pub total_supply: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AssetInformations {
    pub data: AssetInformation,
    pub last_updated: LastUpdated,
    pub next_cursor: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AssetInfo {
    pub asset_name: String,
    pub asset_name_ascii: String,
    pub fingerprint: String,
    pub total_supply: String,
    pub asset_standards: Standards,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CollectionAssets {
    pub data: Vec<AssetInfo>,
    pub last_updated: LastUpdated,
    pub next_cursor: Option<String>,
}
