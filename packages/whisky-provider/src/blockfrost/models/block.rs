use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlockContent {
    /// Block creation time in UNIX time
    #[serde(rename = "time")]
    pub time: i32,
    /// Block number
    #[serde(rename = "height", deserialize_with = "Option::deserialize")]
    pub height: Option<i32>,
    /// Hash of the block
    #[serde(rename = "hash")]
    pub hash: String,
    /// Slot number
    #[serde(rename = "slot", deserialize_with = "Option::deserialize")]
    pub slot: Option<i32>,
    /// Epoch number
    #[serde(rename = "epoch", deserialize_with = "Option::deserialize")]
    pub epoch: Option<i32>,
    /// Slot within the epoch
    #[serde(rename = "epoch_slot", deserialize_with = "Option::deserialize")]
    pub epoch_slot: Option<i32>,
    /// Bech32 ID of the slot leader or specific block description in case there is no slot leader
    #[serde(rename = "slot_leader")]
    pub slot_leader: String,
    /// Block size in Bytes
    #[serde(rename = "size")]
    pub size: i32,
    /// Number of transactions in the block
    #[serde(rename = "tx_count")]
    pub tx_count: i32,
    /// Total output within the block in Lovelaces
    #[serde(rename = "output", deserialize_with = "Option::deserialize")]
    pub output: Option<String>,
    /// Total fees within the block in Lovelaces
    #[serde(rename = "fees", deserialize_with = "Option::deserialize")]
    pub fees: Option<String>,
    /// VRF key of the block
    #[serde(rename = "block_vrf", deserialize_with = "Option::deserialize")]
    pub block_vrf: Option<String>,
    /// The hash of the operational certificate of the block producer
    #[serde(rename = "op_cert", deserialize_with = "Option::deserialize")]
    pub op_cert: Option<String>,
    /// The value of the counter used to produce the operational certificate
    #[serde(rename = "op_cert_counter", deserialize_with = "Option::deserialize")]
    pub op_cert_counter: Option<String>,
    /// Hash of the previous block
    #[serde(rename = "previous_block", deserialize_with = "Option::deserialize")]
    pub previous_block: Option<String>,
    /// Hash of the next block
    #[serde(rename = "next_block", deserialize_with = "Option::deserialize")]
    pub next_block: Option<String>,
    /// Number of block confirmations
    #[serde(rename = "confirmations")]
    pub confirmations: i32,
}
