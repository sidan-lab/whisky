use cardano_serialization_lib as csl;
use pallas::ledger::primitives::alonzo::Redeemer;
use pallas::ledger::primitives::conway::PlutusV2Script;
use std::collections::HashMap;
use uplc::tx::SlotConfig;
use uplc::Fragment;

use crate::model::{Asset, UTxO, UtxoOutput};
use cardano_serialization_lib::address::Address;
use pallas::codec::utils::{Bytes, CborWrap, KeyValuePairs};
use pallas::ledger::primitives::babbage::{
    AssetName, Coin, CostMdls, DatumOption, PlutusData, PolicyId, PostAlonzoTransactionOutput,
    PseudoScript, ScriptRef, TransactionOutput, Value,
};
use pallas::ledger::traverse::{Era, MultiEraTx};
use uplc::{
    tx::{eval_phase_two, ResolvedInput},
    Hash, TransactionInput,
};

pub fn tx_eval(tx_hex: &str, inputs: &Vec<UTxO>) -> Result<Vec<Redeemer>, String> {
    let tx_bytes = hex::decode(tx_hex).expect("Invalid tx hex");
    let mtx = MultiEraTx::decode_for_era(Era::Babbage, &tx_bytes);
    let tx = match mtx {
        Ok(MultiEraTx::Babbage(tx)) => tx.into_owned(),
        _ => return Err("Invalid Tx Era".to_string()),
    };

    eval_phase_two(
        &tx,
        &to_pallas_utxos(inputs)?,
        Some(&get_cost_mdls()),
        None,
        &SlotConfig::default(),
        false,
        |_r| (),
    )
    .map_err(|err| format!("Error occurred during evaluation: {}", err))
}

fn get_cost_mdls() -> CostMdls {
    let plutus_v1_cost_model: Vec<i64> = vec![
        205665, 812, 1, 1, 1000, 571, 0, 1, 1000, 24177, 4, 1, 1000, 32, 117366, 10475, 4, 23000,
        100, 23000, 100, 23000, 100, 23000, 100, 23000, 100, 23000, 100, 100, 100, 23000, 100,
        19537, 32, 175354, 32, 46417, 4, 221973, 511, 0, 1, 89141, 32, 497525, 14068, 4, 2, 196500,
        453240, 220, 0, 1, 1, 1000, 28662, 4, 2, 245000, 216773, 62, 1, 1060367, 12586, 1, 208512,
        421, 1, 187000, 1000, 52998, 1, 80436, 32, 43249, 32, 1000, 32, 80556, 1, 57667, 4, 1000,
        10, 197145, 156, 1, 197145, 156, 1, 204924, 473, 1, 208896, 511, 1, 52467, 32, 64832, 32,
        65493, 32, 22558, 32, 16563, 32, 76511, 32, 196500, 453240, 220, 0, 1, 1, 69522, 11687, 0,
        1, 60091, 32, 196500, 453240, 220, 0, 1, 1, 196500, 453240, 220, 0, 1, 1, 806990, 30482, 4,
        1927926, 82523, 4, 265318, 0, 4, 0, 85931, 32, 205665, 812, 1, 1, 41182, 32, 212342, 32,
        31220, 32, 32696, 32, 43357, 32, 32247, 32, 38314, 32, 9462713, 1021, 10,
    ];

    let plutus_v2_cost_model: Vec<i64> = vec![
        205665, 812, 1, 1, 1000, 571, 0, 1, 1000, 24177, 4, 1, 1000, 32, 117366, 10475, 4, 23000,
        100, 23000, 100, 23000, 100, 23000, 100, 23000, 100, 23000, 100, 100, 100, 23000, 100,
        19537, 32, 175354, 32, 46417, 4, 221973, 511, 0, 1, 89141, 32, 497525, 14068, 4, 2, 196500,
        453240, 220, 0, 1, 1, 1000, 28662, 4, 2, 245000, 216773, 62, 1, 1060367, 12586, 1, 208512,
        421, 1, 187000, 1000, 52998, 1, 80436, 32, 43249, 32, 1000, 32, 80556, 1, 57667, 4, 1000,
        10, 197145, 156, 1, 197145, 156, 1, 204924, 473, 1, 208896, 511, 1, 52467, 32, 64832, 32,
        65493, 32, 22558, 32, 16563, 32, 76511, 32, 196500, 453240, 220, 0, 1, 1, 69522, 11687, 0,
        1, 60091, 32, 196500, 453240, 220, 0, 1, 1, 196500, 453240, 220, 0, 1, 1, 1159724, 392670,
        0, 2, 806990, 30482, 4, 1927926, 82523, 4, 265318, 0, 4, 0, 85931, 32, 205665, 812, 1, 1,
        41182, 32, 212342, 32, 31220, 32, 32696, 32, 43357, 32, 32247, 32, 38314, 32, 20000000000,
        20000000000, 9462713, 1021, 10, 20000000000, 0, 20000000000,
    ];

    CostMdls {
        plutus_v1: Some(plutus_v1_cost_model),
        plutus_v2: Some(plutus_v2_cost_model),
    }
}

fn to_pallas_utxos(utxos: &Vec<UTxO>) -> Result<Vec<ResolvedInput>, String> {
    let mut resolved_inputs = Vec::new();
    for utxo in utxos {
        let tx_hash: [u8; 32] = hex::decode(&utxo.input.tx_hash)
            .map_err(|err| format!("Invalid tx hash found: {}", err))?
            .try_into()
            .map_err(|_e| format!("Invalid tx hash length found"))?;

        let resolved_input = ResolvedInput {
            input: TransactionInput {
                transaction_id: Hash::from(tx_hash),
                index: utxo.input.output_index.try_into().unwrap(),
            },
            output: TransactionOutput::PostAlonzo(PostAlonzoTransactionOutput {
                address: Bytes::from(
                    Address::from_bech32(&utxo.output.address)
                        .unwrap()
                        .to_bytes(),
                ),
                value: to_pallas_value(&utxo.output.amount)?,
                datum_option: to_pallas_datum(&utxo.output)?,
                script_ref: to_pallas_script_ref(&utxo.output)?,
            }),
        };
        resolved_inputs.push(resolved_input);
    }
    Ok(resolved_inputs)
}

// TODO: handle native and plutusV1 scripts
fn to_pallas_script_ref(utxo_output: &UtxoOutput) -> Result<Option<CborWrap<ScriptRef>>, String> {
    if let Some(script) = &utxo_output.script_ref {
        let script_bytes =
            hex::decode(script).map_err(|err| format!("Invalid script hex found: {}", err))?;
        Ok(Some(CborWrap(PseudoScript::PlutusV2Script(
            PlutusV2Script(script_bytes.into()),
        ))))
    } else {
        Ok(None)
    }
}

fn to_pallas_datum(utxo_output: &UtxoOutput) -> Result<Option<DatumOption>, String> {
    if let Some(inline_datum) = &utxo_output.plutus_data {
        let csl_plutus_data = csl::plutus::PlutusData::from_json(
            &inline_datum,
            csl::plutus::PlutusDatumSchema::DetailedSchema,
        )
        .map_err(|err| format!("Invalid plutus data found: {}", err))?;

        let plutus_data_bytes = csl_plutus_data.to_bytes();
        let datum = CborWrap(
            PlutusData::decode_fragment(&plutus_data_bytes)
                .map_err(|err| format!("Invalid plutus data found"))?,
        );
        Ok(Some(DatumOption::Data(datum)))
    } else if let Some(datum_hash) = &utxo_output.data_hash {
        let datum_hash_bytes: [u8; 32] = hex::decode(datum_hash)
            .map_err(|err| format!("Invalid datum hash found: {}", err))?
            .try_into()
            .map_err(|err| format!("Invalid byte length of datum hash found"))?;
        Ok(Some(DatumOption::Hash(Hash::from((datum_hash_bytes)))))
    } else {
        Ok(None)
    }
}

fn to_pallas_value(assets: &Vec<Asset>) -> Result<Value, String> {
    if assets.len() == 1 {
        match assets[0].unit.as_str() {
            "lovelace" => Ok(Value::Coin(assets[0].quantity.parse::<u64>().unwrap())),
            _ => Err("Invalid value".to_string()),
        }
    } else {
        to_pallas_multi_asset_value(assets)
    }
}

fn to_pallas_multi_asset_value(assets: &Vec<Asset>) -> Result<Value, String> {
    let mut coins: Coin = 0;
    let mut asset_mapping: HashMap<String, Vec<(String, String)>> = HashMap::new();
    for asset in assets {
        if asset.unit == "lovelace" || asset.unit == "" {
            coins = asset.quantity.parse::<u64>().unwrap();
        } else {
            let (policy_id, asset_name) = asset.unit.split_at(56);
            asset_mapping
                .entry(policy_id.to_string())
                .or_insert_with(Vec::new)
                .push((asset_name.to_string(), asset.quantity.clone()))
        }
    }

    let mut multi_asset = Vec::new();
    for (policy_id, asset_list) in &asset_mapping {
        let policy_id_bytes: [u8; 28] = hex::decode(policy_id)
            .map_err(|err| format!("Invalid policy id found: {}", err))?
            .try_into()
            .map_err(|_e| format!("Invalid length policy id found"))?;

        let policy_id = PolicyId::from(policy_id_bytes);
        let mut mapped_assets = Vec::new();
        for asset in asset_list {
            let (asset_name, asset_quantity) = asset;
            let asset_name_bytes = AssetName::from(
                hex::decode(asset_name)
                    .map_err(|err| format!("Invalid asset name found: {}", err))?,
            );
            mapped_assets.push((asset_name_bytes, asset_quantity.parse::<u64>().unwrap()));
        }
        multi_asset.push((policy_id, KeyValuePairs::Def(mapped_assets)));
    }
    let pallas_multi_asset = KeyValuePairs::Def(multi_asset);
    return Ok(Value::Multiasset(coins, pallas_multi_asset));
}
