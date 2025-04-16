use std::collections::HashMap;

use super::*;
use serde::Deserialize;

use super::{script::Script, utxo::Utxo};

#[derive(Deserialize, Debug, Clone)]
pub struct Certificates {
    pub auth_committee_hot_certs: Vec<serde_json::Value>,
    pub mir_transfers: Vec<serde_json::Value>,
    pub pool_registrations: Vec<serde_json::Value>,
    pub pool_retirements: Vec<serde_json::Value>,
    pub reg_certs: Vec<serde_json::Value>,
    pub reg_drep_certs: Vec<serde_json::Value>,
    pub resign_committee_cold_certs: Vec<serde_json::Value>,
    pub stake_delegations: Vec<serde_json::Value>,
    pub stake_deregistrations: Vec<serde_json::Value>,
    pub stake_reg_delegations: Vec<serde_json::Value>,
    pub stake_registrations: Vec<serde_json::Value>,
    pub stake_vote_delegations: Vec<serde_json::Value>,
    pub stake_vote_reg_delegations: Vec<serde_json::Value>,
    pub unreg_certs: Vec<serde_json::Value>,
    pub unreg_drep_certs: Vec<serde_json::Value>,
    pub update_drep_certs: Vec<serde_json::Value>,
    pub vote_delegations: Vec<serde_json::Value>,
    pub vote_reg_delegations: Vec<serde_json::Value>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Redeemers {
    pub certificates: Vec<serde_json::Value>,
    pub mints: Vec<serde_json::Value>,
    pub spends: Vec<serde_json::Value>,
    pub withdrawals: Vec<serde_json::Value>,
    pub votes: Vec<serde_json::Value>,
    pub proposals: Vec<serde_json::Value>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MintAsset {
    pub unit: String,
    pub amount: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TransactionDetail {
    pub additional_signers: Vec<String>,
    pub block_absolute_slot: i64,
    pub block_hash: String,
    pub block_height: i64,
    pub block_timestamp: i64,
    pub block_tx_index: i64,
    pub certificates: Certificates,
    pub collateral_inputs: Vec<Utxo>,
    pub collateral_return: serde_json::Value,
    pub deposit: i64,
    pub fee: i64,
    pub inputs: Vec<Utxo>,
    pub invalid_before: Option<i64>,
    pub invalid_hereafter: Option<i64>,
    pub metadata: serde_json::Value,
    pub mint: Vec<MintAsset>,
    pub outputs: Vec<Utxo>,
    pub redeemers: HashMap<String, serde_json::Value>,
    pub reference_inputs: Vec<serde_json::Value>,
    pub scripts_executed: Vec<Script>,
    pub scripts_successful: bool,
    pub size: i64,
    pub tx_hash: String,
    pub withdrawals: Vec<serde_json::Value>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TransactionDetails {
    pub data: TransactionDetail,
    pub last_updated: LastUpdated,
}
