use maestro_rust_sdk::utils::LastUpdated;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
pub struct CollectionAssets {
    pub data: Vec<AssetInfo>,
    pub _last_updated: LastUpdated,
    pub next_cursor: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AssetInfo {
    pub asset_name: String,
    pub asset_name_ascii: String,
    pub fingerprint: String,
    pub total_supply: String,
    pub asset_standards: AssetStandards,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AssetStandards {
    pub cip25_metadata: Cip25Metadata,
    pub cip68_metadata: Option<Cip68Metadata>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Cip25Metadata {
    pub augmentations: Vec<serde_json::Value>,
    pub core: Cip25Core,
    pub description: String,
    pub image: String,
    pub name: String,
    pub website: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Cip25Core {
    pub handle_encoding: String,
    pub og: u32,
    pub prefix: String,
    pub terms_of_use: String,
    pub version: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Cip68Metadata {
    // Add fields for CIP-68 metadata here
}

#[derive(Debug, Deserialize, Clone)]
pub struct AssetMetadata {
    pub data: serde_json::Value,
    pub _last_updated: LastUpdated,
}
