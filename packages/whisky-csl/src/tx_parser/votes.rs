use whisky_common::{
    Anchor, Credential, RefTxIn, ScriptSource, ScriptVote, SimpleScriptSource, SimpleScriptVote,
    Vote, VoteKind, VoteType, Voter, VotingProcedure, WError,
};

use super::{
    context::{ParserContext, RedeemerIndex, Script},
    CSLParser,
};
use cardano_serialization_lib as csl;

impl CSLParser {
    pub fn get_votes(&self) -> &Vec<Vote> {
        &self.tx_body.votes
    }

    pub(super) fn extract_votes(&mut self) -> Result<(), WError> {
        let votes = self.csl_tx_body.voting_procedures();
        if let Some(votes) = votes {
            self.tx_body.votes = csl_votes_to_votes(&votes, &self.context)?;
        }
        Ok(())
    }
}

pub fn csl_votes_to_votes(
    votes: &csl::VotingProcedures,
    context: &ParserContext,
) -> Result<Vec<Vote>, WError> {
    let mut result = Vec::new();
    let voters = votes.get_voters();
    let len = voters.len();

    for i in 0..len {
        let csl_voter = voters.get(i).ok_or_else(|| {
            WError::new(
                "csl_votes_to_votes",
                &format!("Failed to get voter at index {}", i),
            )
        })?;
        let voter_kind = csl_voter.kind();
        let governance_action_ids = votes.get_governance_action_ids_by_voter(&csl_voter);
        let len = governance_action_ids.len();

        let voter_script_hash: Option<csl::ScriptHash>;

        let voter = match voter_kind {
            csl::VoterKind::ConstitutionalCommitteeHotKeyHash => {
                let key_hash = csl_voter
                    .to_constitutional_committee_hot_credential()
                    .ok_or_else(|| {
                        WError::new(
                            "csl_votes_to_votes",
                            "Failed to get constitutional committee hot credential",
                        )
                    })?
                    .to_keyhash()
                    .ok_or_else(|| WError::new("csl_votes_to_votes", "Failed to get key hash"))?;
                voter_script_hash = None;
                Voter::ConstitutionalCommitteeHotCred(Credential::KeyHash(key_hash.to_hex()))
            }
            csl::VoterKind::ConstitutionalCommitteeHotScriptHash => {
                let script_hash = csl_voter
                    .to_constitutional_committee_hot_credential()
                    .ok_or_else(|| {
                        WError::new(
                            "csl_votes_to_votes",
                            "Failed to get constitutional committee hot credential",
                        )
                    })?
                    .to_scripthash()
                    .ok_or_else(|| {
                        WError::new("csl_votes_to_votes", "Failed to get script hash")
                    })?;
                voter_script_hash = Some(script_hash.clone());
                Voter::ConstitutionalCommitteeHotCred(Credential::ScriptHash(script_hash.to_hex()))
            }
            csl::VoterKind::DRepKeyHash => {
                let credential = csl_voter.to_drep_credential().ok_or_else(|| {
                    WError::new("csl_votes_to_votes", "Failed to get drep credential")
                })?;
                let csl_drep_credential = csl::DRep::new_from_credential(&credential);
                voter_script_hash = None;
                Voter::DRepId(csl_drep_credential.to_bech32(true).map_err(|e| {
                    WError::new(
                        "csl_votes_to_votes",
                        &format!("Failed to convert drep to bech32: {:?}", e),
                    )
                })?)
            }
            csl::VoterKind::DRepScriptHash => {
                let credential = csl_voter.to_drep_credential().ok_or_else(|| {
                    WError::new("csl_votes_to_votes", "Failed to get drep credential")
                })?;
                let csl_drep_credential = csl::DRep::new_from_credential(&credential);
                voter_script_hash = csl_drep_credential.to_script_hash();
                Voter::DRepId(csl_drep_credential.to_bech32(true).map_err(|e| {
                    WError::new(
                        "csl_votes_to_votes",
                        &format!("Failed to convert drep to bech32: {:?}", e),
                    )
                })?)
            }
            csl::VoterKind::StakingPoolKeyHash => {
                let key_hash = csl_voter.to_stake_pool_key_hash().ok_or_else(|| {
                    WError::new("csl_votes_to_votes", "Failed to get stake pool key hash")
                })?;
                voter_script_hash = None;
                Voter::StakingPoolKeyHash(key_hash.to_hex())
            }
        };

        for j in 0..len {
            let gov_action_id = governance_action_ids.get(j).ok_or_else(|| {
                WError::new(
                    "csl_votes_to_votes",
                    &format!("Failed to get governance action ID at index {}", j),
                )
            })?;
            let voting_procedure = votes.get(&csl_voter, &gov_action_id).ok_or_else(|| {
                WError::new(
                    "csl_votes_to_votes",
                    &format!(
                        "Failed to get voting procedure for governance action ID: {:?}",
                        gov_action_id
                    ),
                )
            })?;

            let gov_action_id = RefTxIn {
                tx_hash: gov_action_id.transaction_id().to_hex(),
                tx_index: gov_action_id.index(),
                script_size: None,
            };

            let vote_kind = match voting_procedure.vote_kind() {
                csl::VoteKind::No => VoteKind::No,
                csl::VoteKind::Yes => VoteKind::Yes,
                csl::VoteKind::Abstain => VoteKind::Abstain,
            };

            let anchor = if let Some(anchor) = voting_procedure.anchor() {
                Some(Anchor {
                    anchor_url: anchor.url().url(),
                    anchor_data_hash: anchor.anchor_data_hash().to_hex(),
                })
            } else {
                None
            };

            let voting_procedure = VotingProcedure { vote_kind, anchor };

            let vote_type = VoteType {
                voter: voter.clone(),
                gov_action_id,
                voting_procedure,
            };

            if let Some(script_hash) = &voter_script_hash {
                if let Some(script) = context.script_witness.scripts.get(&script_hash) {
                    match script {
                        Script::ProvidedNative(native_script) => {
                            result.push(Vote::SimpleScriptVote(SimpleScriptVote {
                                vote: vote_type,
                                simple_script_source: Some(
                                    SimpleScriptSource::ProvidedSimpleScriptSource(
                                        native_script.clone(),
                                    ),
                                ),
                            }));
                        }
                        Script::ProvidedPlutus(plutus_script) => {
                            let redeemer = context
                                .script_witness
                                .redeemers
                                .get(&RedeemerIndex::Vote(i))
                                .cloned();
                            result.push(Vote::ScriptVote(ScriptVote {
                                vote: vote_type,
                                redeemer,
                                script_source: Some(ScriptSource::ProvidedScriptSource(
                                    plutus_script.clone(),
                                )),
                            }));
                        }
                        Script::ReferencedNative(inline_script) => {
                            result.push(Vote::SimpleScriptVote(SimpleScriptVote {
                                vote: vote_type,
                                simple_script_source: Some(
                                    SimpleScriptSource::InlineSimpleScriptSource(
                                        inline_script.clone(),
                                    ),
                                ),
                            }));
                        }
                        Script::ReferencedPlutus(inline_script) => {
                            let redeemer = context
                                .script_witness
                                .redeemers
                                .get(&RedeemerIndex::Vote(i))
                                .cloned();
                            result.push(Vote::ScriptVote(ScriptVote {
                                vote: vote_type,
                                redeemer,
                                script_source: Some(ScriptSource::InlineScriptSource(
                                    inline_script.clone(),
                                )),
                            }));
                        }
                    }
                } else {
                    result.push(Vote::BasicVote(vote_type));
                }
            } else {
                result.push(Vote::BasicVote(vote_type));
            }
        }
    }
    Ok(result)
}
