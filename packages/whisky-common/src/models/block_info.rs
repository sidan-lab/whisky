use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockInfo {
    pub time: u64,
    pub hash: String,
    pub slot: String,
    pub epoch: u32,
    pub epoch_slot: String,
    pub slot_leader: String,
    pub size: usize,
    pub tx_count: usize,
    pub output: String,
    pub fees: String,
    pub previous_block: String,
    pub next_block: String,
    pub confirmations: usize,
    pub operational_certificate: String,
    pub vrf_key: String,
}
