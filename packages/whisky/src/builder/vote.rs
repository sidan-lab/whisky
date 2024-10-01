use sidan_csl_rs::model::*;

use super::{TxBuilder, Vote, WRedeemer};

impl TxBuilder {
    /// ## Transaction building method
    ///
    /// Indicate that the transaction is voting using a plutus staking script in the TxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `language_version` - The language version of the script
    ///
    /// ### Returns
    ///
    /// * `Self` - The TxBuilder instance
    pub fn voting_plutus_script(&mut self, language_version: &LanguageVersion) -> &mut Self {
        match language_version {
            LanguageVersion::V1 => self.voting_plutus_script_v1(),
            LanguageVersion::V2 => self.voting_plutus_script_v2(),
            LanguageVersion::V3 => self.voting_plutus_script_v3(),
        }
    }

    /// ## Transaction building method
    ///
    /// Indicate that the transaction is voting using a plutus V1 staking script in the TxBuilder instance
    ///
    /// ### Returns
    ///
    /// * `Self` - The TxBuilder instance
    pub fn voting_plutus_script_v1(&mut self) -> &mut Self {
        self.adding_plutus_vote = Some(LanguageVersion::V1);
        self
    }

    /// ## Transaction building method
    ///
    /// Indicate that the transaction is voting using a plutus V2 staking script in the TxBuilder instance
    ///
    /// ### Returns
    ///
    /// * `Self` - The TxBuilder instance
    pub fn voting_plutus_script_v2(&mut self) -> &mut Self {
        self.adding_plutus_vote = Some(LanguageVersion::V2);
        self
    }

    /// ## Transaction building method
    ///
    /// Indicate that the transaction is voting using a plutus V3 staking script in the TxBuilder instance
    ///
    /// ### Returns
    ///
    /// * `Self` - The TxBuilder instance
    pub fn voting_plutus_script_v3(&mut self) -> &mut Self {
        self.adding_plutus_vote = Some(LanguageVersion::V3);
        self
    }

    /// ## Transaction building method
    ///
    /// Add a vote reference to the TxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `tx_hash` - The transaction hash
    /// * `tx_index` - The transaction index
    /// * `vote_script_hash` - The vote script hash
    /// * `version` - The language version, if the language version is None, the script is assumed to be a Native Script
    /// * `script_size` - Size of the script
    ///
    /// ### Returns
    ///
    /// * `Self` - The TxBuilder instance
    pub fn vote_tx_in_reference(
        &mut self,
        tx_hash: &str,
        tx_index: u32,
        vote_script_hash: &str,
        script_size: usize,
    ) -> &mut Self {
        let vote_item = self.vote_item.take();
        if vote_item.is_none() {
            panic!("Undefined output")
        }
        let vote_item = vote_item.unwrap();
        match vote_item {
            Vote::BasicVote(_) => {
                panic!("Script reference cannot be defined for a pubkey vote")
            }
            Vote::SimpleScriptVote(mut simple_script_vote) => {
                simple_script_vote.simple_script_source = Some(
                    SimpleScriptSource::InlineSimpleScriptSource(InlineSimpleScriptSource {
                        ref_tx_in: RefTxIn {
                            tx_hash: tx_hash.to_string(),
                            tx_index,
                        },
                        simple_script_hash: vote_script_hash.to_string(),
                        script_size,
                    }),
                )
            }
            Vote::ScriptVote(mut script_vote) => {
                script_vote.script_source =
                    Some(ScriptSource::InlineScriptSource(InlineScriptSource {
                        ref_tx_in: RefTxIn {
                            tx_hash: tx_hash.to_string(),
                            tx_index,
                        },
                        script_hash: vote_script_hash.to_string(),
                        language_version: self
                            .adding_plutus_vote
                            .clone()
                            .expect("Plutus votes require a language version"),
                        script_size,
                    }));
                self.vote_item = Some(Vote::ScriptVote(script_vote));
            }
        }
        self
    }

    /// ## Transaction building method
    ///
    /// Add a vote in the TxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `voter` - The voter, can be a ConstitutionalCommittee, a DRep or a StakePool
    /// * `gov_action_id` - The transaction hash and transaction id of the governance action
    /// * `voting_precedure` - The voting kind (yes, no, abstain) with an optional anchor
    ///
    /// ### Returns
    ///
    /// * `Self` - The TxBuilder instance
    pub fn vote(
        &mut self,
        voter: &Voter,
        gov_action_id: &RefTxIn,
        voting_procedure: &VotingProcedure,
    ) -> &mut Self {
        if self.vote_item.is_some() {
            self.queue_vote();
        }

        match self.adding_plutus_vote {
            Some(_) => {
                let vote_item = Vote::ScriptVote(ScriptVote {
                    vote: VoteType {
                        voter: voter.clone(),
                        gov_action_id: gov_action_id.clone(),
                        voting_procedure: voting_procedure.clone(),
                    },
                    redeemer: None,
                    script_source: None,
                });
                self.vote_item = Some(vote_item);
            }
            None => {
                let vote_item = Vote::BasicVote(VoteType {
                    voter: voter.clone(),
                    gov_action_id: gov_action_id.clone(),
                    voting_procedure: voting_procedure.clone(),
                });
                self.vote_item = Some(vote_item);
            }
        }
        self
    }

    /// ## Transaction building method
    ///
    /// Add a vote script to the TxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `script_cbor` - The script in CBOR format
    ///
    /// ### Returns
    ///
    /// * `Self` - The TxBuilder instance
    pub fn vote_script(&mut self, script_cbor: &str) -> &mut Self {
        let vote_item = self.vote_item.take();
        if vote_item.is_none() {
            panic!("Undefined vote")
        }
        let vote_item = vote_item.unwrap();
        match vote_item {
            Vote::BasicVote(_) => {
                panic!("Script reference cannot be defined for a pubkey vote")
            }
            Vote::SimpleScriptVote(mut simple_script_vote) => {
                simple_script_vote.simple_script_source = Some(
                    SimpleScriptSource::ProvidedSimpleScriptSource(ProvidedSimpleScriptSource {
                        script_cbor: script_cbor.to_string(),
                    }),
                );
                self.vote_item = Some(Vote::SimpleScriptVote(simple_script_vote));
            }
            Vote::ScriptVote(mut script_vote) => {
                script_vote.script_source =
                    Some(ScriptSource::ProvidedScriptSource(ProvidedScriptSource {
                        script_cbor: script_cbor.to_string(),
                        language_version: self
                            .adding_plutus_vote
                            .clone()
                            .expect("Plutus votes require a language version"),
                    }));
                self.vote_item = Some(Vote::ScriptVote(script_vote));
                self.adding_plutus_vote = None;
            }
        }
        self
    }

    /// ## Transaction building method
    ///
    /// Set the transaction vote redeemer value in the TxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `redeemer` - The redeemer value
    ///
    /// ### Returns
    ///
    /// * `Self` - The TxBuilder instance
    pub fn vote_redeemer_value(&mut self, redeemer: &WRedeemer) -> &mut Self {
        let vote_item = self.vote_item.take();
        if vote_item.is_none() {
            panic!("Undefined input")
        }
        let vote_item = vote_item.unwrap();
        match vote_item {
            Vote::BasicVote(_) => {
                panic!("Redeemer cannot be defined for a basic vote")
            }
            Vote::SimpleScriptVote(_) => {
                panic!("Redeemer cannot be defined for a native script vote")
            }
            Vote::ScriptVote(mut script_vote) => match redeemer.data.to_cbor() {
                Ok(raw_redeemer) => {
                    script_vote.redeemer = Some(Redeemer {
                        data: raw_redeemer,
                        ex_units: redeemer.clone().ex_units,
                    });
                    self.vote_item = Some(Vote::ScriptVote(script_vote));
                }
                Err(_) => panic!("Error converting redeemer to CBOR"),
            },
        }
        self
    }

    /// ## Transaction building method
    ///
    /// Set the vote reference redeemer value in the TxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `redeemer` - The redeemer value
    ///
    /// ### Returns
    ///
    /// * `Self` - The TxBuilder instance
    pub fn vote_reference_tx_in_redeemer_value(&mut self, redeemer: &WRedeemer) -> &mut Self {
        self.vote_redeemer_value(redeemer)
    }
}
