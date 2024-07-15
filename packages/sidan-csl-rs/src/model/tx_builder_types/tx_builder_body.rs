use serde::{Deserialize, Serialize};

use crate::model::JsVecString;

use super::{
    Certificate, Datum, Metadata, MintItem, Output, PubKeyTxIn, RefTxIn, TxIn, ValidityRange,
    Withdrawal,
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeshTxBuilderBody {
    pub inputs: Vec<TxIn>,
    pub outputs: Vec<Output>,
    pub collaterals: Vec<PubKeyTxIn>,
    pub required_signatures: JsVecString,
    pub reference_inputs: Vec<RefTxIn>,
    pub withdrawals: Vec<Withdrawal>,
    pub mints: Vec<MintItem>,
    pub change_address: String,
    pub change_datum: Option<Datum>,
    pub metadata: Vec<Metadata>,
    pub validity_range: ValidityRange,
    pub certificates: Vec<Certificate>,
    pub signing_key: JsVecString,
}
