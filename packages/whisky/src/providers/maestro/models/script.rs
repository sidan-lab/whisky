use super::*;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Script {
    pub bytes: String,
    pub hash: String,
    pub json: serde_json::Value,
    pub r#type: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ScriptByHash {
    pub data: Script,
    pub last_updated: LastUpdated,
}
