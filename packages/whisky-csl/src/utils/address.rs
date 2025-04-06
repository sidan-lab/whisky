use cardano_serialization_lib::{self as csl};
use whisky_common::DeserializedAddress;

pub fn script_to_address(
    network_id: u8,
    script_hash: &str,
    stake_hash: Option<(&str, bool)>,
) -> String {
    match stake_hash {
        Some((stake, is_script)) => {
            let stake_cred = if is_script {
                csl::Credential::from_scripthash(&csl::ScriptHash::from_hex(stake).unwrap())
            } else {
                csl::Credential::from_keyhash(&csl::Ed25519KeyHash::from_hex(stake).unwrap())
            };

            csl::BaseAddress::new(
                network_id,
                &csl::Credential::from_scripthash(&csl::ScriptHash::from_hex(script_hash).unwrap()),
                &stake_cred,
            )
            .to_address()
            .to_bech32(None)
            .unwrap()
        }

        None => csl::EnterpriseAddress::new(
            network_id,
            &csl::Credential::from_scripthash(&csl::ScriptHash::from_hex(script_hash).unwrap()),
        )
        .to_address()
        .to_bech32(None)
        .unwrap(),
    }
}

pub fn deserialize_address(bech32_addr: &str) -> DeserializedAddress {
    let csl_address =
        csl::BaseAddress::from_address(&csl::Address::from_bech32(bech32_addr).unwrap());
    match csl_address {
        Some(address) => {
            let csl_key_hash = address
                .payment_cred()
                .to_keyhash()
                .map(|key_hash| key_hash.to_hex());

            let csl_script_hash = address
                .payment_cred()
                .to_scripthash()
                .map(|script_hash| script_hash.to_hex());

            let csl_stake_key_hash = address
                .stake_cred()
                .to_keyhash()
                .map(|stake_key_hash| stake_key_hash.to_hex());

            let csl_stake_key_script_hash = address
                .stake_cred()
                .to_scripthash()
                .map(|stake_key_script_hash| stake_key_script_hash.to_hex());

            DeserializedAddress::new(
                &csl_key_hash.unwrap_or("".to_string()),
                &csl_script_hash.unwrap_or("".to_string()),
                &csl_stake_key_hash.unwrap_or("".to_string()),
                &csl_stake_key_script_hash.unwrap_or("".to_string()),
            )
        }
        None => {
            let csl_enterprize_address = csl::EnterpriseAddress::from_address(
                &csl::Address::from_bech32(bech32_addr).unwrap(),
            )
            .unwrap();

            let csl_key_hash = csl_enterprize_address
                .payment_cred()
                .to_keyhash()
                .map(|key_hash| key_hash.to_hex());

            let csl_script_hash = csl_enterprize_address
                .payment_cred()
                .to_scripthash()
                .map(|script_hash| script_hash.to_hex());

            DeserializedAddress::new(
                &csl_key_hash.unwrap_or("".to_string()),
                &csl_script_hash.unwrap_or("".to_string()),
                "",
                "",
            )
        }
    }
}
