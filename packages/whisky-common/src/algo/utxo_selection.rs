use crate::{data::Value, errors::WError, models::*};
use std::collections::HashSet;

pub fn select_utxos(
    inputs: &[UTxO],
    required_assets: Value,
    threshold: &str,
) -> Result<Vec<UTxO>, WError> {
    let mut total_required_assets = required_assets.clone();
    total_required_assets.add_asset("lovelace", threshold.parse::<u64>().unwrap());

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

    let mut use_utxo = |index: usize, total_required_assets: &mut Value| {
        let utxo = inputs[index].clone();
        for asset in utxo.output.amount {
            total_required_assets
                .negate_asset(&asset.unit(), asset.quantity().parse::<u64>().unwrap());
            used_utxos.insert(index);
        }
    };

    let mut process_list = |list: Vec<usize>, unit: String, total_required_assets: &mut Value| {
        for index in list {
            let required_asset_value = total_required_assets.get(&unit);
            if required_asset_value == 0 {
                return;
            }
            let utxo = inputs[index].clone();
            for asset in utxo.output.amount {
                if asset.unit() == unit {
                    use_utxo(index, total_required_assets);
                    break;
                }
            }
        }
    };

    let required_units: Vec<String> = total_required_assets.keys();

    for unit in required_units.clone() {
        if unit != *"lovelace" && total_required_assets.get(&unit) > 0 {
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
        if total_required_assets.get(&unit) > 0 {
            println!("Total required assets: {:?}", total_required_assets);
            return Err(WError::new("select_utxos", &format!(
                "{:?} value missing. Reminder, in this selection you required for {:?} extra lovelace as threshold",
                total_required_assets,
                threshold
            )));
        }
    }

    let mut selected_utxos: Vec<UTxO> = vec![];
    for index in used_utxos.iter() {
        selected_utxos.push(inputs[*index].clone());
    }

    Ok(selected_utxos)
}
