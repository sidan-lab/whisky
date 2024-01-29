use cardano_serialization_lib as csl;

use crate::model::builder::*;
use hex::FromHex;

pub fn to_bignum(val: u64) -> csl::utils::BigNum {
    csl::utils::BigNum::from_str(&val.to_string()).unwrap()
}

pub fn build_tx_builder() -> csl::tx_builder::TransactionBuilder {
    let cfg = csl::tx_builder::TransactionBuilderConfigBuilder::new()
        .fee_algo(&csl::fees::LinearFee::new(
            &to_bignum(44),
            &to_bignum(155381),
        ))
        .pool_deposit(&to_bignum(500000000))
        .key_deposit(&to_bignum(2000000))
        .max_value_size(5000)
        .max_tx_size(16384)
        .coins_per_utxo_byte(&to_bignum(4310))
        .ex_unit_prices(&csl::plutus::ExUnitPrices::new(
            &csl::UnitInterval::new(&to_bignum(577), &to_bignum(10000)),
            &csl::UnitInterval::new(&to_bignum(721), &to_bignum(10000000)),
        ))
        .build()
        .unwrap();
    csl::tx_builder::TransactionBuilder::new(&cfg)
}

pub fn to_value(assets: &Vec<Asset>) -> csl::utils::Value {
    let lovelace = assets.iter().find(|asset| asset.unit == "lovelace");
    let mut multi_asset = csl::MultiAsset::new();

    for asset in assets {
        if asset.unit == "lovelace" {
            continue;
        }
        let mut policy_assets = csl::Assets::new();
        let name_bytes =
            Vec::<u8>::from_hex(&asset.unit[56..]).expect("Failed to parse hex asset name");
        policy_assets.insert(
            &csl::AssetName::new(name_bytes).unwrap(),
            &csl::utils::BigNum::from_str(&asset.quantity.to_string()).unwrap(),
        );

        multi_asset.insert(
            &csl::crypto::ScriptHash::from_hex(&asset.unit[0..56]).unwrap(),
            &policy_assets,
        );
    }

    let lovelace_asset = match lovelace {
        Some(asset) => csl::utils::BigNum::from_str(&asset.quantity).unwrap(),
        None => csl::utils::BigNum::from_str("0").unwrap(),
    };

    let mut value = csl::utils::Value::new(&lovelace_asset);

    if assets.len() > 1 || lovelace.is_none() {
        value.set_multiasset(&multi_asset);
    }
    value
}

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

pub fn get_v2_script_hash(script: &str) -> String {
    csl::plutus::PlutusScript::from_hex_with_version(
        script,
        &csl::plutus::Language::new_plutus_v2(),
    )
    .unwrap()
    .hash()
    .to_hex()
}

pub fn address_bech32_to_obj(bech32: &str) {}

// export const addrBech32ToObj = <T>(bech32: string): T => {
//     const hexAddress = csl.Address.from_bech32(bech32).to_hex();
//     const cslAddress = csl.Address.from_hex(hexAddress);
//     const json = JSON.parse(csl.PlutusData.from_address(cslAddress).to_json(1));
//     return json;
// };
