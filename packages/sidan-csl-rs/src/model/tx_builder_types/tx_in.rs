use serde::{Deserialize, Serialize};

use crate::model::{Asset, Redeemer};

use super::{
    DatumSource, InlineSimpleScriptSource, ProvidedSimpleScriptSource, ScriptSource, UTxO,
    UtxoInput, UtxoOutput,
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TxIn {
    PubKeyTxIn(PubKeyTxIn),
    SimpleScriptTxIn(SimpleScriptTxIn),
    ScriptTxIn(ScriptTxIn),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RefTxIn {
    pub tx_hash: String,
    pub tx_index: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PubKeyTxIn {
    pub tx_in: TxInParameter,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleScriptTxIn {
    pub tx_in: TxInParameter,
    pub simple_script_tx_in: SimpleScriptTxInParameter,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SimpleScriptTxInParameter {
    ProvidedSimpleScriptSource(ProvidedSimpleScriptSource),
    InlineSimpleScriptSource(InlineSimpleScriptSource),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScriptTxIn {
    pub tx_in: TxInParameter,
    pub script_tx_in: ScriptTxInParameter,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TxInParameter {
    pub tx_hash: String,
    pub tx_index: u32,
    pub amount: Option<Vec<Asset>>,
    pub address: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScriptTxInParameter {
    pub script_source: Option<ScriptSource>,
    pub datum_source: Option<DatumSource>,
    pub redeemer: Option<Redeemer>,
}

impl TxIn {
    pub fn to_utxo(&self) -> UTxO {
        match self {
            TxIn::PubKeyTxIn(pub_key_tx_in) => UTxO {
                input: UtxoInput {
                    output_index: pub_key_tx_in.tx_in.tx_index,
                    tx_hash: pub_key_tx_in.tx_in.tx_hash.clone(),
                },
                output: UtxoOutput {
                    address: pub_key_tx_in.tx_in.address.clone().unwrap(),
                    amount: pub_key_tx_in.tx_in.amount.clone().unwrap(),
                    data_hash: None,
                    plutus_data: None,
                    script_ref: None,
                    script_hash: None,
                },
            },
            TxIn::SimpleScriptTxIn(simple_script_tx_in) => UTxO {
                input: UtxoInput {
                    output_index: simple_script_tx_in.tx_in.tx_index,
                    tx_hash: simple_script_tx_in.tx_in.tx_hash.clone(),
                },
                output: UtxoOutput {
                    address: simple_script_tx_in.tx_in.address.clone().unwrap(),
                    amount: simple_script_tx_in.tx_in.amount.clone().unwrap(),
                    data_hash: None,
                    plutus_data: None,
                    script_ref: None,
                    script_hash: None,
                },
            },
            TxIn::ScriptTxIn(script_tx_in) => UTxO {
                input: UtxoInput {
                    output_index: script_tx_in.tx_in.tx_index,
                    tx_hash: script_tx_in.tx_in.tx_hash.clone(),
                },
                output: UtxoOutput {
                    address: script_tx_in.tx_in.address.clone().unwrap(),
                    amount: script_tx_in.tx_in.amount.clone().unwrap(),
                    data_hash: None,
                    plutus_data: None,
                    script_ref: None,
                    script_hash: None,
                },
            },
        }
    }
}
