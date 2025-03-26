use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GovernanceProposalInfo {
    pub tx_hash: String,
    pub cert_index: usize,
    pub governance_type: String,
    pub deposit: u64,
    pub return_address: String,
    pub governance_description: String,
    pub ratified_epoch: u32,
    pub enacted_epoch: u32,
    pub dropped_epoch: u32,
    pub expired_epoch: u32,
    pub expiration: u64,
    pub metadata: serde_json::Value,
}
