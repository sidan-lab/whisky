use cardano_serialization_lib::JsError;
use serde_json::json;
use wasm::WasmResult;

use crate::{core::tx_parser::TxParser, *};

#[wasm_bindgen]
pub fn js_parse_tx_body(tx_hex: &str) -> WasmResult {
    let tx_parser = TxParser::new(tx_hex);
    match tx_parser {
        Err(e) => WasmResult::new_error("failure".to_string(), format!("{:?}", e)),
        Ok(parser) => WasmResult::new("success".to_string(), (json!(parser)).to_string()),
    }
}

#[wasm_bindgen]
pub fn js_get_tx_outs_utxo(tx_hex: &str) -> WasmResult {
    let get_tx_outs_utxo = || -> Result<String, JsError> {
        let tx_parser = TxParser::new(tx_hex)?;
        let tx_outs = tx_parser.get_tx_outs_utxo()?;
        Ok((json!(tx_outs)).to_string())
    };
    let res = get_tx_outs_utxo();
    WasmResult::from_result(res)
}

#[test]
fn test_parse_tx_body() {
    let tx_hex = "84a300d90102818258202cb57168ee66b68bd04a0d595060b546edf30c04ae1031b883c9ac797967dd8503018282581d60f95cab9352c14782a366802b7967746a89356e8915c17006149ff68c0082581d60f95cab9352c14782a366802b7967746a89356e8915c17006149ff68c1b000000024d95f5570200a0f5f6";
    let result = js_get_tx_outs_utxo(tx_hex);
    println!("{:?}", result);
}
