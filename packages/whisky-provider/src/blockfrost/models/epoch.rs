use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EpochParam {
    /// Epoch number
    #[serde(rename = "epoch")]
    pub epoch: i32,
    /// The linear factor for the minimum fee calculation for given epoch
    #[serde(rename = "min_fee_a")]
    pub min_fee_a: i32,
    /// The constant factor for the minimum fee calculation
    #[serde(rename = "min_fee_b")]
    pub min_fee_b: i32,
    /// Maximum block body size in Bytes
    #[serde(rename = "max_block_size")]
    pub max_block_size: i32,
    /// Maximum transaction size
    #[serde(rename = "max_tx_size")]
    pub max_tx_size: i32,
    /// Maximum block header size
    #[serde(rename = "max_block_header_size")]
    pub max_block_header_size: i32,
    /// The amount of a key registration deposit in Lovelaces
    #[serde(rename = "key_deposit")]
    pub key_deposit: String,
    /// The amount of a pool registration deposit in Lovelaces
    #[serde(rename = "pool_deposit")]
    pub pool_deposit: String,
    /// Epoch bound on pool retirement
    #[serde(rename = "e_max")]
    pub e_max: i32,
    /// Desired number of pools
    #[serde(rename = "n_opt")]
    pub n_opt: i32,
    /// Pool pledge influence
    #[serde(rename = "a0")]
    pub a0: f64,
    /// Monetary expansion
    #[serde(rename = "rho")]
    pub rho: f64,
    /// Treasury expansion
    #[serde(rename = "tau")]
    pub tau: f64,
    /// Percentage of blocks produced by federated nodes
    #[serde(rename = "decentralisation_param")]
    pub decentralisation_param: f64,
    /// Seed for extra entropy
    #[serde(rename = "extra_entropy", deserialize_with = "Option::deserialize")]
    pub extra_entropy: Option<String>,
    /// Accepted protocol major version
    #[serde(rename = "protocol_major_ver")]
    pub protocol_major_ver: i32,
    /// Accepted protocol minor version
    #[serde(rename = "protocol_minor_ver")]
    pub protocol_minor_ver: i32,
    /// Minimum UTXO value. Use `coins_per_utxo_size` for Alonzo and later eras
    #[serde(rename = "min_utxo")]
    pub min_utxo: String,
    /// Minimum stake cost forced on the pool
    #[serde(rename = "min_pool_cost")]
    pub min_pool_cost: String,
    /// Epoch number only used once
    #[serde(rename = "nonce")]
    pub nonce: String,
    /// Cost models parameters for Plutus Core scripts
    #[serde(rename = "cost_models", deserialize_with = "Option::deserialize")]
    pub cost_models: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// Cost models parameters for Plutus Core scripts in raw list form
    pub cost_models_raw: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
    /// The per word cost of script memory usage
    #[serde(rename = "price_mem", deserialize_with = "Option::deserialize")]
    pub price_mem: Option<f64>,
    /// The cost of script execution step usage
    #[serde(rename = "price_step", deserialize_with = "Option::deserialize")]
    pub price_step: Option<f64>,
    /// The maximum number of execution memory allowed to be used in a single transaction
    #[serde(rename = "max_tx_ex_mem", deserialize_with = "Option::deserialize")]
    pub max_tx_ex_mem: Option<String>,
    /// The maximum number of execution steps allowed to be used in a single transaction
    #[serde(rename = "max_tx_ex_steps", deserialize_with = "Option::deserialize")]
    pub max_tx_ex_steps: Option<String>,
    /// The maximum number of execution memory allowed to be used in a single block
    #[serde(rename = "max_block_ex_mem", deserialize_with = "Option::deserialize")]
    pub max_block_ex_mem: Option<String>,
    /// The maximum number of execution steps allowed to be used in a single block
    #[serde(
        rename = "max_block_ex_steps",
        deserialize_with = "Option::deserialize"
    )]
    pub max_block_ex_steps: Option<String>,
    /// The maximum Val size
    #[serde(rename = "max_val_size", deserialize_with = "Option::deserialize")]
    pub max_val_size: Option<String>,
    /// The percentage of the transactions fee which must be provided as collateral when including non-native scripts
    #[serde(
        rename = "collateral_percent",
        deserialize_with = "Option::deserialize"
    )]
    pub collateral_percent: Option<i32>,
    /// The maximum number of collateral inputs allowed in a transaction
    #[serde(
        rename = "max_collateral_inputs",
        deserialize_with = "Option::deserialize"
    )]
    pub max_collateral_inputs: Option<i32>,
    /// Cost per UTxO word for Alonzo. Cost per UTxO byte for Babbage and later.
    #[serde(
        rename = "coins_per_utxo_size",
        deserialize_with = "Option::deserialize"
    )]
    pub coins_per_utxo_size: Option<String>,
    /// Cost per UTxO word for Alonzo. Cost per UTxO byte for Babbage and later.
    #[serde(
        rename = "coins_per_utxo_word",
        deserialize_with = "Option::deserialize"
    )]
    pub coins_per_utxo_word: Option<String>,
    /// Pool Voting threshold for motion of no-confidence.
    #[serde(
        rename = "pvt_motion_no_confidence",
        deserialize_with = "Option::deserialize"
    )]
    pub pvt_motion_no_confidence: Option<f64>,
    /// Pool Voting threshold for new committee/threshold (normal state).
    #[serde(
        rename = "pvt_committee_normal",
        deserialize_with = "Option::deserialize"
    )]
    pub pvt_committee_normal: Option<f64>,
    /// Pool Voting threshold for new committee/threshold (state of no-confidence).
    #[serde(
        rename = "pvt_committee_no_confidence",
        deserialize_with = "Option::deserialize"
    )]
    pub pvt_committee_no_confidence: Option<f64>,
    /// Pool Voting threshold for hard-fork initiation.
    #[serde(
        rename = "pvt_hard_fork_initiation",
        deserialize_with = "Option::deserialize"
    )]
    pub pvt_hard_fork_initiation: Option<f64>,
    /// DRep Vote threshold for motion of no-confidence.
    #[serde(
        rename = "dvt_motion_no_confidence",
        deserialize_with = "Option::deserialize"
    )]
    pub dvt_motion_no_confidence: Option<f64>,
    /// DRep Vote threshold for new committee/threshold (normal state).
    #[serde(
        rename = "dvt_committee_normal",
        deserialize_with = "Option::deserialize"
    )]
    pub dvt_committee_normal: Option<f64>,
    /// DRep Vote threshold for new committee/threshold (state of no-confidence).
    #[serde(
        rename = "dvt_committee_no_confidence",
        deserialize_with = "Option::deserialize"
    )]
    pub dvt_committee_no_confidence: Option<f64>,
    /// DRep Vote threshold for update to the Constitution.
    #[serde(
        rename = "dvt_update_to_constitution",
        deserialize_with = "Option::deserialize"
    )]
    pub dvt_update_to_constitution: Option<f64>,
    /// DRep Vote threshold for hard-fork initiation.
    #[serde(
        rename = "dvt_hard_fork_initiation",
        deserialize_with = "Option::deserialize"
    )]
    pub dvt_hard_fork_initiation: Option<f64>,
    /// DRep Vote threshold for protocol parameter changes, network group.
    #[serde(
        rename = "dvt_p_p_network_group",
        deserialize_with = "Option::deserialize"
    )]
    pub dvt_p_p_network_group: Option<f64>,
    /// DRep Vote threshold for protocol parameter changes, economic group.
    #[serde(
        rename = "dvt_p_p_economic_group",
        deserialize_with = "Option::deserialize"
    )]
    pub dvt_p_p_economic_group: Option<f64>,
    /// DRep Vote threshold for protocol parameter changes, technical group.
    #[serde(
        rename = "dvt_p_p_technical_group",
        deserialize_with = "Option::deserialize"
    )]
    pub dvt_p_p_technical_group: Option<f64>,
    /// DRep Vote threshold for protocol parameter changes, governance group.
    #[serde(rename = "dvt_p_p_gov_group", deserialize_with = "Option::deserialize")]
    pub dvt_p_p_gov_group: Option<f64>,
    /// DRep Vote threshold for treasury withdrawal.
    #[serde(
        rename = "dvt_treasury_withdrawal",
        deserialize_with = "Option::deserialize"
    )]
    pub dvt_treasury_withdrawal: Option<f64>,
    /// Minimal constitutional committee size.
    #[serde(
        rename = "committee_min_size",
        deserialize_with = "Option::deserialize"
    )]
    pub committee_min_size: Option<String>,
    /// Constitutional committee term limits.
    #[serde(
        rename = "committee_max_term_length",
        deserialize_with = "Option::deserialize"
    )]
    pub committee_max_term_length: Option<String>,
    /// Governance action expiration.
    #[serde(
        rename = "gov_action_lifetime",
        deserialize_with = "Option::deserialize"
    )]
    pub gov_action_lifetime: Option<String>,
    /// Governance action deposit.
    #[serde(
        rename = "gov_action_deposit",
        deserialize_with = "Option::deserialize"
    )]
    pub gov_action_deposit: Option<String>,
    /// DRep deposit amount.
    #[serde(rename = "drep_deposit", deserialize_with = "Option::deserialize")]
    pub drep_deposit: Option<String>,
    /// DRep activity period.
    #[serde(rename = "drep_activity", deserialize_with = "Option::deserialize")]
    pub drep_activity: Option<String>,
    /// Pool Voting threshold for security-relevant protocol parameters changes. Renamed to pvt_p_p_security_group.
    #[serde(
        rename = "pvtpp_security_group",
        deserialize_with = "Option::deserialize"
    )]
    pub pvtpp_security_group: Option<f64>,
    /// Pool Voting threshold for security-relevant protocol parameters changes.
    #[serde(
        rename = "pvt_p_p_security_group",
        deserialize_with = "Option::deserialize"
    )]
    pub pvt_p_p_security_group: Option<f64>,
    #[serde(
        rename = "min_fee_ref_script_cost_per_byte",
        deserialize_with = "Option::deserialize"
    )]
    pub min_fee_ref_script_cost_per_byte: Option<f64>,
}
