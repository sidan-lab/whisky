use crate::*;
use whisky_csl::{calculate_tx_hash, sign_transaction};

use wasm::WasmResult;

#[wasm_bindgen]
pub fn js_calculate_tx_hash(tx_hex: &str) -> WasmResult {
    let result = calculate_tx_hash(tx_hex);
    WasmResult::from_result(result)
}

#[wasm_bindgen]
pub fn js_sign_transaction(tx_hex: String, signing_keys: JsVecString) -> WasmResult {
    let result = sign_transaction(
        &tx_hex,
        signing_keys
            .iter()
            .map(|x| x.as_str())
            .collect::<Vec<&str>>()
            .as_slice(),
    );
    WasmResult::from_result(result)
}
