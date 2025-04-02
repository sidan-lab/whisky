use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct LastUpdated {
    pub _block_hash: String,
    pub _block_slot: i64,
    pub _timestamp: String,
}
