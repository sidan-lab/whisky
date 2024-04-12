use cardano_serialization_lib as csl;
use pallas::ledger::primitives::conway::PlutusV2Script;
use std::collections::HashMap;
use uplc::Fragment;

use crate::model::{Asset, UTxO, UtxoOutput};
use cardano_serialization_lib::address::Address;
use pallas::codec::utils::{Bytes, CborWrap, KeyValuePairs};
use pallas::ledger::primitives::babbage::{
    AssetName, Coin, DatumOption, Multiasset, PlutusData, PolicyId, PostAlonzoTransactionOutput,
    PseudoScript, ScriptRef, TransactionOutput, Value,
};
use pallas::ledger::traverse::{Era, MultiEraTx};
use uplc::{
    tx::{eval_phase_two, ResolvedInput},
    Hash, TransactionInput,
};

// pub fn tx_eval(tx_hex: &str, inputs: &Vec<UTxO>) -> Result<&str, String> {
//     let tx_bytes = hex::decode(tx_hex).expect("Invalid tx hex");
//     let mtx = MultiEraTx::decode_for_era(Era::Babbage, &tx_bytes);
//     let tx = match mtx {
//         Ok(MultiEraTx::Babbage(tx)) => tx.into_owned(),
//         _ => return Err("Invalid Tx Era".to_string()),
//     };

//     eval_phase_two(
//         &tx,
//         utxos,
//         cost_mdls,
//         initial_budget,
//         slot_config,
//         true,
//         with_redeemer,
//     );
//     Ok("")
// }

fn to_pallas_utxo(utxos: &Vec<UTxO>) -> Result<Vec<ResolvedInput>, String> {
    let mut resolved_inputs = Vec::new();
    for utxo in utxos {
        let mut resolved_input: ResolvedInput;
        let resolved_input = ResolvedInput {
            input: TransactionInput {
                transaction_id: Hash::from(
                    hex::decode(utxo.input.tx_hash).unwrap().try_into().unwrap(),
                ),
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
