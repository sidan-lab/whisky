use cardano_serialization_lib as csl;
use whisky_common::WError;

use super::primitives::{
    proto_to_anchor, proto_to_bignum, proto_to_credential, proto_to_unit_interval,
    proto_to_vote_kind,
};
use crate::tx_prototype::types::*;

/// Convert VoterPrototype to CSL Voter
pub fn proto_to_voter(voter: &VoterPrototype) -> Result<csl::Voter, WError> {
    match voter {
        VoterPrototype::ConstitutionalCommitteeHotCred { value: cred } => {
            let credential = proto_to_credential(cred)?;
            Ok(csl::Voter::new_constitutional_committee_hot_credential(
                &credential,
            ))
        }
        VoterPrototype::DRep { value: cred } => {
            let credential = proto_to_credential(cred)?;
            Ok(csl::Voter::new_drep_credential(&credential))
        }
        VoterPrototype::StakingPool {
            value: pool_keyhash,
        } => {
            let keyhash = csl::Ed25519KeyHash::from_hex(pool_keyhash)
                .map_err(WError::from_err("proto_to_voter - invalid pool_keyhash"))?;
            Ok(csl::Voter::new_stake_pool_key_hash(&keyhash))
        }
    }
}

/// Convert GovernanceActionIdPrototype to CSL GovernanceActionId
pub fn proto_to_governance_action_id(
    action_id: &GovernanceActionIdPrototype,
) -> Result<csl::GovernanceActionId, WError> {
    let tx_hash = csl::TransactionHash::from_hex(&action_id.transaction_id).map_err(
        WError::from_err("proto_to_governance_action_id - invalid transaction_id"),
    )?;
    Ok(csl::GovernanceActionId::new(&tx_hash, action_id.index))
}

/// Convert VotingProcedurePrototype to CSL VotingProcedure
pub fn proto_to_voting_procedure(
    procedure: &VotingProcedurePrototype,
) -> Result<csl::VotingProcedure, WError> {
    let vote_kind = proto_to_vote_kind(&procedure.vote);
    if let Some(anchor) = &procedure.anchor {
        Ok(csl::VotingProcedure::new_with_anchor(
            vote_kind,
            &proto_to_anchor(anchor)?,
        ))
    } else {
        Ok(csl::VotingProcedure::new(vote_kind))
    }
}

/// Convert VoterVotesPrototype to CSL VotingProcedures (one voter's votes)
pub fn proto_to_voter_votes(
    voter_votes: &VoterVotesPrototype,
) -> Result<
    (
        csl::Voter,
        Vec<(csl::GovernanceActionId, csl::VotingProcedure)>,
    ),
    WError,
> {
    let voter = proto_to_voter(&voter_votes.voter)?;
    let mut votes = Vec::new();
    for vote in &voter_votes.votes {
        let action_id = proto_to_governance_action_id(&vote.action_id)?;
        let procedure = proto_to_voting_procedure(&vote.voting_procedure)?;
        votes.push((action_id, procedure));
    }
    Ok((voter, votes))
}

/// Convert Vec<VoterVotesPrototype> to CSL VotingProcedures
pub fn proto_to_voting_procedures(
    voter_votes_list: &[VoterVotesPrototype],
) -> Result<csl::VotingProcedures, WError> {
    let mut result = csl::VotingProcedures::new();
    for voter_votes in voter_votes_list {
        let (voter, votes) = proto_to_voter_votes(voter_votes)?;
        for (action_id, procedure) in votes {
            result.insert(&voter, &action_id, &procedure);
        }
    }
    Ok(result)
}

/// Convert GovernanceActionPrototype to CSL GovernanceAction
pub fn proto_to_governance_action(
    action: &GovernanceActionPrototype,
) -> Result<csl::GovernanceAction, WError> {
    match action {
        GovernanceActionPrototype::ParameterChangeAction { value: pca } => {
            let protocol_param_update =
                proto_to_protocol_param_update(&pca.protocol_param_updates)?;
            let gov_action_id = pca
                .gov_action_id
                .as_ref()
                .map(proto_to_governance_action_id)
                .transpose()?;
            let policy_hash = pca
                .policy_hash
                .as_ref()
                .map(|h| {
                    csl::ScriptHash::from_hex(h).map_err(WError::from_err(
                        "proto_to_governance_action - invalid policy_hash",
                    ))
                })
                .transpose()?;

            let pca_csl = match (gov_action_id.as_ref(), policy_hash.as_ref()) {
                (Some(action_id), Some(hash)) => {
                    csl::ParameterChangeAction::new_with_policy_hash_and_action_id(
                        action_id,
                        &protocol_param_update,
                        hash,
                    )
                }
                (Some(action_id), None) => csl::ParameterChangeAction::new_with_action_id(
                    action_id,
                    &protocol_param_update,
                ),
                (None, Some(hash)) => {
                    csl::ParameterChangeAction::new_with_policy_hash(&protocol_param_update, hash)
                }
                (None, None) => csl::ParameterChangeAction::new(&protocol_param_update),
            };
            Ok(csl::GovernanceAction::new_parameter_change_action(&pca_csl))
        }
        GovernanceActionPrototype::HardForkInitiationAction { value: hfia } => {
            let protocol_version =
                csl::ProtocolVersion::new(hfia.protocol_version.major, hfia.protocol_version.minor);
            let gov_action_id = hfia
                .gov_action_id
                .as_ref()
                .map(proto_to_governance_action_id)
                .transpose()?;

            let hfia_csl = match gov_action_id.as_ref() {
                Some(action_id) => {
                    csl::HardForkInitiationAction::new_with_action_id(action_id, &protocol_version)
                }
                None => csl::HardForkInitiationAction::new(&protocol_version),
            };
            Ok(csl::GovernanceAction::new_hard_fork_initiation_action(
                &hfia_csl,
            ))
        }
        GovernanceActionPrototype::TreasuryWithdrawalsAction { value: twa } => {
            let mut withdrawals = csl::TreasuryWithdrawals::new();
            for (addr_str, amount_str) in &twa.withdrawals {
                let reward_address = csl::RewardAddress::from_address(
                    &csl::Address::from_bech32(addr_str).map_err(WError::from_err(
                        "proto_to_governance_action - invalid withdrawal address",
                    ))?,
                )
                .ok_or_else(|| {
                    WError::new("proto_to_governance_action", "invalid reward address")
                })?;
                let amount = proto_to_bignum(amount_str)?;
                withdrawals.insert(&reward_address, &amount);
            }
            let policy_hash = twa
                .policy_hash
                .as_ref()
                .map(|h| {
                    csl::ScriptHash::from_hex(h).map_err(WError::from_err(
                        "proto_to_governance_action - invalid policy_hash",
                    ))
                })
                .transpose()?;

            let twa_csl = match policy_hash.as_ref() {
                Some(hash) => {
                    csl::TreasuryWithdrawalsAction::new_with_policy_hash(&withdrawals, hash)
                }
                None => csl::TreasuryWithdrawalsAction::new(&withdrawals),
            };
            Ok(csl::GovernanceAction::new_treasury_withdrawals_action(
                &twa_csl,
            ))
        }
        GovernanceActionPrototype::NoConfidenceAction { value: nca } => {
            let gov_action_id = nca
                .gov_action_id
                .as_ref()
                .map(proto_to_governance_action_id)
                .transpose()?;

            let nca_csl = match gov_action_id.as_ref() {
                Some(action_id) => csl::NoConfidenceAction::new_with_action_id(action_id),
                None => csl::NoConfidenceAction::new(),
            };
            Ok(csl::GovernanceAction::new_no_confidence_action(&nca_csl))
        }
        GovernanceActionPrototype::UpdateCommitteeAction { value: uca } => {
            let mut members_to_remove = csl::Credentials::new();
            for cred in &uca.members_to_remove {
                members_to_remove.add(&proto_to_credential(cred)?);
            }
            let committee = proto_to_committee(&uca.committee)?;
            let gov_action_id = uca
                .gov_action_id
                .as_ref()
                .map(proto_to_governance_action_id)
                .transpose()?;

            let uca_csl = match gov_action_id.as_ref() {
                Some(action_id) => csl::UpdateCommitteeAction::new_with_action_id(
                    action_id,
                    &committee,
                    &members_to_remove,
                ),
                None => csl::UpdateCommitteeAction::new(&committee, &members_to_remove),
            };
            Ok(csl::GovernanceAction::new_new_committee_action(&uca_csl))
        }
        GovernanceActionPrototype::NewConstitutionAction { value: nca } => {
            let constitution = proto_to_constitution(&nca.constitution)?;
            let gov_action_id = nca
                .gov_action_id
                .as_ref()
                .map(proto_to_governance_action_id)
                .transpose()?;

            let nca_csl = match gov_action_id.as_ref() {
                Some(action_id) => {
                    csl::NewConstitutionAction::new_with_action_id(action_id, &constitution)
                }
                None => csl::NewConstitutionAction::new(&constitution),
            };
            Ok(csl::GovernanceAction::new_new_constitution_action(&nca_csl))
        }
        GovernanceActionPrototype::InfoAction => Ok(csl::GovernanceAction::new_info_action(
            &csl::InfoAction::new(),
        )),
    }
}

fn proto_to_committee(committee: &CommitteePrototype) -> Result<csl::Committee, WError> {
    let quorum = proto_to_unit_interval(&committee.quorum_threshold)?;
    let mut csl_committee = csl::Committee::new(&quorum);
    for member in &committee.members {
        let cred = proto_to_credential(&member.stake_credential)?;
        csl_committee.add_member(&cred, member.term_limit);
    }
    Ok(csl_committee)
}

fn proto_to_constitution(
    constitution: &ConstitutionPrototype,
) -> Result<csl::Constitution, WError> {
    let anchor = proto_to_anchor(&constitution.anchor)?;
    if let Some(script_hash) = &constitution.script_hash {
        let hash = csl::ScriptHash::from_hex(script_hash).map_err(WError::from_err(
            "proto_to_constitution - invalid script_hash",
        ))?;
        Ok(csl::Constitution::new_with_script_hash(&anchor, &hash))
    } else {
        Ok(csl::Constitution::new(&anchor))
    }
}

/// Convert VotingProposalPrototype to CSL VotingProposal
pub fn proto_to_voting_proposal(
    proposal: &VotingProposalPrototype,
) -> Result<csl::VotingProposal, WError> {
    let governance_action = proto_to_governance_action(&proposal.governance_action)?;
    let anchor = proto_to_anchor(&proposal.anchor)?;
    let deposit = proto_to_bignum(&proposal.deposit)?;
    let reward_account = csl::RewardAddress::from_address(
        &csl::Address::from_bech32(&proposal.reward_account).map_err(WError::from_err(
            "proto_to_voting_proposal - invalid reward_account",
        ))?,
    )
    .ok_or_else(|| WError::new("proto_to_voting_proposal", "invalid reward_account"))?;

    Ok(csl::VotingProposal::new(
        &governance_action,
        &anchor,
        &reward_account,
        &deposit,
    ))
}

/// Convert Vec<VotingProposalPrototype> to CSL VotingProposals
pub fn proto_to_voting_proposals(
    proposals: &[VotingProposalPrototype],
) -> Result<csl::VotingProposals, WError> {
    let mut result = csl::VotingProposals::new();
    for proposal in proposals {
        result.add(&proto_to_voting_proposal(proposal)?);
    }
    Ok(result)
}

/// Convert ProtocolParamUpdatePrototype to CSL ProtocolParamUpdate (public for body.rs)
pub fn proto_to_protocol_param_update_from_prototype(
    update: &ProtocolParamUpdatePrototype,
) -> Result<csl::ProtocolParamUpdate, WError> {
    proto_to_protocol_param_update(update)
}

/// Convert ProtocolParamUpdatePrototype to CSL ProtocolParamUpdate
fn proto_to_protocol_param_update(
    update: &ProtocolParamUpdatePrototype,
) -> Result<csl::ProtocolParamUpdate, WError> {
    let mut ppu = csl::ProtocolParamUpdate::new();

    if let Some(minfee_a) = &update.minfee_a {
        ppu.set_minfee_a(&proto_to_bignum(minfee_a)?);
    }
    if let Some(minfee_b) = &update.minfee_b {
        ppu.set_minfee_b(&proto_to_bignum(minfee_b)?);
    }
    if let Some(max_block_body_size) = update.max_block_body_size {
        ppu.set_max_block_body_size(max_block_body_size);
    }
    if let Some(max_tx_size) = update.max_tx_size {
        ppu.set_max_tx_size(max_tx_size);
    }
    if let Some(max_block_header_size) = update.max_block_header_size {
        ppu.set_max_block_header_size(max_block_header_size);
    }
    if let Some(key_deposit) = &update.key_deposit {
        ppu.set_key_deposit(&proto_to_bignum(key_deposit)?);
    }
    if let Some(pool_deposit) = &update.pool_deposit {
        ppu.set_pool_deposit(&proto_to_bignum(pool_deposit)?);
    }
    if let Some(max_epoch) = update.max_epoch {
        ppu.set_max_epoch(max_epoch);
    }
    if let Some(n_opt) = update.n_opt {
        ppu.set_n_opt(n_opt);
    }
    if let Some(pool_pledge_influence) = &update.pool_pledge_influence {
        ppu.set_pool_pledge_influence(&proto_to_unit_interval(pool_pledge_influence)?);
    }
    if let Some(expansion_rate) = &update.expansion_rate {
        ppu.set_expansion_rate(&proto_to_unit_interval(expansion_rate)?);
    }
    if let Some(treasury_growth_rate) = &update.treasury_growth_rate {
        ppu.set_treasury_growth_rate(&proto_to_unit_interval(treasury_growth_rate)?);
    }
    // d and extra_entropy are deprecated in Conway era, skip them
    if let Some(min_pool_cost) = &update.min_pool_cost {
        ppu.set_min_pool_cost(&proto_to_bignum(min_pool_cost)?);
    }
    if let Some(ada_per_utxo_byte) = &update.ada_per_utxo_byte {
        ppu.set_ada_per_utxo_byte(&proto_to_bignum(ada_per_utxo_byte)?);
    }
    if let Some(cost_models) = &update.cost_models {
        let mut costmdls = csl::Costmdls::new();
        for (lang_str, costs) in cost_models {
            let language = match lang_str.as_str() {
                "PlutusV1" => csl::Language::new_plutus_v1(),
                "PlutusV2" => csl::Language::new_plutus_v2(),
                "PlutusV3" => csl::Language::new_plutus_v3(),
                _ => continue,
            };
            let cost_values: Vec<i128> = costs
                .iter()
                .filter_map(|s| s.parse::<i128>().ok())
                .collect();
            let cost_model = csl::CostModel::from(cost_values);
            costmdls.insert(&language, &cost_model);
        }
        ppu.set_cost_models(&costmdls);
    }
    if let Some(execution_costs) = &update.execution_costs {
        ppu.set_execution_costs(&csl::ExUnitPrices::new(
            &proto_to_unit_interval(&execution_costs.mem_price)?,
            &proto_to_unit_interval(&execution_costs.step_price)?,
        ));
    }
    if let Some(max_tx_ex_units) = &update.max_tx_ex_units {
        ppu.set_max_tx_ex_units(&csl::ExUnits::new(
            &proto_to_bignum(&max_tx_ex_units.mem)?,
            &proto_to_bignum(&max_tx_ex_units.steps)?,
        ));
    }
    if let Some(max_block_ex_units) = &update.max_block_ex_units {
        ppu.set_max_block_ex_units(&csl::ExUnits::new(
            &proto_to_bignum(&max_block_ex_units.mem)?,
            &proto_to_bignum(&max_block_ex_units.steps)?,
        ));
    }
    if let Some(max_value_size) = update.max_value_size {
        ppu.set_max_value_size(max_value_size);
    }
    if let Some(collateral_percentage) = update.collateral_percentage {
        ppu.set_collateral_percentage(collateral_percentage);
    }
    if let Some(max_collateral_inputs) = update.max_collateral_inputs {
        ppu.set_max_collateral_inputs(max_collateral_inputs);
    }
    // Conway era params
    if let Some(pool_voting_thresholds) = &update.pool_voting_thresholds {
        ppu.set_pool_voting_thresholds(&csl::PoolVotingThresholds::new(
            &proto_to_unit_interval(&pool_voting_thresholds.motion_no_confidence)?,
            &proto_to_unit_interval(&pool_voting_thresholds.committee_normal)?,
            &proto_to_unit_interval(&pool_voting_thresholds.committee_no_confidence)?,
            &proto_to_unit_interval(&pool_voting_thresholds.hard_fork_initiation)?,
            &proto_to_unit_interval(&pool_voting_thresholds.security_relevant_threshold)?,
        ));
    }
    if let Some(drep_voting_thresholds) = &update.drep_voting_thresholds {
        ppu.set_drep_voting_thresholds(&csl::DRepVotingThresholds::new(
            &proto_to_unit_interval(&drep_voting_thresholds.motion_no_confidence)?,
            &proto_to_unit_interval(&drep_voting_thresholds.committee_normal)?,
            &proto_to_unit_interval(&drep_voting_thresholds.committee_no_confidence)?,
            &proto_to_unit_interval(&drep_voting_thresholds.update_constitution)?,
            &proto_to_unit_interval(&drep_voting_thresholds.hard_fork_initiation)?,
            &proto_to_unit_interval(&drep_voting_thresholds.pp_network_group)?,
            &proto_to_unit_interval(&drep_voting_thresholds.pp_economic_group)?,
            &proto_to_unit_interval(&drep_voting_thresholds.pp_technical_group)?,
            &proto_to_unit_interval(&drep_voting_thresholds.pp_governance_group)?,
            &proto_to_unit_interval(&drep_voting_thresholds.treasury_withdrawal)?,
        ));
    }
    if let Some(min_committee_size) = update.min_committee_size {
        ppu.set_min_committee_size(min_committee_size);
    }
    if let Some(committee_term_limit) = update.committee_term_limit {
        ppu.set_committee_term_limit(committee_term_limit);
    }
    if let Some(governance_action_validity_period) = update.governance_action_validity_period {
        ppu.set_governance_action_validity_period(governance_action_validity_period);
    }
    if let Some(governance_action_deposit) = &update.governance_action_deposit {
        ppu.set_governance_action_deposit(&proto_to_bignum(governance_action_deposit)?);
    }
    if let Some(drep_deposit) = &update.drep_deposit {
        ppu.set_drep_deposit(&proto_to_bignum(drep_deposit)?);
    }
    if let Some(drep_inactivity_period) = update.drep_inactivity_period {
        ppu.set_drep_inactivity_period(drep_inactivity_period);
    }
    if let Some(ref_script_coins_per_byte) = &update.ref_script_coins_per_byte {
        ppu.set_ref_script_coins_per_byte(&proto_to_unit_interval(ref_script_coins_per_byte)?);
    }

    Ok(ppu)
}
