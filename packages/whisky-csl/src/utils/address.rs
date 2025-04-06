use cardano_serialization_lib::{self as csl};

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
