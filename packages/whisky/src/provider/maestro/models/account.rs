use serde::Deserialize;

// use crate::provider::maestro::utils::last_updated::LastUpdated;

#[derive(Deserialize, Debug, Clone)]
pub struct AccountInformation {
    pub delegated_pool: Option<String>,
    pub registered: bool,
    pub rewards_available: i64,
    // pub stake_address: String,
    pub total_balance: i64,
    // pub total_rewarded: i64,
    pub total_withdrawn: i64,
    // pub utxo_balance: i64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct StakeAccountInformation {
    pub data: AccountInformation,
    // pub last_updated: LastUpdated,
}
