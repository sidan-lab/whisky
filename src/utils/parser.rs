use hex;

pub fn bytes_to_hex(bytes: &[u8]) -> String {
    hex::encode(bytes)
}

#[test]
fn test_bytes_to_hex() {
    let bytes = vec![0, 1, 2, 3, 4, 5];
    assert_eq!(bytes_to_hex(&bytes), "000102030405");
}

pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, hex::FromHexError> {
    hex::decode(hex)
}

#[test]
fn test_hex_to_bytes() {
    let bytes = vec![0, 1, 2, 3, 4, 255];
    assert_eq!(hex_to_bytes("0001020304ff").unwrap(), bytes);
}

pub fn string_to_hex(s: &str) -> String {
    hex::encode(s)
}

#[test]
fn test_string_to_hex() {
    assert_eq!(string_to_hex("DELTA"), "44454c5441");
}

pub fn hex_to_string(hex: &str) -> Result<String, std::str::Utf8Error> {
    let bytes = hex::decode(hex).unwrap();
    Ok(std::str::from_utf8(&bytes)?.to_string())
}

#[test]
fn test_hex_to_string() {
    assert_eq!(hex_to_string("44454c5441").unwrap(), "DELTA");
}
