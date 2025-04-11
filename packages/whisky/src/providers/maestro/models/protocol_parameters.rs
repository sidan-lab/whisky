use super::*;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug, Clone)]
pub struct Bytes {
    pub bytes: u64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ExUnits {
    pub memory: u64,
    pub cpu: u64,
}
#[derive(Deserialize, Debug, Clone)]
pub struct LovelaceAmount {
    pub lovelace: u64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ProtocolVersion {
    pub major: u64,
    pub minor: u64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PlutusCostModels {
    pub plutus_v1: Vec<u64>,
    pub plutus_v2: Vec<u64>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ScriptExecutionPrices {
    pub memory: String,
    pub cpu: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ProtocolParametersData {
    pub collateral_percentage: u64,
    pub constitutional_committee_max_term_length: i64,
    pub constitutional_committee_min_size: i64,
    pub delegate_representative_deposit: HashMap<String, serde_json::Value>,
    pub delegate_representative_max_idle_time: i64,
    pub delegate_representative_voting_thresholds: HashMap<String, serde_json::Value>,
    pub desired_number_of_stake_pools: u64,
    pub governance_action_deposit: HashMap<String, serde_json::Value>,
    pub governance_action_lifetime: i64,
    pub max_block_body_size: Bytes,
    pub max_block_header_size: Bytes,
    pub max_collateral_inputs: u64,
    pub max_execution_units_per_block: ExUnits,
    pub max_execution_units_per_transaction: ExUnits,
    pub max_reference_scripts_size: HashMap<String, serde_json::Value>,
    pub max_transaction_size: Bytes,
    pub max_value_size: Bytes,
    pub min_fee_coefficient: u64,
    pub min_fee_constant: HashMap<String, serde_json::Value>,
    pub min_fee_reference_scripts: HashMap<String, serde_json::Value>,
    pub min_stake_pool_cost: HashMap<String, serde_json::Value>,
    pub min_utxo_deposit_coefficient: u64,
    pub min_utxo_deposit_constant: HashMap<String, serde_json::Value>,
    pub monetary_expansion: String,
    pub plutus_cost_models: PlutusCostModels,
    pub script_execution_prices: ScriptExecutionPrices,
    pub stake_credential_deposit: HashMap<String, serde_json::Value>,
    pub stake_pool_deposit: HashMap<String, serde_json::Value>,
    pub stake_pool_pledge_influence: String,
    pub stake_pool_retirement_epoch_bound: u64,
    pub stake_pool_voting_thresholds: HashMap<String, serde_json::Value>,
    pub treasury_expansion: String,
    pub version: ProtocolVersion,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ProtocolParameters {
    pub data: ProtocolParametersData,
    pub last_updated: LastUpdated,
}
