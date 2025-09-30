use crate::*;
use cquisitor_lib::{js_error::JsError, validators::validator::validate_transaction_js};

#[wasm_bindgen]
pub fn js_validate_tx(tx_hex: String, validation_context: String) -> Result<String, JsError> {
    validate_transaction_js(&tx_hex, &validation_context)
}
