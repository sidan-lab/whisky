use serde::{Deserialize, Serialize};

use super::{Anchor, Credential, Redeemer, RefTxIn, ScriptSource, SimpleScriptSource};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Vote {
    BasicVote(VoteType),
    ScriptVote(ScriptVote),
    SimpleScriptVote(SimpleScriptVote),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScriptVote {
    pub vote: VoteType,
    pub redeemer: Option<Redeemer>,
    pub script_source: Option<ScriptSource>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleScriptVote {
    pub vote: VoteType,
    pub simple_script_source: Option<SimpleScriptSource>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VoteType {
    pub voter: Voter,
    pub gov_action_id: RefTxIn,
    pub voting_procedure: VotingProcedure,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Voter {
    ConstitutionalCommitteeHotCred(Credential),
    DRepId(String),
    StakingPoolKeyHash(String),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VotingProcedure {
    pub vote_kind: VoteKind,
    pub anchor: Option<Anchor>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum VoteKind {
    No = 0,
    Yes = 1,
    Abstain = 2,
}
