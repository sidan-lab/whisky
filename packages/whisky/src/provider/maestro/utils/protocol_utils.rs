use maestro_rust_sdk::models::general::ProtocolParametersData;
use whisky_common::models::Protocol;

pub fn protocol_paras_data_to_protocol(protocol_paras_data: ProtocolParametersData) -> Protocol {
    Protocol {
        epoch: protocol_paras_data.stake_pool_retirement_epoch_bound as i32, // TODO: tbc
        min_fee_a: protocol_paras_data.min_fee_coefficient,
        min_fee_b: protocol_paras_data.min_fee_constant.lovelace,
        max_block_size: protocol_paras_data.max_block_body_size.bytes as i32,
        max_tx_size: protocol_paras_data.max_transaction_size.bytes as u32,
        max_block_header_size: protocol_paras_data.max_block_header_size.bytes as i32,
        key_deposit: protocol_paras_data.stake_credential_deposit.lovelace,
        pool_deposit: protocol_paras_data.stake_pool_deposit.lovelace,
        decentralisation: 0.0,
        min_pool_cost: protocol_paras_data.min_stake_pool_cost.lovelace.to_string(),
        price_mem: protocol_paras_data
            .script_execution_prices
            .memory
            .parse()
            .unwrap(),
        price_step: protocol_paras_data
            .script_execution_prices
            .cpu
            .parse()
            .unwrap(),
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
        coins_per_utxo_size: protocol_paras_data.min_utxo_deposit_constant.lovelace, // TODO: tbc
        min_fee_ref_script_cost_per_byte: protocol_paras_data.desired_number_of_stake_pools, // TODO: tbc
    }
}
