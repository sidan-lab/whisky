use crate::csl;
use hex::FromHex;

use crate::model::*;

pub fn to_bignum(val: u64) -> csl::BigNum {
    csl::BigNum::from_str(&val.to_string()).unwrap()
}

// TODO: update ref script coins per byte when value is confirmed
pub fn build_tx_builder() -> csl::TransactionBuilder {
    let cfg = csl::TransactionBuilderConfigBuilder::new()
        .fee_algo(&csl::LinearFee::new(&to_bignum(44), &to_bignum(155381)))
        .pool_deposit(&to_bignum(500000000))
        .key_deposit(&to_bignum(2000000))
        .max_value_size(5000)
        .max_tx_size(16384)
        .coins_per_utxo_byte(&to_bignum(4310))
        .ex_unit_prices(&csl::ExUnitPrices::new(
            &csl::UnitInterval::new(&to_bignum(577), &to_bignum(10000)),
            &csl::UnitInterval::new(&to_bignum(721), &to_bignum(10000000)),
        ))
        .ref_script_coins_per_byte(&csl::UnitInterval::new(&to_bignum(0), &to_bignum(10000)))
        .build()
        .unwrap();
    csl::TransactionBuilder::new(&cfg)
}

pub fn to_value(assets: &Vec<Asset>) -> csl::Value {
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
            &csl::BigNum::from_str(&asset.quantity.to_string()).unwrap(),
        );

        multi_asset.insert(
            &csl::ScriptHash::from_hex(&asset.unit[0..56]).unwrap(),
            &policy_assets,
        );
    }

    let lovelace_asset = match lovelace {
        Some(asset) => csl::BigNum::from_str(&asset.quantity).unwrap(),
        None => csl::BigNum::from_str("0").unwrap(),
    };

    let mut value = csl::Value::new(&lovelace_asset);

    if assets.len() > 1 || lovelace.is_none() {
        value.set_multiasset(&multi_asset);
    }
    value
}
