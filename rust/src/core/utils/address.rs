use crate::{model::*, *};
use cardano_serialization_lib as csl;

#[wasm_bindgen]
pub fn script_to_address(
    script_hash: String,
    stake_hash: Option<String>,
    network_id: u8,
) -> String {
    match stake_hash {
        Some(stake) => csl::address::BaseAddress::new(
            network_id,
            &csl::address::StakeCredential::from_scripthash(
                &csl::crypto::ScriptHash::from_hex(&script_hash).unwrap(),
            ),
            &csl::address::StakeCredential::from_keyhash(
                &csl::crypto::Ed25519KeyHash::from_hex(&stake).unwrap(),
            ),
        )
        .to_address()
        .to_bech32(None)
        .unwrap(),

        None => csl::address::EnterpriseAddress::new(
            network_id,
            &csl::address::StakeCredential::from_scripthash(
                &csl::crypto::ScriptHash::from_hex(&script_hash).unwrap(),
            ),
        )
        .to_address()
        .to_bech32(None)
        .unwrap(),
    }
}

pub fn serialize_bech32_address(bech32_addr: String) -> SerializedAddress {
    let csl_address = csl::address::BaseAddress::from_address(
        &csl::address::Address::from_bech32(&bech32_addr).unwrap(),
    );
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

            SerializedAddress {
                pub_key_hash: csl_key_hash.unwrap_or("".to_string()),
                script_hash: csl_script_hash.unwrap_or("".to_string()),
                stake_key_hash: csl_stake_key_hash.unwrap_or("".to_string()),
            }
        }
        None => {
            let csl_enterprize_address = csl::address::EnterpriseAddress::from_address(
                &csl::address::Address::from_bech32(&bech32_addr).unwrap(),
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

            SerializedAddress {
                pub_key_hash: csl_key_hash.unwrap_or("".to_string()),
                script_hash: csl_script_hash.unwrap_or("".to_string()),
                stake_key_hash: "".to_string(),
            }
        }
    }
}

pub fn address_bech32_to_obj(_bech32: &str) {}

// export const addrBech32ToObj = <T>(bech32: string): T => {
//     const hexAddress = csl.Address.from_bech32(bech32).to_hex();
//     const cslAddress = csl.Address.from_hex(hexAddress);
//     const json = JSON.parse(csl.PlutusData.from_address(cslAddress).to_json(1));
//     return json;
// };
