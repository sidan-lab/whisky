pub mod blockfrost;
pub mod kupo;
pub mod maestro;
pub mod ogmios;
pub use blockfrost::BlockfrostProvider;
pub use maestro::MaestroProvider;
use whisky_common::WError;
use whisky_csl::{
    apply_double_cbor_encoding,
    csl::{NativeScript, PlutusScript, ScriptRef},
};

#[derive(Debug, Clone)]
pub enum ScriptType {
    Plutus(PlutusScript),
    Native(NativeScript),
}

pub fn normalize_plutus_script(script_hex: &str) -> Result<String, WError> {
    apply_double_cbor_encoding(script_hex)
}

pub fn to_script_ref(script: &ScriptType) -> ScriptRef {
    match script {
        ScriptType::Plutus(plutus) => ScriptRef::new_plutus_script(plutus),
        ScriptType::Native(native) => ScriptRef::new_native_script(native),
    }
}
