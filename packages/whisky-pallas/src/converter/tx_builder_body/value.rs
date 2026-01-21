use whisky_common::{Asset, WError};

use crate::wrapper::transaction_body::{MultiassetPositiveCoin, Value};

pub fn convert_value(asset_vec: &Vec<Asset>) -> Result<Value, WError> {
    let mut lovelace: u64 = 0;
    let mut multiasset: Vec<(String, Vec<(String, u64)>)> = Vec::new();
    for asset in asset_vec {
        if asset.unit() == "lovelace".to_string() || asset.unit() == "".to_string() {
            lovelace = asset.quantity().parse::<u64>().map_err(|_| {
                WError::new(
                    "WhiskyPallas - Converting value:",
                    "Invalid lovelace quantity format",
                )
            })?;
        } else {
            let policy = asset.policy();
            let asset_name = asset.name();
            let quantity = asset.quantity().parse::<u64>().map_err(|_| {
                WError::new(
                    "WhiskyPallas - Converting value:",
                    &format!("Invalid quantity format for asset: {}", asset_name),
                )
            })?;
            multiasset.push((policy, vec![(asset_name, quantity)]));
        }
    }
    if multiasset.is_empty() {
        Ok(Value::new(lovelace, None))
    } else {
        Ok(Value::new(
            lovelace,
            Some(MultiassetPositiveCoin::new(multiasset)?),
        ))
    }
}

pub fn value_to_asset_vec(value: &Value) -> Result<Vec<Asset>, WError> {
    let mut assets: Vec<Asset> = Vec::new();
    match &value.inner {
        pallas::ledger::primitives::conway::Value::Coin(coin) => {
            assets.push(Asset::new_from_str("lovelace", &coin.to_string()));
        }
        pallas::ledger::primitives::conway::Value::Multiasset(coin, btree_map) => {
            assets.push(Asset::new_from_str("lovelace", &coin.to_string()));
            for (policy_id, asset_map) in btree_map {
                for (asset_name, quantity) in asset_map {
                    let concated_name =
                        format!("{}{}", policy_id.to_string(), asset_name.to_string());
                    assets.push(Asset::new_from_str(
                        &concated_name,
                        &u64::from(quantity).to_string(),
                    ));
                }
            }
        }
    }
    Ok(assets)
}
