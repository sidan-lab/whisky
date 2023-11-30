use std::collections::HashMap;

pub struct MeshTxBuilderBody {
    pub inputs: Vec<TxIn>,
    pub outputs: Vec<Output>,
    pub collaterals: Vec<PubKeyTxIn>,
    pub required_signatures: Vec<String>,
    pub reference_inputs: Vec<RefTxIn>,
    pub mints: Vec<MintItem>,
    pub change_address: String,
    pub metadata: Vec<Metadata>,
    pub validity_range: ValidityRange,
    pub signing_key: Vec<String>,
}

pub struct Output {
    pub address: String,
    pub amount: Vec<Asset>,
    pub datum: Option<Datum>,
    pub reference_script: Option<String>,
}

pub struct ValidityRange {
    pub invalid_before: Option<u64>,
    pub invalid_hereafter: Option<u64>,
}

pub enum TxIn {
    PubKeyTxIn(PubKeyTxIn),
    ScriptTxIn(ScriptTxIn),
}

pub struct PubKeyTxIn {
    pub type_: String,
    pub tx_in: TxInParameter,
}

pub struct ScriptTxIn {
    pub type_: String,
    pub tx_in: TxInParameter,
    pub script_tx_in: ScriptTxInParameter,
}

pub struct TxInParameter {
    pub tx_hash: String,
    pub tx_index: u64,
    pub amount: Option<Vec<Asset>>,
    pub address: Option<String>,
}

pub struct ScriptTxInParameter {
    pub script_source: Option<ScriptSource>,
    pub datum_source: Option<DatumSource>,
    pub redeemer: Option<Redeemer>,
}

pub struct ScriptSource {
    pub type_: String,
    pub script_cbor: String,
}

pub struct DatumSource {
    pub type_: String,
    pub data: Data,
}

pub struct ScriptSourceInfo {
    pub tx_hash: String,
    pub tx_index: u64,
    pub spending_script_hash: Option<String>,
}

pub struct MintItem {
    pub type_: String,
    pub policy_id: String,
    pub asset_name: String,
    pub amount: u64,
    pub redeemer: Option<Redeemer>,
    pub script_source: Option<ScriptSource>,
}

pub struct Redeemer {
    pub data: Data,
    pub ex_units: Budget,
}

pub struct Asset {
    pub unit: String,
    pub quantity: String,
}

pub struct Budget {
    pub mem: u64,
    pub steps: u64,
}

pub enum Data {
    String(String),
    Number(u64),
    Array(Vec<Data>),
    Map(HashMap<Data, Data>),
    Alternative { alternative: u64, fields: Vec<Data> },
}
