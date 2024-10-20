use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Protocol {
    pub epoch: i32,
    pub min_fee_a: u64,
    pub min_fee_b: u64,
    pub max_block_size: i32,
    pub max_tx_size: u32,
    pub max_block_header_size: i32,
    pub key_deposit: u64,
    pub pool_deposit: u64,
    pub decentralisation: f64,
    pub min_pool_cost: String,
    pub price_mem: f64,
    pub price_step: f64,
    pub max_tx_ex_mem: String,
    pub max_tx_ex_steps: String,
    pub max_block_ex_mem: String,
    pub max_block_ex_steps: String,
    pub max_val_size: u32,
    pub collateral_percent: f64,
    pub max_collateral_inputs: i32,
    pub coins_per_utxo_size: u64,
    pub min_fee_ref_script_cost_per_byte: u64,
}

impl Default for Protocol {
    fn default() -> Self {
        Protocol {
            epoch: 0,
            min_fee_a: 44,
            min_fee_b: 155381,
            max_block_size: 98304,
            max_tx_size: 16384,
            max_block_header_size: 1100,
            key_deposit: 2000000,
            pool_deposit: 500000000,
            min_pool_cost: "340000000".to_string(),
            price_mem: 0.0577,
            price_step: 0.0000721,
            max_tx_ex_mem: "16000000".to_string(),
            max_tx_ex_steps: "10000000000".to_string(),
            max_block_ex_mem: "80000000".to_string(),
            max_block_ex_steps: "40000000000".to_string(),
            max_val_size: 5000,
            collateral_percent: 150.0,
            max_collateral_inputs: 3,
            coins_per_utxo_size: 4310,
            min_fee_ref_script_cost_per_byte: 15,
            decentralisation: 0.0,
        }
    }
}
