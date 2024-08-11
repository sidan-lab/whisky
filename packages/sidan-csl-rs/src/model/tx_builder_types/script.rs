use serde::{Deserialize, Serialize};

use super::RefTxIn;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SimpleScriptSource {
    ProvidedSimpleScriptSource(ProvidedSimpleScriptSource),
    InlineSimpleScriptSource(InlineSimpleScriptSource),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvidedSimpleScriptSource {
    pub script_cbor: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InlineSimpleScriptSource {
    pub ref_tx_in: RefTxIn,
    pub simple_script_hash: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ScriptSource {
    ProvidedScriptSource(ProvidedScriptSource),
    InlineScriptSource(InlineScriptSource),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvidedScriptSource {
    pub script_cbor: String,
    pub language_version: LanguageVersion,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InlineScriptSource {
    pub ref_tx_in: RefTxIn,
    pub script_hash: String,
    pub language_version: LanguageVersion,
    pub script_size: usize,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LanguageVersion {
    V1,
    V2,
    V3,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DatumSource {
    ProvidedDatumSource(ProvidedDatumSource),
    InlineDatumSource(InlineDatumSource),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvidedDatumSource {
    pub data: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InlineDatumSource {
    pub tx_hash: String,
    pub tx_index: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScriptSourceInfo {
    pub tx_hash: String,
    pub tx_index: u32,
    pub spending_script_hash: Option<String>,
}
