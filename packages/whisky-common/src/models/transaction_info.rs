use serde::{Deserialize, Serialize};

use super::UTxO;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionInfo {
    pub index: u32,
    pub block: String,
    pub hash: String,
    pub slot: String,
    pub fees: String,
    pub size: u32,
    pub deposit: String,
    pub invalid_before: String,
    pub invalid_after: String,
    pub inputs: Vec<UTxO>,
    pub outputs: Vec<UTxO>,
    pub block_height: Option<u32>,
    pub block_time: Option<u64>,
}
