mod action;
mod asset;
mod js_vec;
mod serialized_address;
mod value;
pub use action::*;
pub use asset::*;
pub use js_vec::*;
pub use serialized_address::*;
pub use value::*;

#[derive(Clone, Debug, PartialEq)]
pub struct MeshTxBuilderBody {
    pub inputs: Vec<TxIn>,
    pub outputs: Vec<Output>,
    pub collaterals: Vec<PubKeyTxIn>,
    pub required_signatures: JsVecString,
    pub reference_inputs: Vec<RefTxIn>,
    pub mints: Vec<MintItem>,
    pub change_address: String,
    pub change_datum: Option<Datum>,
    pub metadata: Vec<Metadata>,
    pub validity_range: ValidityRange,
    pub signing_key: JsVecString,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Output {
    pub address: String,
    pub amount: Vec<Asset>,
    pub datum: Option<Datum>,
    pub reference_script: Option<ProvidedScriptSource>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ValidityRange {
    pub invalid_before: Option<u64>,
    pub invalid_hereafter: Option<u64>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TxIn {
    PubKeyTxIn(PubKeyTxIn),
    ScriptTxIn(ScriptTxIn),
}

#[derive(Clone, Debug, PartialEq)]
pub struct RefTxIn {
    pub tx_hash: String,
    pub tx_index: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PubKeyTxIn {
    pub type_: String,
    pub tx_in: TxInParameter,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ScriptTxIn {
    pub type_: String,
    pub tx_in: TxInParameter,
    pub script_tx_in: ScriptTxInParameter,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TxInParameter {
    pub tx_hash: String,
    pub tx_index: u32,
    pub amount: Option<Vec<Asset>>,
    pub address: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ScriptTxInParameter {
    pub script_source: Option<ScriptSource>,
    pub datum_source: Option<DatumSource>,
    pub redeemer: Option<Redeemer>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ScriptSource {
    ProvidedScriptSource(ProvidedScriptSource),
    InlineScriptSource(InlineScriptSource),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProvidedScriptSource {
    pub script_cbor: String,
    pub language_version: LanguageVersion,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InlineScriptSource {
    pub tx_hash: String,
    pub tx_index: u32,
    pub spending_script_hash: String,
    pub language_version: LanguageVersion,
}

#[derive(Clone, Debug, PartialEq)]
pub enum LanguageVersion {
    V1,
    V2,
}

#[derive(Clone, Debug, PartialEq)]
pub enum DatumSource {
    ProvidedDatumSource(ProvidedDatumSource),
    InlineDatumSource(InlineDatumSource),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProvidedDatumSource {
    pub data: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InlineDatumSource {
    pub tx_hash: String,
    pub tx_index: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ScriptSourceInfo {
    pub tx_hash: String,
    pub tx_index: u32,
    pub spending_script_hash: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MintItem {
    pub type_: String,
    pub policy_id: String,
    pub asset_name: String,
    pub amount: u64,
    pub redeemer: Option<Redeemer>,
    pub script_source: Option<ScriptSource>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Redeemer {
    pub data: String,
    pub ex_units: Budget,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Budget {
    pub mem: u64,
    pub steps: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Metadata {
    pub tag: String,
    pub metadata: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Datum {
    pub type_: String, // Currently it is either "Hash" or "Inline"
    pub data: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UtxoInput {
    pub output_index: u32,
    pub tx_hash: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UtxoOutput {
    pub address: String,
    pub amount: Vec<Asset>,
    pub data_hash: Option<String>,
    pub plutus_data: Option<String>,
    pub script_ref: Option<String>,
    pub script_hash: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UTxO {
    pub input: UtxoInput,
    pub output: UtxoOutput,
}
