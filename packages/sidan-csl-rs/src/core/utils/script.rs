use crate::*;

#[wasm_bindgen]
pub fn get_v2_script_hash(script: &str) -> String {
    csl::PlutusScript::from_hex_with_version(
        script,
        &csl::Language::new_plutus_v2(),
    )
    .unwrap()
    .hash()
    .to_hex()
}
