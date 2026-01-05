use whisky_common::WError;

pub fn bytes_from_bech32(bech32_str: &str) -> Result<String, WError> {
    let (_hrp, data) = bech32::decode(bech32_str)
        .map_err(|e| WError::new("Bech32 decode error", &format!("{}", e)))?;
    Ok(hex::encode(data))
}
