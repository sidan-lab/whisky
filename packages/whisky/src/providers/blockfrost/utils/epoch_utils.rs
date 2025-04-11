use whisky_common::Protocol;

use crate::providers::blockfrost::models::EpochParam;

pub fn epoch_param_to_protocol(epoch_param: EpochParam) -> Protocol {
    Protocol {
        epoch: epoch_param.epoch,
        min_fee_a: epoch_param.min_fee_a as u64,
        min_fee_b: epoch_param.min_fee_b as u64,
        max_block_size: epoch_param.max_block_size,
        max_tx_size: epoch_param.max_tx_size as u32,
        max_block_header_size: epoch_param.max_block_header_size,
        key_deposit: epoch_param.key_deposit.parse::<u64>().unwrap(), // TODO: handle unwrap
        pool_deposit: epoch_param.pool_deposit.parse::<u64>().unwrap(), // TODO: handle unwrap
        decentralisation: epoch_param.decentralisation_param,
        min_pool_cost: epoch_param.min_pool_cost,
        price_mem: epoch_param.price_mem.unwrap(), // TODO: handle unwrap
        price_step: epoch_param.price_step.unwrap(), // TODO: handle unwrap
        max_tx_ex_mem: epoch_param.max_tx_ex_mem.unwrap_or("".to_string()),
        max_tx_ex_steps: epoch_param.max_tx_ex_steps.unwrap_or("".to_string()),
        max_block_ex_mem: epoch_param.max_block_ex_mem.unwrap_or("".to_string()),
        max_block_ex_steps: epoch_param.max_block_ex_steps.unwrap_or("".to_string()),
        max_val_size: epoch_param.max_val_size.unwrap().parse::<u32>().unwrap(), // TODO: handle unwrap
        collateral_percent: epoch_param.collateral_percent.unwrap() as f64, // TODO: handle unwrap
        max_collateral_inputs: epoch_param.max_collateral_inputs.unwrap(),  // TODO: handle unwrap
        coins_per_utxo_size: epoch_param
            .coins_per_utxo_size
            .unwrap()
            .parse::<u64>()
            .unwrap(), // TODO: handle unwrap
        min_fee_ref_script_cost_per_byte: epoch_param.min_fee_ref_script_cost_per_byte.unwrap()
            as u64, // TODO: handle unwrap
    }
}
