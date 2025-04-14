use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssetAddresses {
    /// Address containing the specific asset
    #[serde(rename = "address")]
    pub address: String,
    /// Asset quantity on the specific address
    #[serde(rename = "quantity")]
    pub quantity: String,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlockfrostAsset {
    /// Hex-encoded asset full name
    #[serde(rename = "asset")]
    pub asset: String,
    /// Policy ID of the asset
    #[serde(rename = "policy_id")]
    pub policy_id: String,
    /// Hex-encoded asset name of the asset
    #[serde(rename = "asset_name", deserialize_with = "Option::deserialize")]
    pub asset_name: Option<String>,
    /// CIP14 based user-facing fingerprint
    #[serde(rename = "fingerprint")]
    pub fingerprint: String,
    /// Current asset quantity
    #[serde(rename = "quantity")]
    pub quantity: String,
    /// ID of the initial minting transaction
    #[serde(rename = "initial_mint_tx_hash")]
    pub initial_mint_tx_hash: String,
    /// Count of mint and burn transactions
    #[serde(rename = "mint_or_burn_count")]
    pub mint_or_burn_count: i32,
    /// On-chain metadata which SHOULD adhere to the valid standards, based on which we perform the look up and display the asset (best effort)
    #[serde(rename = "onchain_metadata", deserialize_with = "Option::deserialize")]
    pub onchain_metadata: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// If on-chain metadata passes validation, we display the standard under which it is valid
    pub onchain_metadata_standard: Option<Option<OnchainMetadataStandard>>,
    /// Arbitrary plutus data (CIP68).
    pub onchain_metadata_extra: Option<Option<String>>,
    #[serde(rename = "metadata", deserialize_with = "Option::deserialize")]
    pub metadata: Option<Box<AssetMetadata>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OnchainMetadataStandard {
    #[serde(rename = "CIP25v1")]
    Cip25v1,
    #[serde(rename = "CIP25v2")]
    Cip25v2,
    #[serde(rename = "CIP68v1")]
    Cip68v1,
    #[serde(rename = "CIP68v2")]
    Cip68v2,
    #[serde(rename = "CIP68v3")]
    Cip68v3,
}

impl Default for OnchainMetadataStandard {
    fn default() -> OnchainMetadataStandard {
        Self::Cip25v1
    }
}

/// AssetMetadata : Off-chain metadata fetched from GitHub based on network. Mainnet: https://github.com/cardano-foundation/cardano-token-registry/ Testnet: https://github.com/input-output-hk/metadata-registry-testnet/
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssetMetadata {
    /// Asset name
    #[serde(rename = "name")]
    pub name: String,
    /// Asset description
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "ticker", deserialize_with = "Option::deserialize")]
    pub ticker: Option<String>,
    /// Asset website
    #[serde(rename = "url", deserialize_with = "Option::deserialize")]
    pub url: Option<String>,
    /// Base64 encoded logo of the asset
    #[serde(rename = "logo", deserialize_with = "Option::deserialize")]
    pub logo: Option<String>,
    /// Number of decimal places of the asset unit
    #[serde(rename = "decimals", deserialize_with = "Option::deserialize")]
    pub decimals: Option<i32>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssetPolicy {
    /// Concatenation of the policy_id and hex-encoded asset_name
    #[serde(rename = "asset")]
    pub asset: String,
    /// Current asset quantity
    #[serde(rename = "quantity")]
    pub quantity: String,
}
