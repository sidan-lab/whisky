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
