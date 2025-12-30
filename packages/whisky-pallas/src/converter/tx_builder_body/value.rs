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
            Some(MultiassetPositiveCoin::new(multiasset)),
        ))
    }
}
