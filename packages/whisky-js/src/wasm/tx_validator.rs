use crate::*;
use cquisitor_lib::validators::validator::validate_transaction_js;

#[wasm_bindgen]
pub fn js_validate_tx(tx_hex: String, validation_context: String) -> WasmResult {
    let eval_result = validate_transaction_js(&tx_hex, &validation_context);
    match eval_result {
        Ok(result) => {
            let result_json = serde_json::to_string(&result).unwrap();
            WasmResult::new("success".to_string(), result_json)
        }
        Err(e) => WasmResult::new_error("failure".to_string(), format!("{:?}", e.to_string())),
    }
}
