use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountInfo {
    pub active: bool,
    pub pool_id: String,
    pub balance: String,
    pub rewards: String,
    pub withdrawals: String,
}
