use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct LastUpdated {
    pub block_hash: String,
    pub block_slot: i64,
    pub timestamp: String,
}
