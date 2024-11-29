use pallas_primitives::conway::{CostMdls, MintedTx, Redeemer};
use uplc::machine::cost_model::ExBudget;
use uplc::tx::error::Error;
use uplc::tx::{eval, DataLookupTable, ResolvedInput, SlotConfig};

pub enum PhaseTwoEvalResult {
    Success(Redeemer),
    Error(Redeemer, Error),
}

pub fn eval_phase_two(
    tx: &MintedTx,
    utxos: &[ResolvedInput],
    cost_mdls: Option<&CostMdls>,
    initial_budget: Option<&ExBudget>,
    slot_config: &SlotConfig,
) -> Result<Vec<PhaseTwoEvalResult>, Error> {
    let redeemers = tx.transaction_witness_set.redeemer.as_ref();

    //TODO: remove it after uplc will be updated to newer cost model
    let cost_mdls = trim_cost_modes(cost_mdls);

    let lookup_table = DataLookupTable::from_transaction(tx, utxos);

    match redeemers {
        Some(rs) => {
            let mut results = Vec::new();
            let mut remaining_budget = *initial_budget.unwrap_or(&ExBudget::default());

            for (redeemer_key, redeemer_value) in rs.iter() {
                let redeemer = Redeemer {
                    tag: redeemer_key.tag,
                    index: redeemer_key.index,
                    data: redeemer_value.data.clone(),
                    ex_units: redeemer_value.ex_units,
                };

                let eval_result = eval::eval_redeemer(
                    tx,
                    utxos,
                    slot_config,
                    &redeemer,
                    &lookup_table,
                    cost_mdls.as_ref(),
                    &remaining_budget,
                );

                // The subtraction is safe here as ex units counting is done during evaluation.
                // Redeemer would fail already if budget was negative.
                remaining_budget.cpu -= redeemer.ex_units.steps as i64;
                remaining_budget.mem -= redeemer.ex_units.mem as i64;

                match eval_result {
                    Ok(redeemer) => results.push(PhaseTwoEvalResult::Success(redeemer)),
                    Err(error) => results.push(PhaseTwoEvalResult::Error(redeemer, error)),
                }
            }

            Ok(results)
        }
        None => Ok(vec![]),
    }
}

fn trim_cost_modes(cost_mdls: Option<&CostMdls>) -> Option<CostMdls> {
    match cost_mdls {
        None => None,
        Some(mdls) => {
            Some(CostMdls {
                plutus_v1: mdls.plutus_v1.clone(),
                plutus_v2: mdls.plutus_v2.clone(),
                plutus_v3: match &mdls.plutus_v3 {
                    None => None,
                    Some(mdls_vec) => Some(mdls_vec[0..251].to_vec())
                }
            })
        }
    }
}