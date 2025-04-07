use serde::Deserialize;
use serde::Serialize;

use crate::provider::maestro::utils::last_updated::LastUpdated;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ScriptVersion {
    plutusv1,
    plutusv2,
    plutusv3,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Script {
    pub bytes: String,
    pub hash: String,
    pub json: serde_json::Value,
    pub r#type: ScriptVersion,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ScriptByHash {
    pub data: Script,
    pub last_updated: LastUpdated,
}
