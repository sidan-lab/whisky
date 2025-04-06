use serde::{Deserialize, Serialize};

use super::{Redeemer, ScriptSource, SimpleScriptSource};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MintItem {
    ScriptMint(ScriptMint),
    SimpleScriptMint(SimpleScriptMint),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScriptMint {
    pub mint: MintParameter,
    pub redeemer: Option<Redeemer>,
    pub script_source: Option<ScriptSource>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleScriptMint {
    pub mint: MintParameter,
    pub script_source: Option<SimpleScriptSource>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MintParameter {
    pub policy_id: String,
    pub asset_name: String,
    pub amount: i128,
}
