use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoyaltiesStandard {
    pub rate: String,
    pub address: String,
}

pub const ROYALTIES_STANDARD_KEYS: &[&str] = &["rate", "address"];

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AssetMetadata {
    FungibleAssetMetadata(FungibleAssetMetadata),
    NonFungibleAssetMetadata(NonFungibleAssetMetadata),
    Royalties(RoyaltiesStandard),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FungibleAssetMetadata {
    pub ticker: String,
    pub decimals: u32,
    pub version: String,
}

pub const FUNGIBLE_ASSET_KEYS: &[&str] = &["ticker", "decimals"];

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum NonFungibleAssetMetadata {
    Audio(AudioAssetMetadata),
    Image(ImageAssetMetadata),
    Smart(SmartAssetMetadata),
    Video(VideoAssetMetadata),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioAssetMetadata {
    pub files: Option<Vec<File>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageAssetMetadata {
    pub files: Option<Vec<File>>,
    pub artists: Option<Vec<Artist>>,
    pub attributes: Option<std::collections::HashMap<String, String>>,
    pub traits: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmartAssetMetadata {
    pub files: Option<Vec<File>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoAssetMetadata {
    pub files: Option<Vec<File>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub name: String,
    pub src: String,
    pub media_type: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Artist {
    pub name: String,
    pub twitter: Option<String>,
}

pub const METADATA_STANDARD_KEYS: &[&str] = &[
    "name",
    "image",
    "mediaType",
    "description",
    "instagram",
    "twitter",
    "discord",
    "website",
];
