use whisky_common::WError;

use crate::wrapper::transaction_body::RewardAccount;

pub fn script_hash_to_stake_address(script_hash: &str, network_id: u8) -> Result<String, WError> {
    let script_hash_bytes = hex::decode(script_hash).map_err(|e| {
        WError::new(
            "script_hash_to_stake_address - invalid script hash",
            &format!("Hex decode error: {}", e.to_string()),
        )
    })?;
    let header_byte: u8 = if network_id == 1 {
        0b1111_0001 // Mainnet
    } else {
        0b1111_0000 // Testnet
    };
    // concat header byte and script hash bytes
    let mut address_bytes = vec![header_byte];
    address_bytes.extend(script_hash_bytes);
    RewardAccount::from_bytes(&address_bytes)?.to_bech32()
}
