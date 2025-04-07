use maestro_rust_sdk::models::epochs::Epoch;
use whisky_common::models::Protocol;

use crate::providers::maestro::models::protocol_parameters::ProtocolParametersData;

pub fn protocol_paras_data_to_protocol(
    protocol_paras_data: ProtocolParametersData,
    epoch: Epoch,
) -> Protocol {
    Protocol {
        epoch: epoch.epoch_no,
        min_fee_a: protocol_paras_data.min_fee_coefficient,
        min_fee_b: protocol_paras_data.min_fee_constant["ada"]["lovelace"]
            .as_u64()
            .unwrap(),
        max_block_size: protocol_paras_data.max_block_body_size.bytes as i32,
        max_tx_size: protocol_paras_data.max_transaction_size.bytes as u32,
        max_block_header_size: protocol_paras_data.max_block_header_size.bytes as i32,
        key_deposit: protocol_paras_data.stake_credential_deposit["ada"]["lovelace"]
            .as_u64()
            .unwrap(),
        pool_deposit: protocol_paras_data.stake_pool_deposit["ada"]["lovelace"]
            .as_u64()
            .unwrap(),
        decentralisation: 0.0,
        min_pool_cost: protocol_paras_data.min_stake_pool_cost["ada"]["lovelace"]
            .as_u64()
            .unwrap()
            .to_string(),
        price_mem: parse_fraction(&protocol_paras_data.script_execution_prices.memory).unwrap(),
        price_step: parse_fraction(&protocol_paras_data.script_execution_prices.cpu).unwrap(),
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
            .unwrap(), // TODO: tbc
        min_fee_ref_script_cost_per_byte: protocol_paras_data.desired_number_of_stake_pools, // TODO: tbc
    }
}

fn parse_fraction(input: &str) -> Result<f64, Box<dyn std::error::Error>> {
    let parts: Vec<&str> = input.split('/').collect();

    if parts.len() != 2 {
        return Err("Invalid input format".into());
    }

    let numerator: f64 = parts[0].trim().parse()?;
    let denominator: f64 = parts[1].trim().parse()?;

    Ok(numerator / denominator)
}
