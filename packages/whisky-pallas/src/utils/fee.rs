use pallas::ledger::primitives::{
    conway::{TransactionBody, WitnessSet},
    Fragment,
};
use whisky_common::{Protocol, WError};

use crate::wrapper::transaction_body::Transaction;

pub fn calculate_fee(
    transaction: Transaction,
    script_size: usize,
    protocol_params: Protocol,
) -> Result<u64, WError> {
    let witness_set = &transaction.inner.transaction_witness_set;
    let fee = protocol_params.min_fee_b
        + transaction
            .inner
            .encode_fragment()
            .map_err(|_| {
                WError::new(
                    "Calculating Fee - ",
                    "Error while serializing TransactionBody",
                )
            })?
            .len() as u64
            * protocol_params.min_fee_a;
    let script_ref_fee = calculate_script_ref_fee(
        script_size,
        protocol_params.min_fee_ref_script_cost_per_byte,
    );
    let Some(redeemers) = &witness_set.redeemer else {
        return Ok(fee + script_ref_fee);
    };
    let script_fee = match redeemers.clone().unwrap() {
        pallas::ledger::primitives::conway::Redeemers::List(redeemers) => {
            let mut script_fee: f64 = 0.0;
            for redeemer in redeemers {
                let mem_units = redeemer.ex_units.mem;
                let step_units = redeemer.ex_units.steps;
                script_fee += mem_units as f64 * protocol_params.price_mem;
                script_fee += step_units as f64 * protocol_params.price_step;
            }
            script_fee.ceil() as u64
        }
        pallas::ledger::primitives::conway::Redeemers::Map(btree_map) => {
            let mut script_fee: f64 = 0.0;
            for (_, redeemer) in btree_map {
                let mem_units = redeemer.ex_units.mem;
                let step_units = redeemer.ex_units.steps;
                script_fee += mem_units as f64 * protocol_params.price_mem;
                script_fee += step_units as f64 * protocol_params.price_step;
            }
            script_fee.ceil() as u64
        }
    };
    Ok(fee + script_fee + script_ref_fee)
}

fn calculate_script_ref_fee(script_size: usize, min_fee_ref_script_cost_per_byte: u64) -> u64 {
    let mut script_fee: f64 = 0.0;
    const TIER_SIZE: u64 = 25600;
    const TIER_MULTIPLIER: f64 = 1.2;

    let mut current_multiplier: f64 = 1.0;
    let mut remaining_size = script_size as u64;
    while remaining_size >= TIER_SIZE {
        script_fee +=
            TIER_SIZE as f64 * current_multiplier * min_fee_ref_script_cost_per_byte as f64;
        remaining_size -= TIER_SIZE;
        current_multiplier *= TIER_MULTIPLIER;
    }
    if remaining_size > 0 {
        script_fee +=
            remaining_size as f64 * current_multiplier * min_fee_ref_script_cost_per_byte as f64;
    }
    script_fee.ceil() as u64
}
