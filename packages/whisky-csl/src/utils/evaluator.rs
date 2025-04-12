use super::phase_two::{eval_phase_two, PhaseTwoEvalResult};
use crate::*;
use cardano_serialization_lib::{self as csl};
use pallas_codec::utils::NonEmptyKeyValuePairs;
use pallas_codec::utils::{Bytes, CborWrap, PositiveCoin};
use pallas_primitives::conway::{Redeemer, RedeemerTag as PRedeemerTag};
use pallas_primitives::{
    conway::{
        AssetName, Coin, CostModels, DatumOption, PlutusData, PolicyId,
        PostAlonzoTransactionOutput, ScriptRef, TransactionOutput, Value,
    },
    Fragment,
};
use pallas_traverse::{Era, MultiEraTx};
use std::collections::HashMap;
use uplc::tx::SlotConfig;
use uplc::{tx::error::Error as UplcError, tx::ResolvedInput, Hash, TransactionInput};
use whisky_common::*;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonSlotConfig {
    pub slot_length: u32,
    pub zero_slot: u64,
    pub zero_time: u64,
}

pub fn evaluate_tx_scripts(
    tx_hex: &str,
    inputs: &[UTxO],
    additional_txs: &[String],
    network: &Network,
    slot_config: &SlotConfig,
) -> Result<Vec<EvalResult>, WError> {
    let tx_bytes = hex::decode(tx_hex).expect("Invalid tx hex");
    let mtx = MultiEraTx::decode_for_era(Era::Conway, &tx_bytes);
    let tx = match mtx {
        Ok(MultiEraTx::Conway(tx)) => tx.into_owned(),
        Ok(_) => {
            return Err(WError::new(
                "evaluate_tx_scripts - Invalid Tx Era",
                "Expected Conway era transaction",
            ))
        }
        Err(err) => {
            return Err(WError::new(
                "evaluate_tx_scripts - decode_for_era",
                &format!("{:?}", err),
            ))
        }
    };

    let tx_outs: Vec<UTxO> = additional_txs
        .iter()
        .flat_map(|tx| {
            let parsed_tx = TxParser::new(tx).unwrap();
            println!(
                "txout: {:?}",
                &parsed_tx.get_tx_outs_utxo().unwrap().clone()
            );
            println!("txout_cbor: {:?}", &parsed_tx.get_tx_outs_cbor().clone());
            parsed_tx.get_tx_outs_utxo().unwrap() // TODO: err handling
        })
        .collect();

    // combine inputs and tx_outs
    let all_inputs: Vec<UTxO> = inputs.iter().chain(tx_outs.iter()).cloned().collect();

    eval_phase_two(
        &tx,
        &to_pallas_utxos(&all_inputs)?,
        Some(&get_cost_mdls(network)?),
        slot_config,
    )
    .map_err(|err| {
        WError::new(
            "evaluate_tx_scripts",
            &format!("Error occurred during evaluation: {}", err),
        )
    })
    .map(|reds| reds.into_iter().map(map_eval_result).collect())
}

pub fn map_eval_result(result: PhaseTwoEvalResult) -> EvalResult {
    match result {
        PhaseTwoEvalResult::Success(redeemer) => {
            EvalResult::Success(map_redeemer_to_action(redeemer))
        }
        PhaseTwoEvalResult::Error(redeemer, err) => {
            EvalResult::Error(map_error_to_eval_error(err, redeemer))
        }
    }
}

pub fn map_error_to_eval_error(err: UplcError, original_redeemer: Redeemer) -> EvalError {
    match err {
        UplcError::Machine(err, budget, logs) => EvalError {
            index: original_redeemer.index,
            budget: Budget {
                mem: budget.mem as u64,
                steps: budget.cpu as u64,
            },
            tag: map_redeemer_tag(&original_redeemer.tag),
            error_message: format!("{}", err),
            logs,
        },
        UplcError::RedeemerError { err, .. } => match *err {
            UplcError::Machine(err, budget, logs) => EvalError {
                index: original_redeemer.index,
                budget: Budget {
                    mem: budget.mem as u64,
                    steps: budget.cpu as u64,
                },
                tag: map_redeemer_tag(&original_redeemer.tag),
                error_message: format!("{}", err),
                logs,
            },
            _ => EvalError {
                index: original_redeemer.index,
                budget: Budget { mem: 0, steps: 0 },
                tag: map_redeemer_tag(&original_redeemer.tag),
                error_message: format!("{}", err),
                logs: vec![],
            },
        },
        _ => EvalError {
            index: original_redeemer.index,
            budget: Budget { mem: 0, steps: 0 },
            tag: map_redeemer_tag(&original_redeemer.tag),
            error_message: format!("{}", err),
            logs: vec![],
        },
    }
}

pub fn map_redeemer_to_action(redeemer: Redeemer) -> Action {
    Action {
        index: redeemer.index,
        budget: Budget {
            mem: redeemer.ex_units.mem,
            steps: redeemer.ex_units.steps,
        },
        tag: map_redeemer_tag(&redeemer.tag),
    }
}

pub fn map_redeemer_tag(tag: &PRedeemerTag) -> RedeemerTag {
    match tag {
        PRedeemerTag::Spend => RedeemerTag::Spend,
        PRedeemerTag::Mint => RedeemerTag::Mint,
        PRedeemerTag::Cert => RedeemerTag::Cert,
        PRedeemerTag::Reward => RedeemerTag::Reward,
        PRedeemerTag::Vote => RedeemerTag::Vote,
        PRedeemerTag::Propose => RedeemerTag::Propose,
    }
}

pub fn get_cost_mdls(network: &Network) -> Result<CostModels, WError> {
    let cost_model_list = get_cost_models_from_network(network);
    if cost_model_list.len() < 3 {
        return Err(WError::new(
            "get_cost_mdls",
            "Cost models have to contain at least PlutusV1, PlutusV2, and PlutusV3 costs",
        ));
    };
    Ok(CostModels {
        plutus_v1: Some(cost_model_list[0].clone()),
        plutus_v2: Some(cost_model_list[1].clone()),
        plutus_v3: Some(cost_model_list[2].clone()),
    })
}

pub fn to_pallas_utxos(utxos: &Vec<UTxO>) -> Result<Vec<ResolvedInput>, WError> {
    let mut resolved_inputs = Vec::new();
    for utxo in utxos {
        let tx_hash: [u8; 32] = hex::decode(&utxo.input.tx_hash)
            .map_err(|err| {
                WError::new(
                    "to_pallas_utxos",
                    &format!("Invalid tx hash found: {}", err),
                )
            })?
            .try_into()
            .map_err(|_e| WError::new("to_pallas_utxos", "Invalid tx hash length found"))?;

        let resolved_input = ResolvedInput {
            input: TransactionInput {
                transaction_id: Hash::from(tx_hash),
                index: utxo.input.output_index.into(),
            },
            output: TransactionOutput::PostAlonzo(PostAlonzoTransactionOutput {
                address: Bytes::from(
                    csl::Address::from_bech32(&utxo.output.address)
                        .map_err(|err| {
                            WError::new(
                                "to_pallas_utxos",
                                &format!("Invalid address found: {:?}", err),
                            )
                        })?
                        .to_bytes(),
                ),
                value: to_pallas_value(&utxo.output.amount)
                    .map_err(WError::add_err_trace("to_pallas_utxos"))?,
                datum_option: to_pallas_datum(&utxo.output)
                    .map_err(WError::add_err_trace("to_pallas_utxos"))?,
                script_ref: to_pallas_script_ref(&utxo.output.script_ref)
                    .map_err(WError::add_err_trace("to_pallas_utxos"))?,
            }),
        };
        resolved_inputs.push(resolved_input);
    }
    Ok(resolved_inputs)
}

pub fn to_pallas_script_ref(
    script_ref: &Option<String>,
) -> Result<Option<CborWrap<ScriptRef>>, WError> {
    if let Some(script_ref) = script_ref {
        let script_bytes = hex::decode(script_ref).map_err(WError::from_err(
            "to_pallas_script_ref - Invalid script ref hex",
        ))?;

        let pallas_script = ScriptRef::decode_fragment(&script_bytes).map_err(WError::from_err(
            "to_pallas_script_ref - Invalid script ref bytes",
        ))?;

        Ok(Some(CborWrap(pallas_script)))
    } else {
        Ok(None)
    }
}

pub fn to_pallas_datum(utxo_output: &UtxoOutput) -> Result<Option<DatumOption>, WError> {
    if let Some(inline_datum) = &utxo_output.plutus_data {
        //hex to bytes
        let plutus_data_bytes = hex::decode(inline_datum).map_err(WError::from_err(
            "to_pallas_datum - Invalid plutus data hex",
        ))?;
        let datum = CborWrap(PlutusData::decode_fragment(&plutus_data_bytes).map_err(
            WError::from_err("to_pallas_datum - Invalid plutus data bytes"),
        )?);
        Ok(Some(DatumOption::Data(datum)))
    } else if let Some(datum_hash) = &utxo_output.data_hash {
        let datum_hash_bytes: [u8; 32] = hex::decode(datum_hash)
            .map_err(WError::from_err("to_pallas_datum - Invalid datum hash hex"))?
            .try_into()
            .map_err(|_e| {
                WError::new("to_pallas_datum", "Invalid byte length of datum hash found")
            })?;
        Ok(Some(DatumOption::Hash(Hash::from(datum_hash_bytes))))
    } else {
        Ok(None)
    }
}

pub fn to_pallas_value(assets: &Vec<Asset>) -> Result<Value, WError> {
    if assets.len() == 1 {
        match assets[0].unit().as_str() {
            "lovelace" => Ok(Value::Coin(assets[0].quantity().parse::<u64>().unwrap())),
            _ => Err(WError::new("to_pallas_value", "Invalid value")),
        }
    } else {
        to_pallas_multi_asset_value(assets)
    }
}

pub fn to_pallas_multi_asset_value(assets: &Vec<Asset>) -> Result<Value, WError> {
    let mut coins: Coin = 0;
    let mut asset_mapping: HashMap<String, Vec<(String, String)>> = HashMap::new();
    for asset in assets {
        if asset.unit() == "lovelace" || asset.unit().is_empty() {
            coins = asset.quantity().parse::<u64>().unwrap();
        } else {
            let asset_unit = asset.unit();
            let (policy_id, asset_name) = asset_unit.split_at(56);
            asset_mapping
                .entry(policy_id.to_string())
                .or_default()
                .push((asset_name.to_string(), asset.quantity().clone()))
        }
    }

    let mut multi_asset = Vec::new();
    for (policy_id, asset_list) in &asset_mapping {
        let policy_id_bytes: [u8; 28] = hex::decode(policy_id)
            .map_err(WError::from_err(
                "to_pallas_multi_asset_value - Invalid policy id hex",
            ))?
            .try_into()
            .map_err(|_e| {
                WError::new(
                    "to_pallas_multi_asset_vale",
                    "Invalid length policy id found",
                )
            })?;

        let policy_id = PolicyId::from(policy_id_bytes);
        let mut mapped_assets = Vec::new();
        for asset in asset_list {
            let (asset_name, asset_quantity) = asset;
            let asset_name_bytes = AssetName::from(hex::decode(asset_name).map_err(
                WError::from_err("to_pallas_multi_asset_value - Invalid asset name hex"),
            )?);
            mapped_assets.push((
                asset_name_bytes,
                PositiveCoin::try_from(asset_quantity.parse::<u64>().unwrap()).unwrap(),
            ));
        }
        multi_asset.push((policy_id, NonEmptyKeyValuePairs::Def(mapped_assets)));
    }
    let pallas_multi_asset = pallas_codec::utils::NonEmptyKeyValuePairs::Def(multi_asset);
    Ok(Value::Multiasset(coins, pallas_multi_asset))
}
