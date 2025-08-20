use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, PartialEq, Clone, Serialize)]
pub struct KupoValue {
    pub coins: i128,
    pub assets: HashMap<String, i128>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct KupoUtxo {
    /// Bech32 encoded addresses
    #[serde(rename = "address")]
    pub address: String,
    /// Transaction hash of the UTXO
    #[serde(rename = "transaction_id")]
    pub tx_hash: String,
    /// UTXO index in the transaction
    #[serde(rename = "transaction_index")]
    pub tx_index: i32,
    /// UTXO index in the transaction
    #[serde(rename = "output_index")]
    pub output_index: i32,
    #[serde(rename = "value")]
    pub value: KupoValue,
    /// CBOR encoded datum
    #[serde(rename = "datum", deserialize_with = "Option::deserialize")]
    pub datum: Option<String>,
    /// The hash of the transaction output datum
    #[serde(rename = "datum_type")]
    pub datum_type: String,
    /// The hash of the script of the output
    #[serde(rename = "script_hash", deserialize_with = "Option::deserialize")]
    pub script_hash: Option<String>,
    /// The resolved script
    #[serde(rename = "script")]
    pub script: Option<Script>,
    /// Block reference at which this transaction was included in the ledger
    #[serde(rename = "created_at")]
    pub created_at: Point,
    /// Block reference at which this transaction input was spent
    #[serde(rename = "spent_at")]
    pub spent_at: Option<Point>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Point {
    pub slot_no: i128,
    pub header_hash: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Script {
    pub language: String,
    pub headerscript_hash: String,
}
