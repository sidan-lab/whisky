use crate::model::Asset;
use serde::{Deserialize, Serialize};

use super::{Datum, ProvidedScriptSource, ProvidedSimpleScriptSource};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OutputScriptSource {
    ProvidedSimpleScriptSource(ProvidedSimpleScriptSource),
    ProvidedScriptSource(ProvidedScriptSource),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    pub address: String,
    pub amount: Vec<Asset>,
    pub datum: Option<Datum>,
    pub reference_script: Option<OutputScriptSource>,
}
