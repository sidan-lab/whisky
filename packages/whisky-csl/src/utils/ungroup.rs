use cardano_serialization_lib::{self as csl};
use hex::FromHex;
use whisky_common::*;

pub fn to_bignum(val: u64) -> Result<csl::BigNum, WError> {
    csl::BigNum::from_str(&val.to_string()).map_err(WError::from_err("to_bignum - invalid value"))
}

pub fn build_tx_builder(params: Option<Protocol>) -> Result<csl::TransactionBuilder, WError> {
    let protocol_params = params.unwrap_or_default();

    let cfg =
        csl::TransactionBuilderConfigBuilder::new()
            .fee_algo(&csl::LinearFee::new(
                &to_bignum(protocol_params.min_fee_a).map_err(WError::add_err_trace(
                    "build_tx_builder - invalid min fee a",
                ))?,
                &to_bignum(protocol_params.min_fee_b).map_err(WError::add_err_trace(
                    "build_tx_builder - invalid min fee b",
                ))?,
            ))
            .pool_deposit(&to_bignum(protocol_params.pool_deposit).map_err(
                WError::add_err_trace("build_tx_builder - invalid pool deposit"),
            )?)
            .key_deposit(
                &to_bignum(protocol_params.key_deposit).map_err(WError::add_err_trace(
                    "build_tx_builder - invalid key deposit",
                ))?,
            )
            .max_value_size(protocol_params.max_val_size)
            .max_tx_size(protocol_params.max_tx_size)
            .coins_per_utxo_byte(&to_bignum(protocol_params.coins_per_utxo_size).map_err(
                WError::add_err_trace("build_tx_builder - invalid coins per utxo byte"),
            )?)
            .ex_unit_prices(&csl::ExUnitPrices::new(
                &csl::UnitInterval::new(
                    &to_bignum((protocol_params.price_mem * 10000.0) as u64).map_err(
                        WError::add_err_trace("build_tx_builder - invalid price mem"),
                    )?,
                    &to_bignum(10000).unwrap(),
                ),
                &csl::UnitInterval::new(
                    &to_bignum((protocol_params.price_step * 10000000.0) as u64).map_err(
                        WError::add_err_trace("build_tx_builder - invalid price step"),
                    )?,
                    &to_bignum(10000000).unwrap(),
                ),
            ))
            .ref_script_coins_per_byte(&csl::UnitInterval::new(
                &to_bignum(protocol_params.min_fee_ref_script_cost_per_byte).map_err(
                    WError::add_err_trace(
                        "build_tx_builder - invalid min fee ref script cost per byte",
                    ),
                )?,
                &to_bignum(1).map_err(WError::add_err_trace(
                    "build_tx_builder - invalid min fee ref script cost per byte",
                ))?,
            ))
            .deduplicate_explicit_ref_inputs_with_regular_inputs(true)
            .build()
            .unwrap();
    Ok(csl::TransactionBuilder::new(&cfg))
}

pub fn to_value(assets: &Vec<Asset>) -> Result<csl::Value, WError> {
    let lovelace = assets.iter().find(|asset| asset.unit() == "lovelace");
    let mut multi_asset = csl::MultiAsset::new();

    for asset in assets {
        if asset.unit() == "lovelace" {
            continue;
        }
        let name_bytes = Vec::<u8>::from_hex(&asset.unit()[56..])
            .map_err(WError::from_err("to_value - invalid asset name hex"))?;
        multi_asset.set_asset(
            &csl::ScriptHash::from_hex(&asset.unit()[0..56])
                .map_err(WError::from_err("to_value - invalid asset script hash"))?,
            &csl::AssetName::new(name_bytes)
                .map_err(WError::from_err("to_value - invalid asset name hex"))?,
            &csl::BigNum::from_str(&asset.quantity().to_string())
                .map_err(WError::from_err("to_value - invalid asset quantity"))?,
        );
    }

    let lovelace_asset = match lovelace {
        Some(asset) => csl::BigNum::from_str(&asset.quantity()).map_err(WError::from_err(
            "to_value - invalid lovelace asset quantity",
        ))?,
        None => csl::BigNum::from_str("0").unwrap(),
    };

    let mut value = csl::Value::new(&lovelace_asset);

    if assets.len() > 1 || lovelace.is_none() {
        value.set_multiasset(&multi_asset);
    }
    Ok(value)
}
