use crate::*;
use cquisitor_lib::{
    js_error::JsError,
    validators::{input_contexts::ValidationInputContext, validator::validate_transaction_js},
};

#[wasm_bindgen]
pub fn js_validate_tx(tx_hex: &str, validation_context: &str) -> Result<String, JsError> {
    // First, deserialize the JSON string into our JS-friendly wrapper type
    let validation_context_js: ValidationInputContextJS =
        serde_json::from_str(validation_context).map_err(|e| JsError::new(&e.to_string()))?;

    // Convert the JS wrapper type to the original type
    let validation_context: ValidationInputContext = validation_context_js.into();

    // Serialize the converted type back to JSON for the original function
    let validation_context_json =
        serde_json::to_string(&validation_context).map_err(|e| JsError::new(&e.to_string()))?;

    // Call the original validation function
    validate_transaction_js(&tx_hex, &validation_context_json)
}
