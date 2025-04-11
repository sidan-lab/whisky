use serde::{Deserialize, Serialize};

use crate::models::Asset;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UtxoInput {
    pub output_index: u32,
    pub tx_hash: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UtxoOutput {
    pub address: String,
    pub amount: Vec<Asset>,
    pub data_hash: Option<String>,
    pub plutus_data: Option<String>,
    pub script_ref: Option<String>,
    pub script_hash: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UTxO {
    pub input: UtxoInput,
    pub output: UtxoOutput,
}
