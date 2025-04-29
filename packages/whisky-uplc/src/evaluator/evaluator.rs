use super::phase_two::{eval_phase_two, PhaseTwoEvalResult};
use crate::models::to_uplc_utxos;
use pallas_primitives::conway::CostModels;
use pallas_primitives::conway::{Redeemer, RedeemerTag as PRedeemerTag};
use pallas_traverse::{Era, MultiEraTx};
use uplc::tx::error::Error as UplcError;
use uplc::tx::SlotConfig;
use whisky_common::*;
use whisky_csl::TxParser;

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
        &to_uplc_utxos(&all_inputs)?,
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
            logs: logs.iter().map(|log| log.to_string()).collect(),
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
                logs: logs.iter().map(|log| log.to_string()).collect(),
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
