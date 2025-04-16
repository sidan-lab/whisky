use serde::{Deserialize, Serialize};

use super::Asset;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlockfrostTxInfo {
    /// Transaction hash
    #[serde(rename = "hash")]
    pub hash: String,
    /// Block hash
    #[serde(rename = "block")]
    pub block: String,
    /// Block number
    #[serde(rename = "block_height")]
    pub block_height: i32,
    /// Block creation time in UNIX time
    #[serde(rename = "block_time")]
    pub block_time: i32,
    /// Slot number
    #[serde(rename = "slot")]
    pub slot: i32,
    /// Transaction index within the block
    #[serde(rename = "index")]
    pub index: i32,
    #[serde(rename = "output_amount")]
    pub output_amount: Vec<Asset>,
    /// Fees of the transaction in Lovelaces
    #[serde(rename = "fees")]
    pub fees: String,
    /// Deposit within the transaction in Lovelaces
    #[serde(rename = "deposit")]
    pub deposit: String,
    /// Size of the transaction in Bytes
    #[serde(rename = "size")]
    pub size: i32,
    /// Left (included) endpoint of the timelock validity intervals
    #[serde(rename = "invalid_before", deserialize_with = "Option::deserialize")]
    pub invalid_before: Option<String>,
    /// Right (excluded) endpoint of the timelock validity intervals
    #[serde(rename = "invalid_hereafter", deserialize_with = "Option::deserialize")]
    pub invalid_hereafter: Option<String>,
    /// Count of UTXOs within the transaction
    #[serde(rename = "utxo_count")]
    pub utxo_count: i32,
    /// Count of the withdrawals within the transaction
    #[serde(rename = "withdrawal_count")]
    pub withdrawal_count: i32,
    /// Count of the MIR certificates within the transaction
    #[serde(rename = "mir_cert_count")]
    pub mir_cert_count: i32,
    /// Count of the delegations within the transaction
    #[serde(rename = "delegation_count")]
    pub delegation_count: i32,
    /// Count of the stake keys (de)registration within the transaction
    #[serde(rename = "stake_cert_count")]
    pub stake_cert_count: i32,
    /// Count of the stake pool registration and update certificates within the transaction
    #[serde(rename = "pool_update_count")]
    pub pool_update_count: i32,
    /// Count of the stake pool retirement certificates within the transaction
    #[serde(rename = "pool_retire_count")]
    pub pool_retire_count: i32,
    /// Count of asset mints and burns within the transaction
    #[serde(rename = "asset_mint_or_burn_count")]
    pub asset_mint_or_burn_count: i32,
    /// Count of redeemers within the transaction
    #[serde(rename = "redeemer_count")]
    pub redeemer_count: i32,
    /// True if contract script passed validation
    #[serde(rename = "valid_contract")]
    pub valid_contract: bool,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlockfrostTxUtxo {
    /// Transaction hash
    #[serde(rename = "hash")]
    pub hash: String,
    #[serde(rename = "inputs")]
    pub inputs: Vec<BlockfrostTxUtxoInputs>,
    #[serde(rename = "outputs")]
    pub outputs: Vec<BlockfrostTxUtxoOutputs>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlockfrostTxUtxoInputs {
    /// Input address
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "amount")]
    pub amount: Vec<Asset>,
    /// Hash of the UTXO transaction
    #[serde(rename = "tx_hash")]
    pub tx_hash: String,
    /// UTXO index in the transaction
    #[serde(rename = "output_index")]
    pub output_index: i32,
    /// The hash of the transaction output datum
    #[serde(rename = "data_hash", deserialize_with = "Option::deserialize")]
    pub data_hash: Option<String>,
    /// CBOR encoded inline datum
    #[serde(rename = "inline_datum", deserialize_with = "Option::deserialize")]
    pub inline_datum: Option<String>,
    /// The hash of the reference script of the input
    #[serde(
        rename = "reference_script_hash",
        deserialize_with = "Option::deserialize"
    )]
    pub reference_script_hash: Option<String>,
    /// Whether the input is a collateral consumed on script validation failure
    #[serde(rename = "collateral")]
    pub collateral: bool,
    /// Whether the input is a reference transaction input
    #[serde(rename = "reference", skip_serializing_if = "Option::is_none")]
    pub reference: Option<bool>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlockfrostTxUtxoOutputs {
    /// Output address
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "amount")]
    pub amount: Vec<Asset>,
    /// UTXO index in the transaction
    #[serde(rename = "output_index")]
    pub output_index: i32,
    /// The hash of the transaction output datum
    #[serde(rename = "data_hash", deserialize_with = "Option::deserialize")]
    pub data_hash: Option<String>,
    /// CBOR encoded inline datum
    #[serde(rename = "inline_datum", deserialize_with = "Option::deserialize")]
    pub inline_datum: Option<String>,
    /// Whether the output is a collateral output
    #[serde(rename = "collateral")]
    pub collateral: bool,
    /// The hash of the reference script of the output
    #[serde(
        rename = "reference_script_hash",
        deserialize_with = "Option::deserialize"
    )]
    pub reference_script_hash: Option<String>,
    /// Transaction hash that consumed the UTXO or null for unconsumed UTXOs. Always null for collateral outputs.
    pub consumed_by_tx: Option<Option<String>>,
}
