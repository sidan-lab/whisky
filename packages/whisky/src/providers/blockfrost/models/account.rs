use serde::Deserialize;

// use crate::provider::maestro::utils::last_updated::LastUpdated;

#[derive(Deserialize, Debug, Clone)]
pub struct BlockfrostAccountInfo {
    pub active: bool,
    pub pool_id: Option<String>,
    pub controlled_amount: String,
    pub withdrawable_amount: String,
    pub withdrawals_sum: String,
}
