use crate::model::builder::*;
use std::collections::{HashMap, HashSet};

pub fn select_utxos(
    inputs: Vec<UTxO>,
    required_assets: HashMap<String, String>,
    threshold: String,
) -> Vec<UTxO> {
    let mut total_required_assets = required_assets.clone();
    let lovelace_value = total_required_assets
        .get("lovelace")
        .unwrap()
        .parse::<i64>()
        .unwrap();
    let threshold_parsed = threshold.parse::<i64>().unwrap();
    let sum = lovelace_value + threshold_parsed;
    total_required_assets.insert("lovelace".to_string(), sum.to_string());

    // Classify the utxos
    let mut only_lovelace: Vec<usize> = vec![];
    let mut singleton: Vec<usize> = vec![];
    let mut pair: Vec<usize> = vec![];
    let mut rest: Vec<usize> = vec![];
    for (index, utxo) in inputs.iter().enumerate() {
        match utxo.output.amount.len() {
            1 => only_lovelace.push(index),
            2 => singleton.push(index),
            3 => pair.push(index),
            _ => rest.push(index),
        }
    }

    let mut used_utxos: HashSet<usize> = HashSet::new();
    let mut use_utxo = |index: usize, total_required_assets: &mut HashMap<String, String>| {
        let utxo = inputs[index].clone();
        for asset in utxo.output.amount {
            let required_asset_value = total_required_assets
                .get(&asset.unit)
                .unwrap_or(&"0".to_string())
                .parse::<i64>()
                .unwrap();
            let utxo_asset_value = asset.quantity.parse::<i64>().unwrap();
            let final_required_asset_value = required_asset_value - utxo_asset_value;
            total_required_assets.insert(asset.unit, final_required_asset_value.to_string());
            used_utxos.insert(index);
        }
    };

    let mut process_list =
        |list: Vec<usize>, unit: String, total_required_assets: &mut HashMap<String, String>| {
            for index in list {
                let required_asset_value = total_required_assets
                    .get(&unit)
                    .unwrap_or(&"0".to_string())
                    .parse::<i64>()
                    .unwrap();
                if required_asset_value <= 0 {
                    return;
                }
                let utxo = inputs[index].clone();
                for asset in utxo.output.amount {
                    if asset.unit == unit {
                        use_utxo(index, total_required_assets);
                        break;
                    }
                }
            }
        };

    let required_units: Vec<String> = total_required_assets.keys().cloned().collect();

    for unit in required_units.clone() {
        if unit != *"lovelace"
            && total_required_assets
                .get(&unit)
                .unwrap()
                .parse::<i64>()
                .unwrap()
                > 0
        {
            process_list(singleton.clone(), unit.clone(), &mut total_required_assets);
            process_list(pair.clone(), unit.clone(), &mut total_required_assets);
            process_list(rest.clone(), unit.clone(), &mut total_required_assets);
        }
    }

    process_list(
        only_lovelace.clone(),
        "lovelace".to_string(),
        &mut total_required_assets,
    );

    process_list(
        singleton.clone(),
        "lovelace".to_string(),
        &mut total_required_assets,
    );
    process_list(
        pair.clone(),
        "lovelace".to_string(),
        &mut total_required_assets,
    );
    process_list(
        rest.clone(),
        "lovelace".to_string(),
        &mut total_required_assets,
    );

    for unit in required_units.clone() {
        if total_required_assets
            .get(&unit)
            .unwrap()
            .parse::<i64>()
            .unwrap()
            > 0
        {
            panic!("Selection failed");
        }
    }

    let mut selected_utxos: Vec<UTxO> = vec![];
    for index in used_utxos.iter() {
        selected_utxos.push(inputs[*index].clone());
    }

    selected_utxos
}

#[test]
fn test_basic_selection() {
    let utxo_list = vec![UTxO {
        input: UtxoInput {
            output_index: 0,
            tx_hash: "test".to_string(),
        },
        output: UtxoOutput {
            address: "test".to_string(),
            amount: vec![Asset {
                unit: "lovelace".to_string(),
                quantity: "10000000".to_string(),
            }],
            data_hash: None,
            plutus_data: None,
            script_ref: None,
            script_hash: None,
        },
    }];

    let mut required_assets: HashMap<String, String> = HashMap::new();
    required_assets.insert("lovelace".to_string(), "5000000".to_string());
    let selected_list = select_utxos(utxo_list.clone(), required_assets, "5000000".to_string());
    assert_eq!(utxo_list, selected_list);
}
