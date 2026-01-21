use bech32::{self};
use whisky_common::WError;

pub fn bytes_from_bech32(bech32_str: &str) -> Result<String, WError> {
    let (_hrp, data) = bech32::decode(bech32_str)
        .map_err(|e| WError::new("Bech32 decode error", &format!("{}", e)))?;
    Ok(hex::encode(data))
}

pub fn bech32_from_bytes(bytes_hex: &str) -> Result<String, WError> {
    let bytes = hex::decode(bytes_hex)
        .map_err(|e| WError::new("Address bytes decode error", &format!("{}", e)))?;

    let header_byte = bytes
        .first()
        .ok_or_else(|| WError::new("Bech32 encode error", "Empty bytes"))?;
    // Determine HRP based on header byte, if last bit is 0, it's testnet, else mainnet
    let hrp = if header_byte & 0b1 == 0 {
        "addr_test"
    } else {
        "addr"
    };
    let bech32_str = bech32::encode::<bech32::Bech32>(bech32::Hrp::parse(hrp).unwrap(), &bytes)
        .map_err(|e| WError::new("Bech32 encode error", &format!("{}", e)))?;
    Ok(bech32_str)
}
