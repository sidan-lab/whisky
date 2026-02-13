use pallas::codec::minicbor;
use pallas::ledger::primitives::conway::{
    CostModels as PallasCostModels, Redeemer as PallasRedeemer, Tx,
};
use uplc::machine::cost_model::ExBudget;
use uplc::tx::error::Error;
use uplc::tx::{eval, iter_redeemers, DataLookupTable, ResolvedInput, SlotConfig};
use uplc::Fragment;
// Import pallas_codec for 0.31.0 types
use pallas_codec::minicbor as minicbor_031;

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

                // Convert back to pallas 1.0.0-alpha.3 Redeemer for result
                // We need to serialize/deserialize to convert between versions
                let convert_redeemer =
                    |r: &pallas_primitives::conway::Redeemer| -> Result<PallasRedeemer, Error> {
                        let mut r_bytes = Vec::new();
                        // Use minicbor from pallas 0.31.0 for encoding 0.31.0 types
                        minicbor_031::encode(r, &mut r_bytes).map_err(|e| {
                            Error::FragmentDecode(
                                format!("Failed to encode redeemer: {:?}", e).into(),
                            )
                        })?;
                        // Use pallas 1.0.0-alpha.3's minicbor for decoding into 1.0.0-alpha.3 types
                        minicbor::decode(&r_bytes).map_err(|e| {
                            Error::FragmentDecode(
                                format!("Failed to decode redeemer: {:?}", e).into(),
                            )
                        })
                    };

                let pallas_redeemer = convert_redeemer(&redeemer_for_uplc)?;

                match eval_result {
                    Ok(updated_redeemer) => {
                        let result_redeemer = convert_redeemer(&updated_redeemer)?;
                        results.push(PhaseTwoEvalResult::Success(result_redeemer))
                    }
                    Err(error) => results.push(PhaseTwoEvalResult::Error(pallas_redeemer, error)),
                }
            }

            Ok(results)
        }
        None => Ok(vec![]),
    }
}
