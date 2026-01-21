use pallas::ledger::primitives::conway::Tx;
use whisky_common::{
    RefTxIn, ScriptVote, SimpleScriptVote, Vote, VoteKind, VoteType, Voter, VotingProcedure, WError,
};

use crate::{
    tx_parser::context::{ParserContext, RedeemerIndex},
    wrapper::{
        transaction_body::{Anchor, DRep, DRepKind},
        witness_set::redeemer::RedeemerTag,
    },
};

pub fn extract_votes(pallas_tx: &Tx, context: &ParserContext) -> Result<Vec<Vote>, WError> {
    let mut votes: Vec<Vote> = vec![];
    if let Some(pallas_votes) = &pallas_tx.transaction_body.voting_procedures {
        for (index, (voter, procedures)) in pallas_votes.iter().enumerate() {
            match voter {
                pallas::ledger::primitives::conway::Voter::ConstitutionalCommitteeScript(hash) => {
                    let whisky_voter: Voter = Voter::ConstitutionalCommitteeHotCred(
                        whisky_common::Credential::ScriptHash(hash.to_string()),
                    );
                    let script = context.script_witnesses.scripts.get(&hash.to_string());
                    if let Some(script) = script {
                        for (gov_action_id, procedure) in procedures {
                            let gov_action_ref_tx: RefTxIn = RefTxIn {
                                tx_hash: gov_action_id.transaction_id.to_string(),
                                tx_index: gov_action_id.action_index,
                                script_size: Some(0),
                            };
                            let vote_kind: VoteKind = match procedure.vote {
                                pallas::ledger::primitives::conway::Vote::No => VoteKind::No,
                                pallas::ledger::primitives::conway::Vote::Yes => VoteKind::Yes,
                                pallas::ledger::primitives::conway::Vote::Abstain => {
                                    VoteKind::Abstain
                                }
                            };
                            let voting_procedure: VotingProcedure = VotingProcedure {
                                vote_kind,
                                anchor: procedure
                                    .anchor
                                    .clone()
                                    .map(|a| Anchor { inner: a }.to_whisky_anchor()),
                            };
                            match script {
                                crate::tx_parser::context::Script::ProvidedNative(
                                    provided_simple_script_source,
                                ) => {
                                    let vote_type = VoteType {
                                        voter: whisky_voter.clone(),
                                        gov_action_id: gov_action_ref_tx,
                                        voting_procedure,
                                    };
                                    let whisky_vote: Vote = Vote::SimpleScriptVote(SimpleScriptVote {
                                        vote: vote_type,
                                        simple_script_source: Some(whisky_common::SimpleScriptSource::ProvidedSimpleScriptSource(provided_simple_script_source.clone()))
                                    });
                                    votes.push(whisky_vote);
                                }
                                crate::tx_parser::context::Script::ProvidedPlutus(
                                    provided_script_source,
                                ) => {
                                    let redeemer = context
                                        .script_witnesses
                                        .redeemers
                                        .get(&RedeemerIndex {
                                            tag: RedeemerTag::Vote,
                                            index: index as u32,
                                        })
                                        .ok_or(WError::new(
                                            "WhiskyPallas parser - vote redeemer",
                                            &format!("Failed to find redeemer for vote script with hash: {}", hash.to_string()),
                                        ))?;
                                    let vote_type = VoteType {
                                        voter: whisky_voter.clone(),
                                        gov_action_id: gov_action_ref_tx,
                                        voting_procedure,
                                    };
                                    let whisky_vote: Vote = Vote::ScriptVote(ScriptVote {
                                        vote: vote_type,
                                        script_source: Some(
                                            whisky_common::ScriptSource::ProvidedScriptSource(
                                                provided_script_source.clone(),
                                            ),
                                        ),
                                        redeemer: Some(redeemer.clone()),
                                    });
                                    votes.push(whisky_vote);
                                }
                                crate::tx_parser::context::Script::ReferencedNative(
                                    inline_simple_script_source,
                                ) => {
                                    let vote_type = VoteType {
                                        voter: whisky_voter.clone(),
                                        gov_action_id: gov_action_ref_tx,
                                        voting_procedure,
                                    };
                                    let whisky_vote: Vote = Vote::SimpleScriptVote(SimpleScriptVote {
                                        vote: vote_type,
                                        simple_script_source: Some(whisky_common::SimpleScriptSource::InlineSimpleScriptSource(inline_simple_script_source.clone()))
                                    });
                                    votes.push(whisky_vote);
                                }
                                crate::tx_parser::context::Script::ReferencedPlutus(
                                    inline_script_source,
                                ) => {
                                    let redeemer = context
                                        .script_witnesses
                                        .redeemers
                                        .get(&RedeemerIndex {
                                            tag: RedeemerTag::Vote,
                                            index: index as u32,
                                        })
                                        .ok_or(WError::new(
                                            "WhiskyPallas parser - vote redeemer",
                                            &format!("Failed to find redeemer for vote script with hash: {}", hash.to_string()),
                                        ))?;
                                    let vote_type = VoteType {
                                        voter: whisky_voter.clone(),
                                        gov_action_id: gov_action_ref_tx,
                                        voting_procedure,
                                    };
                                    let whisky_vote: Vote = Vote::ScriptVote(ScriptVote {
                                        vote: vote_type,
                                        script_source: Some(
                                            whisky_common::ScriptSource::InlineScriptSource(
                                                inline_script_source.clone(),
                                            ),
                                        ),
                                        redeemer: Some(redeemer.clone()),
                                    });
                                    votes.push(whisky_vote);
                                }
                            }
                        }
                    } else {
                        return Err(WError::new(
                            "WhiskyPallas Parser - extract_votes",
                            &format!(
                                "Failed to find script for voter script hash: {}",
                                hash.to_string()
                            ),
                        ));
                    }
                }
                pallas::ledger::primitives::conway::Voter::ConstitutionalCommitteeKey(hash) => {
                    let whisky_voter: Voter = Voter::ConstitutionalCommitteeHotCred(
                        whisky_common::Credential::KeyHash(hash.to_string()),
                    );
                    for (gov_action_id, procedure) in procedures {
                        let gov_action_ref_tx: RefTxIn = RefTxIn {
                            tx_hash: gov_action_id.transaction_id.to_string(),
                            tx_index: gov_action_id.action_index,
                            script_size: Some(0),
                        };
                        let vote_kind: VoteKind = match procedure.vote {
                            pallas::ledger::primitives::conway::Vote::No => VoteKind::No,
                            pallas::ledger::primitives::conway::Vote::Yes => VoteKind::Yes,
                            pallas::ledger::primitives::conway::Vote::Abstain => VoteKind::Abstain,
                        };
                        let voting_procedure: VotingProcedure = VotingProcedure {
                            vote_kind,
                            anchor: procedure
                                .anchor
                                .clone()
                                .map(|a| Anchor { inner: a }.to_whisky_anchor()),
                        };
                        let vote_type: VoteType = VoteType {
                            voter: whisky_voter.clone(),
                            gov_action_id: gov_action_ref_tx,
                            voting_procedure,
                        };
                        let whisky_vote: Vote = Vote::BasicVote(vote_type);
                        votes.push(whisky_vote);
                    }
                }
                pallas::ledger::primitives::conway::Voter::DRepScript(hash) => {
                    let whisky_voter: Voter = Voter::DRepId(
                        DRep::new(DRepKind::Script {
                            script_hash: hash.to_string(),
                        })?
                        .to_bech32_cip129()?,
                    );
                    let script = context.script_witnesses.scripts.get(&hash.to_string());
                    if let Some(script) = script {
                        for (gov_action_id, procedure) in procedures {
                            let gov_action_ref_tx: RefTxIn = RefTxIn {
                                tx_hash: gov_action_id.transaction_id.to_string(),
                                tx_index: gov_action_id.action_index,
                                script_size: Some(0),
                            };
                            let vote_kind: VoteKind = match procedure.vote {
                                pallas::ledger::primitives::conway::Vote::No => VoteKind::No,
                                pallas::ledger::primitives::conway::Vote::Yes => VoteKind::Yes,
                                pallas::ledger::primitives::conway::Vote::Abstain => {
                                    VoteKind::Abstain
                                }
                            };
                            let voting_procedure: VotingProcedure = VotingProcedure {
                                vote_kind,
                                anchor: procedure
                                    .anchor
                                    .clone()
                                    .map(|a| Anchor { inner: a }.to_whisky_anchor()),
                            };
                            match script {
                                crate::tx_parser::context::Script::ProvidedNative(
                                    provided_simple_script_source,
                                ) => {
                                    let vote_type = VoteType {
                                        voter: whisky_voter.clone(),
                                        gov_action_id: gov_action_ref_tx,
                                        voting_procedure,
                                    };
                                    let whisky_vote: Vote = Vote::SimpleScriptVote(SimpleScriptVote {
                                        vote: vote_type,
                                        simple_script_source: Some(whisky_common::SimpleScriptSource::ProvidedSimpleScriptSource(provided_simple_script_source.clone()))
                                    });
                                    votes.push(whisky_vote);
                                }
                                crate::tx_parser::context::Script::ProvidedPlutus(
                                    provided_script_source,
                                ) => {
                                    let redeemer = context
                                        .script_witnesses
                                        .redeemers
                                        .get(&RedeemerIndex {
                                            tag: RedeemerTag::Vote,
                                            index: index as u32,
                                        })
                                        .ok_or(WError::new(
                                            "WhiskyPallas parser - vote redeemer",
                                            &format!("Failed to find redeemer for vote script with hash: {}", hash.to_string()),
                                        ))?;
                                    let vote_type = VoteType {
                                        voter: whisky_voter.clone(),
                                        gov_action_id: gov_action_ref_tx,
                                        voting_procedure,
                                    };
                                    let whisky_vote: Vote = Vote::ScriptVote(ScriptVote {
                                        vote: vote_type,
                                        script_source: Some(
                                            whisky_common::ScriptSource::ProvidedScriptSource(
                                                provided_script_source.clone(),
                                            ),
                                        ),
                                        redeemer: Some(redeemer.clone()),
                                    });
                                    votes.push(whisky_vote);
                                }
                                crate::tx_parser::context::Script::ReferencedNative(
                                    inline_simple_script_source,
                                ) => {
                                    let vote_type = VoteType {
                                        voter: whisky_voter.clone(),
                                        gov_action_id: gov_action_ref_tx,
                                        voting_procedure,
                                    };
                                    let whisky_vote: Vote = Vote::SimpleScriptVote(SimpleScriptVote {
                                        vote: vote_type,
                                        simple_script_source: Some(whisky_common::SimpleScriptSource::InlineSimpleScriptSource(inline_simple_script_source.clone()))
                                    });
                                    votes.push(whisky_vote);
                                }
                                crate::tx_parser::context::Script::ReferencedPlutus(
                                    inline_script_source,
                                ) => {
                                    let redeemer = context
                                        .script_witnesses
                                        .redeemers
                                        .get(&RedeemerIndex {
                                            tag: RedeemerTag::Vote,
                                            index: index as u32,
                                        })
                                        .ok_or(WError::new(
                                            "WhiskyPallas parser - vote redeemer",
                                            &format!("Failed to find redeemer for vote script with hash: {}", hash.to_string()),
                                        ))?;
                                    let vote_type = VoteType {
                                        voter: whisky_voter.clone(),
                                        gov_action_id: gov_action_ref_tx,
                                        voting_procedure,
                                    };
                                    let whisky_vote: Vote = Vote::ScriptVote(ScriptVote {
                                        vote: vote_type,
                                        script_source: Some(
                                            whisky_common::ScriptSource::InlineScriptSource(
                                                inline_script_source.clone(),
                                            ),
                                        ),
                                        redeemer: Some(redeemer.clone()),
                                    });
                                    votes.push(whisky_vote);
                                }
                            }
                        }
                    } else {
                        return Err(WError::new(
                            "WhiskyPallas Parser - extract_votes",
                            &format!(
                                "Failed to find script for voter script hash: {}",
                                hash.to_string()
                            ),
                        ));
                    }
                }
                pallas::ledger::primitives::conway::Voter::DRepKey(hash) => {
                    let whisky_voter: Voter = Voter::DRepId(
                        DRep::new(DRepKind::Key {
                            addr_key_hash: hash.to_string(),
                        })?
                        .to_bech32_cip129()?,
                    );

                    for (gov_action_id, procedure) in procedures {
                        let gov_action_ref_tx: RefTxIn = RefTxIn {
                            tx_hash: gov_action_id.transaction_id.to_string(),
                            tx_index: gov_action_id.action_index,
                            script_size: Some(0),
                        };
                        let vote_kind: VoteKind = match procedure.vote {
                            pallas::ledger::primitives::conway::Vote::No => VoteKind::No,
                            pallas::ledger::primitives::conway::Vote::Yes => VoteKind::Yes,
                            pallas::ledger::primitives::conway::Vote::Abstain => VoteKind::Abstain,
                        };
                        let voting_procedure: VotingProcedure = VotingProcedure {
                            vote_kind,
                            anchor: procedure
                                .anchor
                                .clone()
                                .map(|a| Anchor { inner: a }.to_whisky_anchor()),
                        };
                        let vote_type: VoteType = VoteType {
                            voter: whisky_voter.clone(),
                            gov_action_id: gov_action_ref_tx,
                            voting_procedure,
                        };
                        let whisky_vote: Vote = Vote::BasicVote(vote_type);
                        votes.push(whisky_vote);
                    }
                }
                pallas::ledger::primitives::conway::Voter::StakePoolKey(hash) => {
                    let whisky_voter: Voter = Voter::StakingPoolKeyHash(hash.to_string());
                    for (gov_action_id, procedure) in procedures {
                        let gov_action_ref_tx: RefTxIn = RefTxIn {
                            tx_hash: gov_action_id.transaction_id.to_string(),
                            tx_index: gov_action_id.action_index,
                            script_size: Some(0),
                        };
                        let vote_kind: VoteKind = match procedure.vote {
                            pallas::ledger::primitives::conway::Vote::No => VoteKind::No,
                            pallas::ledger::primitives::conway::Vote::Yes => VoteKind::Yes,
                            pallas::ledger::primitives::conway::Vote::Abstain => VoteKind::Abstain,
                        };
                        let voting_procedure: VotingProcedure = VotingProcedure {
                            vote_kind,
                            anchor: procedure
                                .anchor
                                .clone()
                                .map(|a| Anchor { inner: a }.to_whisky_anchor()),
                        };
                        let vote_type: VoteType = VoteType {
                            voter: whisky_voter.clone(),
                            gov_action_id: gov_action_ref_tx,
                            voting_procedure,
                        };
                        let whisky_vote: Vote = Vote::BasicVote(vote_type);
                        votes.push(whisky_vote);
                    }
                }
            }
        }
    }
    Ok(Vec::new())
}
