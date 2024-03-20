use crate::*;
use cardano_serialization_lib as csl;

#[wasm_bindgen]
pub fn get_v2_script_hash(script: &str) -> String {
    csl::plutus::PlutusScript::from_hex_with_version(
        script,
        &csl::plutus::Language::new_plutus_v2(),
    )
    .unwrap()
    .hash()
    .to_hex()
}
