use pallas_primitives::conway::{CostModels, MintedTx, Redeemer};
use uplc::machine::cost_model::ExBudget;
use uplc::tx::error::Error;
use uplc::tx::{eval, iter_redeemers, DataLookupTable, ResolvedInput, SlotConfig};

pub enum PhaseTwoEvalResult {
    Success(Redeemer),
    Error(Redeemer, Error),
}

pub fn eval_phase_two(
    tx: &MintedTx,
    utxos: &[ResolvedInput],
    cost_mdls: Option<&CostModels>,
    slot_config: &SlotConfig,
) -> Result<Vec<PhaseTwoEvalResult>, Error> {
    let redeemers = tx.transaction_witness_set.redeemer.as_ref();

    let lookup_table = DataLookupTable::from_transaction(tx, utxos);

    match redeemers {
        Some(rs) => {
            let mut results = Vec::new();
            for (key, data, ex_units) in iter_redeemers(rs) {
                let remaining_budget = ExBudget {
                    cpu: i64::MAX,
                    mem: i64::MAX,
                };
                let redeemer = Redeemer {
                    tag: key.tag,
                    index: key.index,
                    data: data.clone(),
                    ex_units,
                };

                let eval_result = eval::eval_redeemer(
                    tx,
                    utxos,
                    slot_config,
                    &redeemer,
                    &lookup_table,
                    cost_mdls,
                    &remaining_budget,
                );

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

// fn trim_cost_modes(cost_mdls: Option<&CostModels>) -> Option<CostModels> {
//     match cost_mdls {
//         None => None,
//         Some(mdls) => {
//             Some(CostModels {
//                 plutus_v1: mdls.plutus_v1.clone(),
//                 plutus_v2: mdls.plutus_v2.clone(),
//                 plutus_v3: match &mdls.plutus_v3 {
//                     None => None,
//                     Some(mdls_vec) => Some(mdls_vec[0..251].to_vec())
//                 }
//             })
//         }
//     }
// }