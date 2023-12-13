use cardano_serialization_lib as csl;

use crate::builder::models::*;
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
    return csl::tx_builder::TransactionBuilder::new(&cfg);
}

pub fn to_value(assets: &Vec<Asset>) -> csl::utils::Value {
    let lovelace = assets.iter().find(|asset| asset.unit == "lovelace");
    let mut multi_asset = csl::MultiAsset::new();

    for asset in assets {
        let mut policy_assets = csl::Assets::new();
        let name_bytes = Vec::<u8>::from_hex(asset.unit[56..].to_string())
            .expect("Failed to parse hex asset name");
        policy_assets.insert(
            &csl::AssetName::new(name_bytes).unwrap(),
            &csl::utils::BigNum::from_str(&asset.quantity.to_string()).unwrap(),
        );

        multi_asset.insert(
            &csl::crypto::ScriptHash::from_hex(&asset.unit[0..56].to_string()).unwrap(),
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
