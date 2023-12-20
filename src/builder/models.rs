use std::collections::HashMap;

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct Output {
    pub address: String,
    pub amount: Vec<Asset>,
    pub datum: Option<Datum>,
    pub reference_script: Option<ProvidedScriptSource>,
}

#[derive(Clone, Debug)]
pub struct ValidityRange {
    pub invalid_before: Option<u64>,
    pub invalid_hereafter: Option<u64>,
}

#[derive(Clone, Debug)]
pub enum TxIn {
    PubKeyTxIn(PubKeyTxIn),
    ScriptTxIn(ScriptTxIn),
}

#[derive(Clone, Debug)]
pub struct RefTxIn {
    pub tx_hash: String,
    pub tx_index: u32,
}

#[derive(Clone, Debug)]
pub struct PubKeyTxIn {
    pub type_: String,
    pub tx_in: TxInParameter,
}

#[derive(Clone, Debug)]
pub struct ScriptTxIn {
    pub type_: String,
    pub tx_in: TxInParameter,
    pub script_tx_in: ScriptTxInParameter,
}

#[derive(Clone, Debug)]
pub struct TxInParameter {
    pub tx_hash: String,
    pub tx_index: u32,
    pub amount: Option<Vec<Asset>>,
    pub address: Option<String>,
}

#[derive(Clone, Debug)]
pub struct ScriptTxInParameter {
    pub script_source: Option<ScriptSource>,
    pub datum_source: Option<DatumSource>,
    pub redeemer: Option<Redeemer>,
}

#[derive(Clone, Debug)]
pub enum ScriptSource {
    ProvidedScriptSource(ProvidedScriptSource),
    InlineScriptSource(InlineScriptSource),
}

#[derive(Clone, Debug)]
pub struct ProvidedScriptSource {
    pub script_cbor: String,
    pub language_version: LanguageVersion,
}

#[derive(Clone, Debug)]
pub struct InlineScriptSource {
    pub tx_hash: String,
    pub tx_index: u32,
    pub spending_script_hash: String,
    pub language_version: LanguageVersion
}

#[derive(Clone, Debug)]
pub enum LanguageVersion {
    V1,
    V2,
}

#[derive(Clone, Debug)]
pub enum DatumSource {
    ProvidedDatumSource(ProvidedDatumSource),
    InlineDatumSource(InlineDatumSource),
}

#[derive(Clone, Debug)]
pub struct ProvidedDatumSource {
    pub data: String,
}

#[derive(Clone, Debug)]
pub struct InlineDatumSource {
    pub tx_hash: String,
    pub tx_index: u32,
}

#[derive(Clone, Debug)]
pub struct ScriptSourceInfo {
    pub tx_hash: String,
    pub tx_index: u32,
    pub spending_script_hash: Option<String>,
}

#[derive(Clone, Debug)]
pub struct MintItem {
    pub type_: String,
    pub policy_id: String,
    pub asset_name: String,
    pub amount: u64,
    pub redeemer: Option<Redeemer>,
    pub script_source: Option<ScriptSource>,
}

#[derive(Clone, Debug)]
pub struct Redeemer {
    pub data: String,
    pub ex_units: Budget,
}

#[derive(Clone, Debug)]
pub struct Asset {
    pub unit: String,
    pub quantity: String,
}

#[derive(Clone, Debug)]
pub struct Budget {
    pub mem: u64,
    pub steps: u64,
}

#[derive(Clone, Debug)]
pub struct Metadata {
    pub tag: String,
    pub metadata: HashMap<String, String>,
}

#[derive(Clone, Debug)]
pub struct Datum {
    pub type_: String,
    pub data: String,
}
