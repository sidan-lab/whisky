use std::collections::HashMap;

use whisky_common::models::{Asset, UTxO};

pub fn utxos_to_assets(utxos: &[UTxO]) -> HashMap<String, String> {
    let mut balance: HashMap<String, String> = HashMap::new();

    for utxo in utxos {
        for asset in &utxo.output.amount {
            let (policy, name) = Asset::unit_to_tuple(&asset.unit());
            let quantity = asset.quantity().clone();
            let asset_key = if policy.is_empty() {
                name
            } else {
                format!("{}/{}", policy, name)
            };

            balance
                .entry(asset_key)
                .and_modify(|v| {
                    *v = (Asset::new(asset.unit().clone(), v.clone()).quantity_i128()
                        + Asset::new(asset.unit().clone(), quantity.clone()).quantity_i128())
                    .to_string()
                })
                .or_insert(Asset::new(asset.unit().clone(), quantity).quantity());
        }
    }

    balance
}
