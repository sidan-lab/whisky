use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ScriptVersion {
    #[serde(rename = "native")]
    Native,
    #[serde(rename = "plutusv1")]
    Plutusv1,
    #[serde(rename = "plutusv2")]
    Plutusv2,
    #[serde(rename = "plutusv3")]
    Plutusv3,
}

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
    pub r#type: ScriptVersion,
}
