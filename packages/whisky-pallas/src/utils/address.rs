use whisky_common::DeserializedAddress;

pub fn deserialize_address(bech32_addr: &str) -> DeserializedAddress {
    // let csl_address =
    //     csl::BaseAddress::from_address(&csl::Address::from_bech32(bech32_addr).unwrap());
    // match csl_address {
    //     Some(address) => {
    //         let csl_key_hash = address
    //             .payment_cred()
    //             .to_keyhash()
    //             .map(|key_hash| key_hash.to_hex());

    //         let csl_script_hash = address
    //             .payment_cred()
    //             .to_scripthash()
    //             .map(|script_hash| script_hash.to_hex());

    //         let csl_stake_key_hash = address
    //             .stake_cred()
    //             .to_keyhash()
    //             .map(|stake_key_hash| stake_key_hash.to_hex());

    //         let csl_stake_key_script_hash = address
    //             .stake_cred()
    //             .to_scripthash()
    //             .map(|stake_key_script_hash| stake_key_script_hash.to_hex());

    //         DeserializedAddress::new(
    //             &csl_key_hash.unwrap_or("".to_string()),
    //             &csl_script_hash.unwrap_or("".to_string()),
    //             &csl_stake_key_hash.unwrap_or("".to_string()),
    //             &csl_stake_key_script_hash.unwrap_or("".to_string()),
    //         )
    //     }
    //     None => {
    //         let csl_enterprize_address = csl::EnterpriseAddress::from_address(
    //             &csl::Address::from_bech32(bech32_addr).unwrap(),
    //         )
    //         .unwrap();

    //         let csl_key_hash = csl_enterprize_address
    //             .payment_cred()
    //             .to_keyhash()
    //             .map(|key_hash| key_hash.to_hex());

    //         let csl_script_hash = csl_enterprize_address
    //             .payment_cred()
    //             .to_scripthash()
    //             .map(|script_hash| script_hash.to_hex());

    //         DeserializedAddress::new(
    //             &csl_key_hash.unwrap_or("".to_string()),
    //             &csl_script_hash.unwrap_or("".to_string()),
    //             "",
    //             "",
    //         )
    //     }
    // }
    DeserializedAddress::new("", "", "", "")
}
