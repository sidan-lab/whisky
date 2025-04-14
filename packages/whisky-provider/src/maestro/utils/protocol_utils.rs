use maestro_rust_sdk::models::epochs::Epoch;
use whisky_common::{models::Protocol, WError};

use crate::maestro::models::protocol_parameters::ProtocolParametersData;

pub fn protocol_paras_data_to_protocol(
    protocol_paras_data: ProtocolParametersData,
    epoch: Epoch,
) -> Result<Protocol, WError> {
    let protocol = Protocol {
        epoch: epoch.epoch_no,
        min_fee_a: protocol_paras_data.min_fee_coefficient,
        min_fee_b: protocol_paras_data.min_fee_constant["ada"]["lovelace"]
            .as_u64()
            .ok_or_else(WError::from_opt(
                "protocol_paras_data_to_protocol",
                "min_fee_constant",
            ))?,
        max_block_size: protocol_paras_data.max_block_body_size.bytes as i32,
        max_tx_size: protocol_paras_data.max_transaction_size.bytes as u32,
        max_block_header_size: protocol_paras_data.max_block_header_size.bytes as i32,
        key_deposit: protocol_paras_data.stake_credential_deposit["ada"]["lovelace"]
            .as_u64()
            .ok_or_else(WError::from_opt(
                "protocol_paras_data_to_protocol",
                "stake_credential_deposit",
            ))?,
        pool_deposit: protocol_paras_data.stake_pool_deposit["ada"]["lovelace"]
            .as_u64()
            .ok_or_else(WError::from_opt(
                "protocol_paras_data_to_protocol",
                "stake_pool_deposit",
            ))?,
        decentralisation: 0.0,
        min_pool_cost: protocol_paras_data.min_stake_pool_cost["ada"]["lovelace"]
            .as_u64()
            .ok_or_else(WError::from_opt(
                "protocol_paras_data_to_protocol",
                "min_stake_pool_cost",
            ))?
            .to_string(),
        price_mem: parse_fraction(&protocol_paras_data.script_execution_prices.memory).map_err(
            WError::from_err("protocol_paras_data_to_protocol - price_mem"),
        )?,
        price_step: parse_fraction(&protocol_paras_data.script_execution_prices.cpu).map_err(
            WError::from_err("protocol_paras_data_to_protocol - price_step"),
        )?,
        max_tx_ex_mem: protocol_paras_data
            .max_execution_units_per_transaction
            .memory
            .to_string(),
        max_tx_ex_steps: protocol_paras_data
            .max_execution_units_per_transaction
            .memory
            .to_string(),
        max_block_ex_mem: protocol_paras_data
            .max_execution_units_per_block
            .memory
            .to_string(),
        max_block_ex_steps: protocol_paras_data
            .max_execution_units_per_block
            .memory
            .to_string(),
        max_val_size: protocol_paras_data.max_value_size.bytes as u32,
        collateral_percent: protocol_paras_data.collateral_percentage as f64,
        max_collateral_inputs: protocol_paras_data.max_collateral_inputs as i32,
        coins_per_utxo_size: protocol_paras_data.min_utxo_deposit_constant["ada"]["lovelace"]
            .as_u64()
            .ok_or_else(WError::from_opt(
                "protocol_paras_data_to_protocol",
                "min_utxo_deposit_constant",
            ))?,
        min_fee_ref_script_cost_per_byte: protocol_paras_data.desired_number_of_stake_pools,
    };
    Ok(protocol)
}

fn parse_fraction(input: &str) -> Result<f64, WError> {
    let parts: Vec<&str> = input.split('/').collect();

    if parts.len() != 2 {
        return Err(WError::new("parse_fraction", "Invalid input format"));
    }

    let numerator: f64 = parts[0]
        .trim()
        .parse()
        .map_err(WError::from_err("parse_fraction - numerator"))?;
    let denominator: f64 = parts[1]
        .trim()
        .parse()
        .map_err(WError::from_err("parse_fraction - denominator"))?;

    Ok(numerator / denominator)
}
