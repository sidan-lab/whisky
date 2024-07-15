use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Certificate {
    RegisterStake(RegisterStake),
    DeregisterStake(DeregisterStake),
    DelegateStake(DelegateStake),
    RegisterPool(RegisterPool),
    RetirePool(RetirePool),
    VoteDelegation(VoteDelegation),
    StakeAndVoteDelegation(StakeAndVoteDelegation),
    StakeRegistrationAndDelegation(StakeRegistrationAndDelegation),
    VoteRegistrationAndDelegation(VoteRegistrationAndDelegation),
    StakeVoteRegistrationAndDelegation(StakeVoteRegistrationAndDelegation),
    CommitteeHotAuth(CommitteeHotAuth),
    CommitteeColdResign(CommitteeColdResign),
    DRepRegistration(DRepRegistration),
    DRepDeregistration(DRepDeregistration),
    DRepUpdate(DRepUpdate),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterPool {
    pub pool_params: PoolParams,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PoolParams {
    pub vrf_key_hash: String,
    pub operator: String,
    pub pledge: String,
    pub cost: String,
    pub margin: (u64, u64),
    pub relays: Vec<Relay>,
    pub owners: Vec<String>,
    pub reward_address: String,
    pub metadata: Option<PoolMetadata>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Relay {
    SingleHostAddr(SingleHostAddr),
    SingleHostName(SingleHostName),
    MultiHostName(MultiHostName),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SingleHostAddr {
    pub ipv4: Option<String>,
    pub ipv6: Option<String>,
    pub port: Option<u16>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SingleHostName {
    pub domain_name: String,
    pub port: Option<u16>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultiHostName {
    pub domain_name: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PoolMetadata {
    pub url: String,
    pub hash: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterStake {
    pub stake_key_hash: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DelegateStake {
    pub stake_key_hash: String,
    pub pool_id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeregisterStake {
    pub stake_key_hash: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RetirePool {
    pub pool_id: String,
    pub epoch: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VoteDelegation {
    pub stake_key_hash: String,
    pub drep: DRep,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DRep {
    KeyHash(String),
    ScriptHash(String),
    AlwaysAbstain,
    AlwaysNoConfidence,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StakeAndVoteDelegation {
    pub stake_key_hash: String,
    pub pool_key_hash: String,
    pub drep: DRep,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StakeRegistrationAndDelegation {
    pub stake_key_hash: String,
    pub pool_key_hash: String,
    pub coin: u64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VoteRegistrationAndDelegation {
    pub stake_key_hash: String,
    pub drep: DRep,
    pub coin: u64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StakeVoteRegistrationAndDelegation {
    pub stake_key_hash: String,
    pub pool_key_hash: String,
    pub drep: DRep,
    pub coin: u64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommitteeHotAuth {
    pub committee_cold_key_hash: String,
    pub committee_hot_key_hash: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommitteeColdResign {
    pub committee_cold_key_hash: String,
    pub anchor: Option<Anchor>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Anchor {
    pub anchor_url: String,
    pub anchor_data_hash: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DRepRegistration {
    pub voting_key_hash: String,
    pub coin: u64,
    pub anchor: Option<Anchor>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DRepDeregistration {
    pub voting_key_hash: String,
    pub coin: u64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DRepUpdate {
    pub voting_key_hash: String,
    pub anchor: Option<Anchor>,
}