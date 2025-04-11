use crate::errors::WError;

pub fn bytes_to_hex(bytes: &[u8]) -> String {
    hex::encode(bytes)
}

pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, WError> {
    hex::decode(hex).map_err(|err| WError::new("hex_to_bytes", err.to_string().as_str()))
}

pub fn string_to_hex(s: &str) -> String {
    hex::encode(s)
}

pub fn hex_to_string(hex: &str) -> Result<String, WError> {
    let bytes = hex::decode(hex).map_err(|err| {
        WError::new(
            "hex_to_string",
            &format!("Invalid hex string found: {}", err),
        )
    })?;
    Ok(std::str::from_utf8(&bytes)
        .map_err(|err| {
            WError::new(
                "hex_to_string",
                &format!("Invalid bytes for utf-8 found: {}", err),
            )
        })?
        .to_string())
}
