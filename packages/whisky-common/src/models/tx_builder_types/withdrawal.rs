use serde::{Deserialize, Serialize};

use super::{Redeemer, ScriptSource, SimpleScriptSource};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Withdrawal {
    PubKeyWithdrawal(PubKeyWithdrawal),
    PlutusScriptWithdrawal(PlutusScriptWithdrawal),
    SimpleScriptWithdrawal(SimpleScriptWithdrawal),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PubKeyWithdrawal {
    pub address: String,
    pub coin: u64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlutusScriptWithdrawal {
    pub address: String,
    pub coin: u64,
    pub script_source: Option<ScriptSource>,
    pub redeemer: Option<Redeemer>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleScriptWithdrawal {
    pub address: String,
    pub coin: u64,
    pub script_source: Option<SimpleScriptSource>,
}
