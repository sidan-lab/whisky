use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// Basic Type Aliases
// ============================================================================

pub type AddressPrototype = String;
pub type URLPrototype = String;
pub type AnchorDataHashPrototype = String;
pub type AssetNamePrototype = String;
pub type AssetNamesPrototype = Vec<String>;
pub type AuxiliaryDataHashPrototype = String;
pub type BigIntPrototype = String;
pub type BigNumPrototype = String;
pub type VkeyPrototype = String;
pub type DNSRecordAorAAAAPrototype = String;
pub type DNSRecordSRVPrototype = String;
pub type BlockHashPrototype = String;
pub type DataHashPrototype = String;
pub type Ed25519KeyHashPrototype = String;
pub type Ed25519KeyHashesPrototype = Vec<String>;
pub type Ed25519SignaturePrototype = String;
pub type GenesisDelegateHashPrototype = String;
pub type GenesisHashPrototype = String;
pub type GenesisHashesPrototype = Vec<String>;
pub type IntPrototype = String;
pub type KESVKeyPrototype = String;
pub type PoolMetadataHashPrototype = String;
pub type PublicKeyPrototype = String;
pub type RewardAddressPrototype = String;
pub type RewardAddressesPrototype = Vec<String>;
pub type ScriptDataHashPrototype = String;
pub type ScriptHashPrototype = String;
pub type ScriptHashesPrototype = Vec<String>;
pub type TransactionHashPrototype = String;
pub type PlutusScriptPrototype = String;
pub type PlutusScriptsPrototype = Vec<String>;
pub type CostModelPrototype = Vec<String>;

// ============================================================================
// IPv4 and IPv6 Types
// ============================================================================

pub type Ipv4Prototype = [u8; 4];
pub type Ipv6Prototype = [u8; 16];

// ============================================================================
// Nonce Hash (32 bytes)
// ============================================================================

pub type NonceHashPrototype = [u8; 32];

// ============================================================================
// Anchor
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AnchorPrototype {
    pub anchor_data_hash: String,
    pub anchor_url: URLPrototype,
}

// ============================================================================
// Assets and MultiAsset
// ============================================================================

pub type AssetsPrototype = HashMap<String, String>;
pub type MultiAssetPrototype = HashMap<String, AssetsPrototype>;
pub type MintPrototype = MultiAssetPrototype;

// ============================================================================
// Native Scripts
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum NativeScriptPrototype {
    #[serde(rename = "SCRIPT_PUBKEY")]
    ScriptPubkey { value: ScriptPubkeyPrototype },
    #[serde(rename = "SCRIPT_ALL")]
    ScriptAll { value: ScriptAllPrototype },
    #[serde(rename = "SCRIPT_ANY")]
    ScriptAny { value: ScriptAnyPrototype },
    #[serde(rename = "SCRIPT_N_OF_K")]
    ScriptNOfK { value: ScriptNOfKPrototype },
    #[serde(rename = "TIMELOCK_START")]
    TimelockStart { value: TimelockStartPrototype },
    #[serde(rename = "TIMELOCK_EXPIRY")]
    TimelockExpiry { value: TimelockExpiryPrototype },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScriptPubkeyPrototype {
    pub addr_keyhash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScriptAllPrototype {
    pub native_scripts: Vec<NativeScriptPrototype>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScriptAnyPrototype {
    pub native_scripts: Vec<NativeScriptPrototype>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScriptNOfKPrototype {
    pub n: u32,
    pub native_scripts: Vec<NativeScriptPrototype>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelockStartPrototype {
    pub slot: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelockExpiryPrototype {
    pub slot: String,
}

pub type NativeScriptsPrototype = Vec<NativeScriptPrototype>;

// ============================================================================
// Credential Types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum CredTypePrototype {
    #[serde(rename = "SCRIPT")]
    Script { value: String },
    #[serde(rename = "KEY")]
    Key { value: String },
}

pub type CredentialPrototype = CredTypePrototype;
pub type CredentialsPrototype = Vec<CredTypePrototype>;

// ============================================================================
// Relay Types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum RelayPrototype {
    #[serde(rename = "SINGLE_HOST_ADDR")]
    SingleHostAddr { value: SingleHostAddrPrototype },
    #[serde(rename = "SINGLE_HOST_NAME")]
    SingleHostName { value: SingleHostNamePrototype },
    #[serde(rename = "MULTI_HOST_NAME")]
    MultiHostName { value: MultiHostNamePrototype },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SingleHostAddrPrototype {
    pub ipv4: Option<Ipv4Prototype>,
    pub ipv6: Option<Ipv6Prototype>,
    pub port: Option<u16>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SingleHostNamePrototype {
    pub dns_name: DNSRecordAorAAAAPrototype,
    pub port: Option<u16>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MultiHostNamePrototype {
    pub dns_name: DNSRecordSRVPrototype,
}

pub type RelaysPrototype = Vec<RelayPrototype>;

// ============================================================================
// Unit Interval
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UnitIntervalPrototype {
    pub denominator: String,
    pub numerator: String,
}

// ============================================================================
// Protocol Version
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProtocolVersionPrototype {
    pub major: u32,
    pub minor: u32,
}

// ============================================================================
// Pool Types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PoolMetadataPrototype {
    pub pool_metadata_hash: String,
    pub url: URLPrototype,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PoolParamsPrototype {
    pub cost: String,
    pub margin: UnitIntervalPrototype,
    pub operator: String,
    pub pledge: String,
    pub pool_metadata: Option<PoolMetadataPrototype>,
    pub pool_owners: Vec<String>,
    pub relays: RelaysPrototype,
    pub reward_account: String,
    pub vrf_keyhash: String,
}

// ============================================================================
// MIR Types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum MIRPotPrototype {
    #[serde(rename = "RESERVES")]
    Reserves,
    #[serde(rename = "TREASURY")]
    Treasury,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StakeToCoinPrototype {
    pub amount: String,
    pub stake_cred: CredTypePrototype,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum MIREnumPrototype {
    #[serde(rename = "TO_OTHER_POT")]
    ToOtherPot { value: String },
    #[serde(rename = "TO_STAKE_CREDENTIALS")]
    ToStakeCredentials { value: Vec<StakeToCoinPrototype> },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MoveInstantaneousRewardPrototype {
    pub pot: MIRPotPrototype,
    pub variant: MIREnumPrototype,
}

pub type MIRToStakeCredentialsPrototype = Vec<StakeToCoinPrototype>;

// ============================================================================
// DRep Types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum DRepPrototype {
    #[serde(rename = "ALWAYS_ABSTAIN")]
    AlwaysAbstain,
    #[serde(rename = "ALWAYS_NO_CONFIDENCE")]
    AlwaysNoConfidence,
    #[serde(rename = "KEY_HASH")]
    KeyHash { value: String },
    #[serde(rename = "SCRIPT_HASH")]
    ScriptHash { value: String },
}

// ============================================================================
// Certificates
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StakeRegistrationPrototype {
    pub coin: Option<String>,
    pub stake_credential: CredTypePrototype,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StakeDeregistrationPrototype {
    pub coin: Option<String>,
    pub stake_credential: CredTypePrototype,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StakeDelegationPrototype {
    pub pool_keyhash: String,
    pub stake_credential: CredTypePrototype,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PoolRegistrationPrototype {
    pub pool_params: PoolParamsPrototype,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PoolRetirementPrototype {
    pub epoch: u32,
    pub pool_keyhash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GenesisKeyDelegationPrototype {
    pub genesis_delegate_hash: String,
    pub genesishash: String,
    pub vrf_keyhash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MoveInstantaneousRewardsCertPrototype {
    pub move_instantaneous_reward: MoveInstantaneousRewardPrototype,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CommitteeHotAuthPrototype {
    pub committee_cold_credential: CredTypePrototype,
    pub committee_hot_credential: CredTypePrototype,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CommitteeColdResignPrototype {
    pub anchor: Option<AnchorPrototype>,
    pub committee_cold_credential: CredTypePrototype,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DRepDeregistrationPrototype {
    pub coin: String,
    pub voting_credential: CredTypePrototype,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DRepRegistrationPrototype {
    pub anchor: Option<AnchorPrototype>,
    pub coin: String,
    pub voting_credential: CredTypePrototype,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DRepUpdatePrototype {
    pub anchor: Option<AnchorPrototype>,
    pub voting_credential: CredTypePrototype,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StakeAndVoteDelegationPrototype {
    pub drep: DRepPrototype,
    pub pool_keyhash: String,
    pub stake_credential: CredTypePrototype,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StakeRegistrationAndDelegationPrototype {
    pub coin: String,
    pub pool_keyhash: String,
    pub stake_credential: CredTypePrototype,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StakeVoteRegistrationAndDelegationPrototype {
    pub coin: String,
    pub drep: DRepPrototype,
    pub pool_keyhash: String,
    pub stake_credential: CredTypePrototype,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VoteDelegationPrototype {
    pub drep: DRepPrototype,
    pub stake_credential: CredTypePrototype,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VoteRegistrationAndDelegationPrototype {
    pub coin: String,
    pub drep: DRepPrototype,
    pub stake_credential: CredTypePrototype,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum CertificatePrototype {
    #[serde(rename = "STAKE_REGISTRATION")]
    StakeRegistration { value: StakeRegistrationPrototype },
    #[serde(rename = "STAKE_DEREGISTRATION")]
    StakeDeregistration { value: StakeDeregistrationPrototype },
    #[serde(rename = "STAKE_DELEGATION")]
    StakeDelegation { value: StakeDelegationPrototype },
    #[serde(rename = "POOL_REGISTRATION")]
    PoolRegistration { value: PoolRegistrationPrototype },
    #[serde(rename = "POOL_RETIREMENT")]
    PoolRetirement { value: PoolRetirementPrototype },
    #[serde(rename = "GENESIS_KEY_DELEGATION")]
    GenesisKeyDelegation {
        value: GenesisKeyDelegationPrototype,
    },
    #[serde(rename = "MOVE_INSTANTANEOUS_REWARDS_CERT")]
    MoveInstantaneousRewardsCert {
        value: MoveInstantaneousRewardsCertPrototype,
    },
    #[serde(rename = "COMMITTEE_HOT_AUTH")]
    CommitteeHotAuth { value: CommitteeHotAuthPrototype },
    #[serde(rename = "COMMITTEE_COLD_RESIGN")]
    CommitteeColdResign { value: CommitteeColdResignPrototype },
    #[serde(rename = "DREP_DEREGISTRATION")]
    DRepDeregistration { value: DRepDeregistrationPrototype },
    #[serde(rename = "DREP_REGISTRATION")]
    DRepRegistration { value: DRepRegistrationPrototype },
    #[serde(rename = "DREP_UPDATE")]
    DRepUpdate { value: DRepUpdatePrototype },
    #[serde(rename = "STAKE_AND_VOTE_DELEGATION")]
    StakeAndVoteDelegation {
        value: StakeAndVoteDelegationPrototype,
    },
    #[serde(rename = "STAKE_REGISTRATION_AND_DELEGATION")]
    StakeRegistrationAndDelegation {
        value: StakeRegistrationAndDelegationPrototype,
    },
    #[serde(rename = "STAKE_VOTE_REGISTRATION_AND_DELEGATION")]
    StakeVoteRegistrationAndDelegation {
        value: StakeVoteRegistrationAndDelegationPrototype,
    },
    #[serde(rename = "VOTE_DELEGATION")]
    VoteDelegation { value: VoteDelegationPrototype },
    #[serde(rename = "VOTE_REGISTRATION_AND_DELEGATION")]
    VoteRegistrationAndDelegation {
        value: VoteRegistrationAndDelegationPrototype,
    },
}

pub type CertificatesPrototype = Vec<CertificatePrototype>;

// ============================================================================
// Plutus Data Types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum PlutusDataVariant {
    #[serde(rename = "CBOR")]
    Cbor { hex: String },
    #[serde(rename = "MANUAL")]
    Manual { data: PlutusData },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum PlutusData {
    #[serde(rename = "INTEGER")]
    Integer { value: i128 },
    #[serde(rename = "BYTES")]
    Bytes { value: String },
    #[serde(rename = "LIST")]
    List { value: Vec<PlutusData> },
    #[serde(rename = "MAP")]
    Map {
        value: Vec<(PlutusData, PlutusData)>,
    },
    #[serde(rename = "CONSTR")]
    Constr {
        alternative: u64,
        fields: Vec<PlutusData>,
    },
}

// ============================================================================
// Data Option (for output datum)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum DataOptionPrototype {
    #[serde(rename = "DATA_HASH")]
    DataHash { value: String },
    #[serde(rename = "DATA")]
    Data { value: PlutusDataVariant },
}

// ============================================================================
// Script Reference
// ============================================================================

/// ScriptRef is stored as a CBOR hex string
pub type ScriptRefPrototype = String;

// ============================================================================
// Network ID
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum NetworkIdPrototype {
    #[serde(rename = "TESTNET")]
    Testnet,
    #[serde(rename = "MAINNET")]
    Mainnet,
}

// ============================================================================
// Value and Transaction Output
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ValuePrototype {
    pub coin: String,
    pub multiasset: Option<MultiAssetPrototype>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransactionOutputPrototype {
    pub address: String,
    pub amount: ValuePrototype,
    pub plutus_data: Option<DataOptionPrototype>,
    pub script_ref: Option<ScriptRefPrototype>,
}

pub type TransactionOutputsPrototype = Vec<TransactionOutputPrototype>;

// ============================================================================
// Transaction Input
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransactionInputPrototype {
    pub index: u32,
    pub transaction_id: String,
}

pub type TransactionInputsPrototype = Vec<TransactionInputPrototype>;

// ============================================================================
// Transaction Unspent Output (UTxO)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransactionUnspentOutputPrototype {
    pub input: TransactionInputPrototype,
    pub output: TransactionOutputPrototype,
}

pub type TransactionUnspentOutputsPrototype = Vec<TransactionUnspentOutputPrototype>;

// ============================================================================
// Withdrawals
// ============================================================================

pub type WithdrawalsPrototype = HashMap<String, String>;

// ============================================================================
// Voting Types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum VoterPrototype {
    #[serde(rename = "CONSTITUTIONAL_COMMITTEE_HOT_CRED")]
    ConstitutionalCommitteeHotCred { value: CredTypePrototype },
    #[serde(rename = "DREP")]
    DRep { value: CredTypePrototype },
    #[serde(rename = "STAKING_POOL")]
    StakingPool { value: String },
}

pub type VotersPrototype = Vec<VoterPrototype>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum VoteKindPrototype {
    #[serde(rename = "NO")]
    No,
    #[serde(rename = "YES")]
    Yes,
    #[serde(rename = "ABSTAIN")]
    Abstain,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GovernanceActionIdPrototype {
    pub index: u32,
    pub transaction_id: String,
}

pub type GovernanceActionIdsPrototype = Vec<GovernanceActionIdPrototype>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VotingProcedurePrototype {
    pub anchor: Option<AnchorPrototype>,
    pub vote: VoteKindPrototype,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VotePrototype {
    pub action_id: GovernanceActionIdPrototype,
    pub voting_procedure: VotingProcedurePrototype,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VoterVotesPrototype {
    pub voter: VoterPrototype,
    pub votes: Vec<VotePrototype>,
}

pub type VotingProceduresPrototype = Vec<VoterVotesPrototype>;

// ============================================================================
// Governance Actions
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ParameterChangeActionPrototype {
    pub gov_action_id: Option<GovernanceActionIdPrototype>,
    pub policy_hash: Option<String>,
    pub protocol_param_updates: ProtocolParamUpdatePrototype,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HardForkInitiationActionPrototype {
    pub gov_action_id: Option<GovernanceActionIdPrototype>,
    pub protocol_version: ProtocolVersionPrototype,
}

pub type TreasuryWithdrawalsPrototype = HashMap<String, String>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TreasuryWithdrawalsActionPrototype {
    pub policy_hash: Option<String>,
    pub withdrawals: TreasuryWithdrawalsPrototype,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NoConfidenceActionPrototype {
    pub gov_action_id: Option<GovernanceActionIdPrototype>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CommitteeMemberPrototype {
    pub stake_credential: CredTypePrototype,
    pub term_limit: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CommitteePrototype {
    pub members: Vec<CommitteeMemberPrototype>,
    pub quorum_threshold: UnitIntervalPrototype,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateCommitteeActionPrototype {
    pub committee: CommitteePrototype,
    pub gov_action_id: Option<GovernanceActionIdPrototype>,
    pub members_to_remove: Vec<CredTypePrototype>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConstitutionPrototype {
    pub anchor: AnchorPrototype,
    pub script_hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NewConstitutionActionPrototype {
    pub constitution: ConstitutionPrototype,
    pub gov_action_id: Option<GovernanceActionIdPrototype>,
}

pub type InfoActionPrototype = ();

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum GovernanceActionPrototype {
    #[serde(rename = "PARAMETER_CHANGE_ACTION")]
    ParameterChangeAction {
        value: ParameterChangeActionPrototype,
    },
    #[serde(rename = "HARD_FORK_INITIATION_ACTION")]
    HardForkInitiationAction {
        value: HardForkInitiationActionPrototype,
    },
    #[serde(rename = "TREASURY_WITHDRAWALS_ACTION")]
    TreasuryWithdrawalsAction {
        value: TreasuryWithdrawalsActionPrototype,
    },
    #[serde(rename = "NO_CONFIDENCE_ACTION")]
    NoConfidenceAction { value: NoConfidenceActionPrototype },
    #[serde(rename = "UPDATE_COMMITTEE_ACTION")]
    UpdateCommitteeAction {
        value: UpdateCommitteeActionPrototype,
    },
    #[serde(rename = "NEW_CONSTITUTION_ACTION")]
    NewConstitutionAction {
        value: NewConstitutionActionPrototype,
    },
    #[serde(rename = "INFO_ACTION")]
    InfoAction,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VotingProposalPrototype {
    pub anchor: AnchorPrototype,
    pub deposit: String,
    pub governance_action: GovernanceActionPrototype,
    pub reward_account: String,
}

pub type VotingProposalsPrototype = Vec<VotingProposalPrototype>;

// ============================================================================
// Protocol Parameter Update
// ============================================================================

pub type CostmdlsPrototype = HashMap<String, CostModelPrototype>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DRepVotingThresholdsPrototype {
    pub committee_no_confidence: UnitIntervalPrototype,
    pub committee_normal: UnitIntervalPrototype,
    pub hard_fork_initiation: UnitIntervalPrototype,
    pub motion_no_confidence: UnitIntervalPrototype,
    pub pp_economic_group: UnitIntervalPrototype,
    pub pp_governance_group: UnitIntervalPrototype,
    pub pp_network_group: UnitIntervalPrototype,
    pub pp_technical_group: UnitIntervalPrototype,
    pub treasury_withdrawal: UnitIntervalPrototype,
    pub update_constitution: UnitIntervalPrototype,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PoolVotingThresholdsPrototype {
    pub committee_no_confidence: UnitIntervalPrototype,
    pub committee_normal: UnitIntervalPrototype,
    pub hard_fork_initiation: UnitIntervalPrototype,
    pub motion_no_confidence: UnitIntervalPrototype,
    pub security_relevant_threshold: UnitIntervalPrototype,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExUnitPricesPrototype {
    pub mem_price: UnitIntervalPrototype,
    pub step_price: UnitIntervalPrototype,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NoncePrototype {
    pub hash: Option<NonceHashPrototype>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExUnitsPrototype {
    pub mem: String,
    pub steps: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProtocolParamUpdatePrototype {
    pub ada_per_utxo_byte: Option<String>,
    pub collateral_percentage: Option<u32>,
    pub committee_term_limit: Option<u32>,
    pub cost_models: Option<CostmdlsPrototype>,
    pub d: Option<UnitIntervalPrototype>,
    pub drep_deposit: Option<String>,
    pub drep_inactivity_period: Option<u32>,
    pub drep_voting_thresholds: Option<DRepVotingThresholdsPrototype>,
    pub execution_costs: Option<ExUnitPricesPrototype>,
    pub expansion_rate: Option<UnitIntervalPrototype>,
    pub extra_entropy: Option<NoncePrototype>,
    pub governance_action_deposit: Option<String>,
    pub governance_action_validity_period: Option<u32>,
    pub key_deposit: Option<String>,
    pub max_block_body_size: Option<u32>,
    pub max_block_ex_units: Option<ExUnitsPrototype>,
    pub max_block_header_size: Option<u32>,
    pub max_collateral_inputs: Option<u32>,
    pub max_epoch: Option<u32>,
    pub max_tx_ex_units: Option<ExUnitsPrototype>,
    pub max_tx_size: Option<u32>,
    pub max_value_size: Option<u32>,
    pub min_committee_size: Option<u32>,
    pub min_pool_cost: Option<String>,
    pub minfee_a: Option<String>,
    pub minfee_b: Option<String>,
    pub n_opt: Option<u32>,
    pub pool_deposit: Option<String>,
    pub pool_pledge_influence: Option<UnitIntervalPrototype>,
    pub pool_voting_thresholds: Option<PoolVotingThresholdsPrototype>,
    pub protocol_version: Option<ProtocolVersionPrototype>,
    pub ref_script_coins_per_byte: Option<UnitIntervalPrototype>,
    pub treasury_growth_rate: Option<UnitIntervalPrototype>,
}

pub type ProposedProtocolParameterUpdatesPrototype = HashMap<String, ProtocolParamUpdatePrototype>;

// ============================================================================
// Update
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdatePrototype {
    pub epoch: u32,
    pub proposed_protocol_parameter_updates: ProposedProtocolParameterUpdatesPrototype,
}

// ============================================================================
// Redeemer
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum RedeemerTagPrototype {
    #[serde(rename = "SPEND")]
    Spend,
    #[serde(rename = "MINT")]
    Mint,
    #[serde(rename = "CERT")]
    Cert,
    #[serde(rename = "REWARD")]
    Reward,
    #[serde(rename = "VOTE")]
    Vote,
    #[serde(rename = "VOTING_PROPOSAL")]
    VotingProposal,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RedeemerPrototype {
    pub data: PlutusDataVariant,
    pub ex_units: ExUnitsPrototype,
    pub index: String,
    pub tag: RedeemerTagPrototype,
}

pub type RedeemersPrototype = Vec<RedeemerPrototype>;

// ============================================================================
// Language
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum LanguageKindPrototype {
    #[serde(rename = "PLUTUS_V1")]
    PlutusV1,
    #[serde(rename = "PLUTUS_V2")]
    PlutusV2,
    #[serde(rename = "PLUTUS_V3")]
    PlutusV3,
}

pub type LanguagePrototype = LanguageKindPrototype;
pub type LanguagesPrototype = Vec<LanguagePrototype>;

// ============================================================================
// Witness Types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VkeywitnessPrototype {
    pub signature: String,
    pub vkey: VkeyPrototype,
}

pub type VkeywitnessesPrototype = Vec<VkeywitnessPrototype>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BootstrapWitnessPrototype {
    pub attributes: Vec<u8>,
    pub chain_code: Vec<u8>,
    pub signature: String,
    pub vkey: VkeyPrototype,
}

pub type BootstrapWitnessesPrototype = Vec<BootstrapWitnessPrototype>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlutusListPrototype {
    pub definite_encoding: Option<bool>,
    pub elems: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransactionWitnessSetPrototype {
    pub bootstraps: Option<Vec<BootstrapWitnessPrototype>>,
    pub native_scripts: Option<Vec<NativeScriptPrototype>>,
    pub plutus_data: Option<PlutusListPrototype>,
    pub plutus_scripts: Option<Vec<String>>,
    pub redeemers: Option<Vec<RedeemerPrototype>>,
    pub vkeys: Option<Vec<VkeywitnessPrototype>>,
}

// ============================================================================
// Metadata Types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum MetadatumPrototype {
    #[serde(rename = "INT")]
    Int { value: i128 },
    #[serde(rename = "BYTES")]
    Bytes { value: Vec<u8> },
    #[serde(rename = "STRING")]
    String { value: String },
    #[serde(rename = "LIST")]
    List { value: Vec<MetadatumPrototype> },
    #[serde(rename = "MAP")]
    Map {
        value: Vec<(MetadatumPrototype, MetadatumPrototype)>,
    },
}

pub type TxMetadataPrototype = HashMap<String, MetadatumPrototype>;

// ============================================================================
// Auxiliary Data
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuxiliaryDataPrototype {
    pub metadata: Option<TxMetadataPrototype>,
    pub native_scripts: Option<Vec<NativeScriptPrototype>>,
    pub plutus_scripts: Option<Vec<String>>,
    pub prefer_alonzo_format: bool,
}

pub type AuxiliaryDataSetPrototype = HashMap<String, AuxiliaryDataPrototype>;

// ============================================================================
// Transaction Body
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransactionBodyPrototype {
    pub auxiliary_data_hash: Option<String>,
    pub certs: Option<Vec<CertificatePrototype>>,
    pub collateral: Option<Vec<TransactionInputPrototype>>,
    pub collateral_return: Option<TransactionOutputPrototype>,
    pub current_treasury_value: Option<String>,
    pub donation: Option<String>,
    pub fee: String,
    pub inputs: Vec<TransactionInputPrototype>,
    pub mint: Option<MintPrototype>,
    pub network_id: Option<NetworkIdPrototype>,
    pub outputs: TransactionOutputsPrototype,
    pub reference_inputs: Option<Vec<TransactionInputPrototype>>,
    pub required_signers: Option<Vec<String>>,
    pub script_data_hash: Option<String>,
    pub total_collateral: Option<String>,
    pub ttl: Option<String>,
    pub update: Option<UpdatePrototype>,
    pub validity_start_interval: Option<String>,
    pub voting_procedures: Option<Vec<VoterVotesPrototype>>,
    pub voting_proposals: Option<Vec<VotingProposalPrototype>>,
    pub withdrawals: Option<WithdrawalsPrototype>,
}

pub type TransactionBodiesPrototype = Vec<TransactionBodyPrototype>;

// ============================================================================
// Transaction (Main Type)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransactionPrototype {
    pub auxiliary_data: Option<AuxiliaryDataPrototype>,
    pub body: TransactionBodyPrototype,
    pub is_valid: bool,
    pub witness_set: TransactionWitnessSetPrototype,
}

// ============================================================================
// Helper Implementation for Parsing
// ============================================================================

impl TransactionPrototype {
    /// Parse a Transaction from a JSON string
    pub fn from_json(json_str: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json_str)
    }

    /// Serialize the Transaction to a JSON string
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// Serialize the Transaction to a pretty-printed JSON string
    pub fn to_json_pretty(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}
