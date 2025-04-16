use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, PartialEq, Clone, Serialize)]
pub struct Asset {
    pub unit: String,
    pub quantity: String,
}
#[derive(Deserialize, Debug, Clone)]
pub struct BlockfrostUtxo {
    /// Bech32 encoded addresses
    #[serde(rename = "address")]
    pub address: String,
    /// Transaction hash of the UTXO
    #[serde(rename = "tx_hash")]
    pub tx_hash: String,
    /// UTXO index in the transaction
    #[serde(rename = "tx_index")]
    pub tx_index: i32,
    /// UTXO index in the transaction
    #[serde(rename = "output_index")]
    pub output_index: i32,
    #[serde(rename = "amount")]
    pub amount: Vec<Asset>,
    /// Block hash of the UTXO
    #[serde(rename = "block")]
    pub block: String,
    /// The hash of the transaction output datum
    #[serde(rename = "data_hash", deserialize_with = "Option::deserialize")]
    pub data_hash: Option<String>,
    /// CBOR encoded inline datum
    #[serde(rename = "inline_datum", deserialize_with = "Option::deserialize")]
    pub inline_datum: Option<String>,
    /// The hash of the reference script of the output
    #[serde(
        rename = "reference_script_hash",
        deserialize_with = "Option::deserialize"
    )]
    pub reference_script_hash: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ReferenceScript {
    pub bytes: String,
    pub hash: String,
    pub json: Option<HashMap<String, serde_json::Value>>,
    pub r#type: String,
}
