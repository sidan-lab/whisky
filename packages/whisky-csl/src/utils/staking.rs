use cardano_serialization_lib::{self as csl, WError};

pub fn script_hash_to_stake_address(script_hash: &str, network_id: u8) -> Result<String, WError> {
    let script_hash = csl::ScriptHash::from_hex(script_hash)?;
    let credential = csl::Credential::from_scripthash(&script_hash);
    let stake_address = csl::RewardAddress::new(network_id, &credential)
        .to_address()
        .to_bech32(None)?;
    Ok(stake_address)
}
