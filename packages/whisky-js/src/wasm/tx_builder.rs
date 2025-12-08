use crate::*;
use wasm::WasmResult;
use whisky_common::*;
use whisky_csl::{tx_prototype::TransactionPrototype, WhiskyCSL};

/// ## WASM Transaction building method
///
/// Serialize the transaction body
///
/// ### Arguments
///
/// * `tx_builder_body_json` - The transaction builder body information, serialized as JSON string
/// * `params_json` - Optional protocol parameters, default as Cardano mainnet configuration, serialized as JSON string
///
/// ### Returns
///
/// * `String` - the built transaction hex
#[wasm_bindgen]
pub fn js_serialize_tx_body(tx_builder_body_json: &str, params_json: &str) -> WasmResult {
    let tx_builder_body: TxBuilderBody = match serde_json::from_str(tx_builder_body_json) {
        Ok(tx_builder_body) => tx_builder_body,
        Err(e) => {
            return WasmResult::new_error("failure".to_string(), format!("Invalid JSON: {:?}", e))
        }
    };

    let params: Option<Protocol> = match serde_json::from_str(params_json) {
        Ok(params) => Some(params),
        Err(e) => {
            return WasmResult::new_error(
                "failure".to_string(),
                format!("Invalid Protocol Param JSON: {:?} \n {:?}", params_json, e),
            )
        }
    };

    let mut tx_builder = WhiskyCSL::new(params).unwrap();
    tx_builder.tx_builder_body = tx_builder_body;

    match tx_builder.unbalanced_serialize_tx_body() {
        Ok(tx_hex) => WasmResult::new("success".to_string(), tx_hex.to_string()),
        Err(e) => WasmResult::new_error("failure".to_string(), format!("{:?}", e.to_string())),
    }
}

/// ## WASM Transaction Prototype to Hex
///
/// Convert a TransactionPrototype JSON to CBOR hex string
///
/// ### Arguments
///
/// * `tx_json` - The transaction prototype, serialized as JSON string
///
/// ### Returns
///
/// * `String` - the transaction CBOR hex
#[wasm_bindgen]
pub fn js_tx_prototype_to_hex(tx_json: &str) -> WasmResult {
    let tx_prototype: TransactionPrototype = match serde_json::from_str(tx_json) {
        Ok(tx) => tx,
        Err(e) => {
            return WasmResult::new_error(
                "failure".to_string(),
                format!("Invalid TransactionPrototype JSON: {:?}", e),
            )
        }
    };

    match tx_prototype.to_hex() {
        Ok(hex) => WasmResult::new("success".to_string(), hex),
        Err(e) => WasmResult::new_error("failure".to_string(), format!("{:?}", e)),
    }
}
