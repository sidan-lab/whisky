use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlockfrostAsset {
    /// Address containing the specific asset
    #[serde(rename = "address")]
    pub address: String,
    /// Asset quantity on the specific address
    #[serde(rename = "quantity")]
    pub quantity: String,
}
