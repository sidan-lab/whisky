use serde::{Deserialize, Serialize};

use super::{
    Certificate, Datum, Metadata, MintItem, Network, Output, PubKeyTxIn, RefTxIn, TxIn,
    ValidityRange, Vote, Withdrawal,
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TxBuilderBody {
    pub inputs: Vec<TxIn>,
    pub outputs: Vec<Output>,
    pub collaterals: Vec<PubKeyTxIn>,
    pub required_signatures: Vec<String>,
    pub reference_inputs: Vec<RefTxIn>,
    pub withdrawals: Vec<Withdrawal>,
    pub mints: Vec<MintItem>,
    pub change_address: String,
    pub change_datum: Option<Datum>,
    pub metadata: Vec<Metadata>,
    pub validity_range: ValidityRange,
    pub certificates: Vec<Certificate>,
    pub votes: Vec<Vote>,
    pub signing_key: Vec<String>,
    pub network: Option<Network>,
}
