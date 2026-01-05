use std::collections::BTreeMap;

use pallas::ledger::primitives::conway::{
    CostModels as PallasCostModels, DRepVotingThresholds as PallasDRepVotingThresholds,
    ExUnitPrices as PallasExUnitPrices, ExUnits as PallasExUnits,
    PoolVotingThresholds as PallasPoolVotingThresholds,
};
use pallas::ledger::primitives::{
    conway::ProtocolParamUpdate as PallasProtocolParamUpdate, Fragment, RationalNumber,
};
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CostModels {
    pub plutus_v1: Option<Vec<i64>>,
    pub plutus_v2: Option<Vec<i64>>,
    pub plutus_v3: Option<Vec<i64>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PoolVotingThresholds {
    pub motion_no_confidence: (u64, u64),
    pub committee_normal: (u64, u64),
    pub committee_no_confidence: (u64, u64),
    pub hard_fork_initiation: (u64, u64),
    pub security_voting_threshold: (u64, u64),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DRepVotingThresholds {
    pub motion_no_confidence: (u64, u64),
    pub committee_normal: (u64, u64),
    pub committee_no_confidence: (u64, u64),
    pub update_constitution: (u64, u64),
    pub hard_fork_initiation: (u64, u64),
    pub pp_network_group: (u64, u64),
    pub pp_economic_group: (u64, u64),
    pub pp_technical_group: (u64, u64),
    pub pp_governance_group: (u64, u64),
    pub treasury_withdrawal: (u64, u64),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ExUnitPrices {
    pub mem_price: (u64, u64),
    pub step_price: (u64, u64),
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ExUnits {
    pub mem: u64,
    pub steps: u64,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ProtocolParamUpdate {
    pub inner: PallasProtocolParamUpdate,
}

impl ProtocolParamUpdate {
    pub fn new(
        minfee_a: Option<u64>,
        minfee_b: Option<u64>,
        max_block_body_size: Option<u64>,
        max_transaction_size: Option<u64>,
        max_block_header_size: Option<u64>,
        key_deposit: Option<u64>,
        pool_deposit: Option<u64>,
        maximum_epoch: Option<u64>,
        desired_number_of_stake_pools: Option<u64>,
        pool_pledge_influence: Option<(u64, u64)>, // RationalNumber
        expansion_rate: Option<(u64, u64)>,
        treasury_growth_rate: Option<(u64, u64)>,
        min_pool_cost: Option<u64>,
        ada_per_utxo_byte: Option<u64>,
        cost_models_for_script_languages: Option<CostModels>,
        execution_costs: Option<ExUnitPrices>,
        max_tx_ex_units: Option<ExUnits>,
        max_block_ex_units: Option<ExUnits>,
        max_value_size: Option<u64>,
        collateral_percentage: Option<u64>,
        max_collateral_inputs: Option<u64>,
        pool_voting_thresholds: Option<PoolVotingThresholds>,
        drep_voting_thresholds: Option<DRepVotingThresholds>,
        min_committee_size: Option<u64>,
        committee_term_limit: Option<u64>,
        governance_action_validity_period: Option<u64>,
        governance_action_deposit: Option<u64>,
        drep_deposit: Option<u64>,
        drep_inactivity_period: Option<u64>,
        minfee_refscript_cost_per_byte: Option<(u64, u64)>, // unit interval
    ) -> Result<Self, String> {
        Ok(Self {
            inner: PallasProtocolParamUpdate {
                minfee_a,
                minfee_b,
                max_block_body_size,
                max_transaction_size,
                max_block_header_size,
                key_deposit,
                pool_deposit,
                maximum_epoch,
                desired_number_of_stake_pools,
                pool_pledge_influence: Self::parse_rational_number(pool_pledge_influence),
                expansion_rate: Self::parse_rational_number(expansion_rate),
                treasury_growth_rate: Self::parse_rational_number(treasury_growth_rate),
                min_pool_cost,
                ada_per_utxo_byte,
                cost_models_for_script_languages: Self::parse_cost_models(
                    cost_models_for_script_languages,
                ),
                execution_costs: Self::parse_ex_unit_prices(execution_costs),
                max_tx_ex_units: Self::parse_ex_units(max_tx_ex_units),
                max_block_ex_units: Self::parse_ex_units(max_block_ex_units),
                max_value_size,
                collateral_percentage,
                max_collateral_inputs,
                pool_voting_thresholds: Self::parse_pool_voting_thresholds(pool_voting_thresholds),
                drep_voting_thresholds: Self::parse_drep_voting_thresholds(drep_voting_thresholds),
                min_committee_size,
                committee_term_limit,
                governance_action_validity_period,
                governance_action_deposit,
                drep_deposit,
                drep_inactivity_period,
                minfee_refscript_cost_per_byte: Self::parse_rational_number(
                    minfee_refscript_cost_per_byte,
                ),
            },
        })
    }

    pub fn encode(&self) -> String {
        hex::encode(
            self.inner
                .encode_fragment()
                .expect("encoding failed at ProtocolParameterUpdate"),
        )
    }

    pub fn decode_bytes(bytes: &[u8]) -> Result<Self, String> {
        let inner = PallasProtocolParamUpdate::decode_fragment(&bytes)
            .map_err(|e| format!("Fragment decode error: {}", e.to_string()))?;
        Ok(Self { inner })
    }

    fn parse_rational_number(tuple: Option<(u64, u64)>) -> Option<RationalNumber> {
        match tuple {
            Some((numerator, denominator)) => Some(RationalNumber {
                numerator,
                denominator,
            }),
            None => None,
        }
    }

    fn parse_cost_models(cost_models: Option<CostModels>) -> Option<PallasCostModels> {
        match cost_models {
            Some(models) => Some(PallasCostModels {
                plutus_v1: models.plutus_v1,
                plutus_v2: models.plutus_v2,
                plutus_v3: models.plutus_v3,
                unknown: BTreeMap::new(),
            }),
            None => None,
        }
    }

    fn parse_ex_unit_prices(tuple: Option<ExUnitPrices>) -> Option<PallasExUnitPrices> {
        match tuple {
            Some(prices) => Some(PallasExUnitPrices {
                mem_price: Self::parse_rational_number(Some(prices.mem_price))?,
                step_price: Self::parse_rational_number(Some(prices.step_price))?,
            }),
            None => None,
        }
    }

    fn parse_ex_units(ex_units: Option<ExUnits>) -> Option<PallasExUnits> {
        match ex_units {
            Some(units) => Some(pallas::ledger::primitives::conway::ExUnits {
                mem: units.mem,
                steps: units.steps,
            }),
            None => None,
        }
    }

    fn parse_pool_voting_thresholds(
        tuple: Option<PoolVotingThresholds>,
    ) -> Option<PallasPoolVotingThresholds> {
        match tuple {
            Some(thresholds) => Some(PallasPoolVotingThresholds {
                motion_no_confidence: Self::parse_rational_number(Some(
                    thresholds.motion_no_confidence,
                ))?,
                committee_normal: Self::parse_rational_number(Some(thresholds.committee_normal))?,
                committee_no_confidence: Self::parse_rational_number(Some(
                    thresholds.committee_no_confidence,
                ))?,
                hard_fork_initiation: Self::parse_rational_number(Some(
                    thresholds.hard_fork_initiation,
                ))?,
                security_voting_threshold: Self::parse_rational_number(Some(
                    thresholds.security_voting_threshold,
                ))?,
            }),
            None => None,
        }
    }

    fn parse_drep_voting_thresholds(
        tuple: Option<DRepVotingThresholds>,
    ) -> Option<PallasDRepVotingThresholds> {
        match tuple {
            Some(thresholds) => Some(PallasDRepVotingThresholds {
                motion_no_confidence: Self::parse_rational_number(Some(
                    thresholds.motion_no_confidence,
                ))?,
                committee_normal: Self::parse_rational_number(Some(thresholds.committee_normal))?,
                committee_no_confidence: Self::parse_rational_number(Some(
                    thresholds.committee_no_confidence,
                ))?,
                update_constitution: Self::parse_rational_number(Some(
                    thresholds.update_constitution,
                ))?,
                hard_fork_initiation: Self::parse_rational_number(Some(
                    thresholds.hard_fork_initiation,
                ))?,
                pp_network_group: Self::parse_rational_number(Some(thresholds.pp_network_group))?,
                pp_economic_group: Self::parse_rational_number(Some(thresholds.pp_economic_group))?,
                pp_technical_group: Self::parse_rational_number(Some(
                    thresholds.pp_technical_group,
                ))?,
                pp_governance_group: Self::parse_rational_number(Some(
                    thresholds.pp_governance_group,
                ))?,
                treasury_withdrawal: Self::parse_rational_number(Some(
                    thresholds.treasury_withdrawal,
                ))?,
            }),
            None => None,
        }
    }
}
