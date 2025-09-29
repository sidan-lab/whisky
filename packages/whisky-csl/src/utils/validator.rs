use cardano_serialization_lib::JsError;
use cquisitor_lib::validators::input_contexts::ValidationInputContext;
use cquisitor_lib::validators::validation_result::ValidationResult;
use cquisitor_lib::validators::validator::validate_transaction;

pub fn validate_tx(
    tx_hex: &str,
    validation_context: ValidationInputContext,
) -> Result<ValidationResult, JsError> {
    validate_transaction(tx_hex, validation_context).map_err(|e| JsError::from_str(&e.to_string()))
}
