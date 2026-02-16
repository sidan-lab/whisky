use std::str::FromStr;

use pallas::ledger::addresses::{Address, Network, ShelleyAddress, ShelleyDelegationPart};
use pallas_crypto::hash::Hash;
use whisky_common::{DeserializedAddress, WError};

pub fn script_to_address(
    network_id: u8,
    script_hash: &str,
    stake_cred: Option<(&str, bool)>,
) -> String {
    let stake_cred = match stake_cred {
        Some((stake, is_script)) => {
            let stake_cred = if is_script {
                ShelleyDelegationPart::Script(Hash::from_str(stake).unwrap())
            } else {
                ShelleyDelegationPart::Key(Hash::from_str(stake).unwrap())
            };
            stake_cred
        }
        None => ShelleyDelegationPart::Null,
    };
    let payment_cred =
        pallas::ledger::addresses::ShelleyPaymentPart::Script(Hash::from_str(script_hash).unwrap());

    let address = ShelleyAddress::new(
        Network::try_from(network_id).unwrap(),
        payment_cred,
        stake_cred,
    );
    address.to_bech32().unwrap()
}

pub fn serialize_address_obj(
    address_obj: DeserializedAddress,
    network_id: u8,
) -> Result<String, WError> {
    let payment_cred = match (
        address_obj.pub_key_hash.as_str(),
        address_obj.script_hash.as_str(),
    ) {
        (pub_key_hash, "") => {
            pallas::ledger::addresses::ShelleyPaymentPart::Key(Hash::from_str(pub_key_hash).unwrap())
        }
        ("", script_hash) => {
            pallas::ledger::addresses::ShelleyPaymentPart::Script(Hash::from_str(script_hash).unwrap())
        }
        _ => Err(WError::new(
            "serialze_address_obj",
            &format!(
                "Must provide exactly one of pub_key_hash or script_hash, pub_key_hash: {}, script_hash: {}",
                address_obj.pub_key_hash, address_obj.script_hash
            ),
        ))?,
    };

    let stake_cred_opt = match (
        address_obj.stake_key_hash.as_str(),
        address_obj.stake_key_script_hash.as_str(),
    ) {
        ("","") => None,
        (stake_key_hash, "") => Some(ShelleyDelegationPart::Key(Hash::from_str(stake_key_hash).unwrap())),
        ("", stake_script_hash) => Some(ShelleyDelegationPart::Script(Hash::from_str(stake_script_hash).unwrap())),
        _ => Err(WError::new(
            "serialze_address_obj",
            &format!(
                "Must provide at most one of stake_key_hash or stake_script_hash, stake_key_hash: {}, stake_script_hash: {}",
                address_obj.stake_key_hash, address_obj.stake_key_script_hash
            ),
        ))?,
    };

    match stake_cred_opt {
        Some(stake_cred) => Ok(ShelleyAddress::new(
            Network::try_from(network_id).unwrap(),
            payment_cred,
            stake_cred,
        )
        .to_bech32()
        .unwrap()),
        None => Ok(ShelleyAddress::new(
            Network::try_from(network_id).unwrap(),
            payment_cred,
            ShelleyDelegationPart::Null,
        )
        .to_bech32()
        .unwrap()),
    }
}

pub fn deserialize_address(bech32_address: &str) -> Result<DeserializedAddress, WError> {
    let address = Address::from_bech32(bech32_address).map_err(|e| {
        WError::new(
            "deserialize_address",
            &format!(
                "Failed to parse address from bech32: {}, error: {}",
                bech32_address, e
            ),
        )
    })?;

    match address {
        Address::Byron(byron_address) => {
            return Err(WError::new(
                "deserialize_address",
                &format!(
                    "Byron addresses are not supported: {}",
                    byron_address.to_base58()
                ),
            ))?
        }
        Address::Shelley(shelley_address) => {
            let (payment_pkh, payment_script_hash) = match shelley_address.payment() {
                pallas::ledger::addresses::ShelleyPaymentPart::Key(key_hash) => {
                    (key_hash.to_string(), String::new())
                }
                pallas::ledger::addresses::ShelleyPaymentPart::Script(script_hash) => {
                    (String::new(), script_hash.to_string())
                }
            };
            let (stake_pkh, stake_script_hash) = match shelley_address.delegation() {
                ShelleyDelegationPart::Null => (String::new(), String::new()),
                ShelleyDelegationPart::Key(stake_key_hash) => {
                    (stake_key_hash.to_string(), String::new())
                }
                ShelleyDelegationPart::Script(stake_script_hash) => {
                    (String::new(), stake_script_hash.to_string())
                }
                ShelleyDelegationPart::Pointer(_pointer) => {
                    return Err(WError::new(
                        "deserialize_address",
                        "Pointer stake keys are not supported",
                    ))
                }
            };

            Ok(DeserializedAddress::new(
                &payment_pkh,
                &payment_script_hash,
                &stake_pkh,
                &stake_script_hash,
            ))
        }
        Address::Stake(stake_address) => {
            let stake_payload = stake_address.payload();
            let (stake_pkh, stake_script_hash) = match stake_payload {
                pallas::ledger::addresses::StakePayload::Stake(hash) => {
                    (hash.to_string(), String::new())
                }
                pallas::ledger::addresses::StakePayload::Script(hash) => {
                    (String::new(), hash.to_string())
                }
            };

            Ok(DeserializedAddress::new(
                &String::new(),
                &String::new(),
                &stake_pkh,
                &stake_script_hash,
            ))
        }
    }
}
