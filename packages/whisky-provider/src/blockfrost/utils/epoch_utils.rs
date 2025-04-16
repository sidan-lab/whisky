use whisky_common::{Protocol, WError};

use crate::blockfrost::models::EpochParam;

pub fn epoch_param_to_protocol(epoch_param: EpochParam) -> Result<Protocol, WError> {
    let protocol =
        Protocol {
            epoch: epoch_param.epoch,
            min_fee_a: epoch_param.min_fee_a as u64,
            min_fee_b: epoch_param.min_fee_b as u64,
            max_block_size: epoch_param.max_block_size,
            max_tx_size: epoch_param.max_tx_size as u32,
            max_block_header_size: epoch_param.max_block_header_size,
            key_deposit: epoch_param
                .key_deposit
                .parse::<u64>()
                .map_err(WError::from_err("epoch_param_to_protocol - key_deposit"))?,
            pool_deposit: epoch_param
                .pool_deposit
                .parse::<u64>()
                .map_err(WError::from_err("epoch_param_to_protocol - pool_deposit"))?,
            decentralisation: epoch_param.decentralisation_param,
            min_pool_cost: epoch_param.min_pool_cost,
            price_mem: epoch_param
                .price_mem
                .ok_or_else(WError::from_opt("epoch_param_to_protocol", "price_mem"))?,
            price_step: epoch_param
                .price_step
                .ok_or_else(WError::from_opt("epoch_param_to_protocol", "price_step"))?,
            max_tx_ex_mem: epoch_param.max_tx_ex_mem.unwrap_or("".to_string()),
            max_tx_ex_steps: epoch_param.max_tx_ex_steps.unwrap_or("".to_string()),
            max_block_ex_mem: epoch_param.max_block_ex_mem.unwrap_or("".to_string()),
            max_block_ex_steps: epoch_param.max_block_ex_steps.unwrap_or("".to_string()),
            max_val_size: epoch_param
                .max_val_size
                .ok_or_else(WError::from_opt("epoch_param_to_protocol", "max_val_size"))?
                .parse::<u32>()
                .map_err(WError::from_err(
                    "epoch_param_to_protocol - max_val_size - parse",
                ))?,
            collateral_percent: epoch_param.collateral_percent.ok_or_else(WError::from_opt(
                "epoch_param_to_protocol",
                "collateral_percent",
            ))? as f64,
            max_collateral_inputs: epoch_param.max_collateral_inputs.ok_or_else(
                WError::from_opt("epoch_param_to_protocol", "max_collateral_inputs"),
            )?,
            coins_per_utxo_size: epoch_param
                .coins_per_utxo_size
                .ok_or_else(WError::from_opt(
                    "epoch_param_to_protocol",
                    "coins_per_utxo_size",
                ))?
                .parse::<u64>()
                .map_err(WError::from_err(
                    "epoch_param_to_protocol - coins_per_utxo_size - parse",
                ))?,
            min_fee_ref_script_cost_per_byte: epoch_param
                .min_fee_ref_script_cost_per_byte
                .ok_or_else(WError::from_opt(
                    "epoch_param_to_protocol",
                    "min_fee_ref_script_cost_per_byte",
                ))? as u64,
        };
    Ok(protocol)
}
