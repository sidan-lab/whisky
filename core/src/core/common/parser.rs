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

pub fn hex_to_string(hex: &str) -> Result<String, std::str::Utf8Error> {
    let bytes = hex::decode(hex).unwrap();
    Ok(std::str::from_utf8(&bytes)?.to_string())
}
