use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Asset {
    pub amount: i64,
    pub unit: String,
}
#[derive(Deserialize, Debug, Clone)]
pub struct Utxo {
    pub address: String,
    pub assets: Vec<Asset>,
    pub datum: Option<HashMap<String, serde_json::Value>>,
    pub index: i64,
    pub reference_script: Option<ReferenceScript>,
    pub tx_hash: String,
    #[serde(alias = "txout_cbor")]
    pub tx_out_cbor: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ReferenceScript {
    pub bytes: String,
    pub hash: String,
    pub json: Option<HashMap<String, serde_json::Value>>,
    pub r#type: String,
}
