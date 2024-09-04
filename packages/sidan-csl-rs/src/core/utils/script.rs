use cardano_serialization_lib::JsError;
use model::LanguageVersion;

use crate::*;

pub fn get_native_script_hash(script: &str) -> Result<String, JsError> {
    let script_hash = csl::NativeScript::from_hex(script)?.hash().to_hex();
    Ok(script_hash)
}

pub fn get_script_hash(script: &str, version: LanguageVersion) -> Result<String, JsError> {
    let language_version = match version {
        LanguageVersion::V1 => csl::Language::new_plutus_v1(),
        LanguageVersion::V2 => csl::Language::new_plutus_v2(),
        LanguageVersion::V3 => csl::Language::new_plutus_v3(),
    };
    let script_hash = csl::PlutusScript::from_hex_with_version(script, &language_version)?
        .hash()
        .to_hex();
    Ok(script_hash)
}

#[wasm_bindgen]
pub fn get_v2_script_hash(script: &str) -> String {
    csl::PlutusScript::from_hex_with_version(script, &csl::Language::new_plutus_v2())
        .unwrap()
        .hash()
        .to_hex()
}
