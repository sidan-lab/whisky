use crate::*;
use serde_json::json;
use wasm::WasmResult;
use whisky_common::*;
use whisky_csl::*;

#[wasm_bindgen]
pub fn js_parse_tx_body(tx_hex: &str, resolved_utxos: &JsVecString) -> WasmResult {
    let mut deserialized_utxos: Vec<UTxO> = Vec::new();
    for utxo_json in resolved_utxos {
        match serde_json::from_str(utxo_json.as_str()) {
            Ok(utxo) => deserialized_utxos.push(utxo),
            Err(e) => {
                return WasmResult::new_error(
                    "failure".to_string(),
                    format!("Error in decoding UTXO: {:?}", e),
                );
            }
        }
    }
    let mut tx_parser = CSLParser::new();
    let tx_parser = tx_parser.parse(tx_hex, &deserialized_utxos);
    let builder_body = tx_parser.map(|parser| {
        let builder_body = parser.tx_body.clone();
        json!(builder_body).to_string()
    });
    WasmResult::from_result(builder_body)
}

#[wasm_bindgen]
pub fn js_get_tx_outs_utxo(tx_hex: &str) -> WasmResult {
    let get_tx_outs_utxo = || -> Result<String, WError> {
        let tx_outs = CSLParser::extract_output_utxos(tx_hex)?;
        Ok(json!(tx_outs).to_string())
    };
    let res = get_tx_outs_utxo();
    WasmResult::from_result(res)
}

#[wasm_bindgen]
pub fn js_get_required_inputs_to_resolve(tx_hex: &str) -> WasmResult {
    let get_required_inputs = || -> Result<String, WError> {
        let required_inputs = CSLParser::extract_all_required_inputs(tx_hex)?;
        Ok(json!(required_inputs).to_string())
    };
    let res = get_required_inputs();
    WasmResult::from_result(res)
}
