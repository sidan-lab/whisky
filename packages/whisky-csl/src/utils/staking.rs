use cardano_serialization_lib::{self as csl};
use whisky_common::WError;

pub fn script_hash_to_stake_address(script_hash: &str, network_id: u8) -> Result<String, WError> {
    let script_hash = csl::ScriptHash::from_hex(script_hash).map_err(WError::from_err(
        "script_hash_to_stake_address - invalid script hash",
    ))?;
    let credential = csl::Credential::from_scripthash(&script_hash);
    let stake_address = csl::RewardAddress::new(network_id, &credential)
        .to_address()
        .to_bech32(None)
        .map_err(WError::from_err(
            "script_hash_to_stake_address - failed to convert to bech32",
        ))?;
    Ok(stake_address)
}
