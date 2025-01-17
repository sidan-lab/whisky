use cardano_serialization_lib::JsError;
use hex;

pub fn bytes_to_hex(bytes: &[u8]) -> String {
    hex::encode(bytes)
}

pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, hex::FromHexError> {
    hex::decode(hex)
}

pub fn string_to_hex(s: &str) -> String {
    hex::encode(s)
}

pub fn hex_to_string(hex: &str) -> Result<String, JsError> {
    let bytes = hex::decode(hex)
        .map_err(|err| JsError::from_str(&format!("Invalid hex string found: {}", err)))?;
    Ok(std::str::from_utf8(&bytes)
        .map_err(|err| JsError::from_str(&format!("Invalid bytes for utf-8 found: {}", err)))?
        .to_string())
}
