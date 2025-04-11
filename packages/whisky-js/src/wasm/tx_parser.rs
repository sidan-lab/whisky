use crate::*;
use serde_json::json;
use wasm::WasmResult;
use whisky_common::*;
use whisky_csl::*;

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
    let get_tx_outs_utxo = || -> Result<String, WError> {
        let tx_parser = TxParser::new(tx_hex)?;
        let tx_outs = tx_parser.get_tx_outs_utxo()?;
        Ok((json!(tx_outs)).to_string())
    };
    let res = get_tx_outs_utxo();
    WasmResult::from_result(res)
}
