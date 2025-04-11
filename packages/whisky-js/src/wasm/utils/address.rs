use crate::*;
use serde_json::{from_str, Value};
use whisky_csl::csl;

#[wasm_bindgen]
pub fn deserialize_bech32_address(bech32_addr: &str) -> WasmDeserializedAddress {
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

            WasmDeserializedAddress::new(
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

            WasmDeserializedAddress::new(
                &csl_key_hash.unwrap_or("".to_string()),
                &csl_script_hash.unwrap_or("".to_string()),
                "",
                "",
            )
        }
    }
}

#[wasm_bindgen]
pub fn parse_plutus_address_obj_to_bech32(plutus_data_address_obj: &str, network_id: u8) -> String {
    let plutus_data_address: Value =
        from_str(plutus_data_address_obj).expect("Invalid json string");
    let plutus_data_key_obj = plutus_data_address.get("fields").unwrap();
    let plutus_data_key_list = plutus_data_key_obj.as_array().unwrap();

    let plutus_data_payment_key_obj = &plutus_data_key_list[0];
    let plutus_data_stake_key_obj = &plutus_data_key_list[1];

    let payment_key_hash = plutus_data_payment_key_obj["fields"][0]["bytes"]
        .as_str()
        .unwrap();

    let csl_payment_credential =
        if plutus_data_payment_key_obj["constructor"].as_u64().unwrap() == 0 {
            csl::Credential::from_keyhash(&csl::Ed25519KeyHash::from_hex(payment_key_hash).unwrap())
        } else {
            csl::Credential::from_scripthash(&csl::ScriptHash::from_hex(payment_key_hash).unwrap())
        };

    if plutus_data_stake_key_obj["constructor"].as_u64().unwrap() == 0 {
        let stake_key_hash = plutus_data_stake_key_obj["fields"][0]["fields"][0]["fields"][0]
            ["bytes"]
            .as_str()
            .unwrap();
        if plutus_data_stake_key_obj["fields"][0]["fields"][0]["constructor"]
            .as_u64()
            .unwrap()
            == 0
        {
            csl::BaseAddress::new(
                network_id,
                &csl_payment_credential,
                &csl::Credential::from_keyhash(
                    &csl::Ed25519KeyHash::from_hex(stake_key_hash).unwrap(),
                ),
            )
            .to_address()
            .to_bech32(None)
            .unwrap()
        } else {
            csl::BaseAddress::new(
                network_id,
                &csl_payment_credential,
                &csl::Credential::from_scripthash(
                    &csl::ScriptHash::from_hex(stake_key_hash).unwrap(),
                ),
            )
            .to_address()
            .to_bech32(None)
            .unwrap()
        }
    } else {
        csl::EnterpriseAddress::new(network_id, &csl_payment_credential)
            .to_address()
            .to_bech32(None)
            .unwrap()
    }
}
