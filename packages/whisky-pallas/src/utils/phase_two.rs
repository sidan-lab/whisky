use pallas::codec::minicbor;
use pallas::ledger::primitives::conway::{CostModels as PallasCostModels, Tx};
use pallas_primitives::conway::Redeemer as PallasRedeemer;
use uplc::machine::cost_model::ExBudget;
use uplc::tx::error::Error;
use uplc::tx::{eval, iter_redeemers, DataLookupTable, ResolvedInput, SlotConfig};
use uplc::Fragment;

pub enum PhaseTwoEvalResult {
    Success(PallasRedeemer),
    Error(PallasRedeemer, Error),
}

pub fn eval_phase_two(
    tx: &Tx,
    utxos: &[ResolvedInput],
    cost_mdls: Option<&PallasCostModels>,
    slot_config: &SlotConfig,
) -> Result<Vec<PhaseTwoEvalResult>, Error> {
    // Convert pallas 1.0.0-alpha.3 Tx to pallas 0.31.0 MintedTx for uplc compatibility
    // Encode using pallas 1.0.0-alpha.3's CBOR encoder
    let mut tx_bytes = Vec::new();
    minicbor::encode(tx, &mut tx_bytes)
        .map_err(|e| Error::FragmentDecode(format!("Failed to encode tx: {:?}", e).into()))?;

    // Deserialize into pallas 0.31.0 types (via uplc's pallas-primitives 0.31.0 dependency)
    let tx_for_uplc =
        pallas_primitives::conway::MintedTx::decode_fragment(&tx_bytes).map_err(|e| {
            Error::FragmentDecode(format!("Failed to decode tx for uplc: {:?}", e).into())
        })?;

    let lookup_table = DataLookupTable::from_transaction(&tx_for_uplc, utxos);

    // Convert cost models if provided
    let cost_mdls_for_uplc = cost_mdls
        .map(|cm| {
            let mut cm_bytes = Vec::new();
            minicbor::encode(cm, &mut cm_bytes).ok()?;
            pallas_primitives::conway::CostModels::decode_fragment(&cm_bytes).ok()
        })
        .flatten();

    let redeemers = tx_for_uplc.transaction_witness_set.redeemer.as_ref();

    match redeemers {
        Some(rs) => {
            let mut results = Vec::new();
            for (key, data, ex_units) in iter_redeemers(rs) {
                let remaining_budget = ExBudget {
                    cpu: i64::MAX,
                    mem: i64::MAX,
                };
                let redeemer_for_uplc = pallas_primitives::conway::Redeemer {
                    tag: key.tag,
                    index: key.index,
                    data: data.clone(),
                    ex_units,
                };

                let eval_result = eval::eval_redeemer(
                    &tx_for_uplc,
                    utxos,
                    slot_config,
                    &redeemer_for_uplc,
                    &lookup_table,
                    cost_mdls_for_uplc.as_ref(),
                    &remaining_budget,
                );

                // The redeemer_for_uplc is already pallas_primitives 0.31.0, so no conversion needed
                let pallas_redeemer = redeemer_for_uplc.clone();

                match eval_result {
                    Ok(updated_redeemer) => {
                        results.push(PhaseTwoEvalResult::Success(updated_redeemer))
                    }
                    Err(error) => results.push(PhaseTwoEvalResult::Error(pallas_redeemer, error)),
                }
            }

            Ok(results)
        }
        None => Ok(vec![]),
    }
}
