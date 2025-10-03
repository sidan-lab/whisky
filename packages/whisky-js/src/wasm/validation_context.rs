/*!
 * Validation Context with JS-friendly interfaces
 *
 * This module provides JS-friendly wrapper types that accept hex strings instead of Vec<u8>
 * for easier integration with JavaScript/TypeScript code. The wrapper types are automatically
 * converted to the original types using From/Into traits.
 *
 * Key wrapper types:
 * - ValidationInputContextJS: Main wrapper for validation context
 * - LocalCredentialJS: Wrapper for LocalCredential (KeyHash/ScriptHash as hex strings)
 * - GovernanceActionIdJS: Wrapper for GovernanceActionId (tx_hash as hex string)
 * - VoterJS: Wrapper for Voter enum (all variants accept hex strings)
 * - CommitteeInputContextJS: Wrapper for CommitteeInputContext
 * - GovActionInputContextJS: Wrapper for GovActionInputContext
 */

use cquisitor_lib::validators::{
    common::{GovernanceActionId, GovernanceActionType, LocalCredential, NetworkType, Voter},
    input_contexts::{
        AccountInputContext, CommitteeInputContext, DrepInputContext, GovActionInputContext,
        PoolInputContext, UtxoInputContext, ValidationInputContext,
    },
    protocol_params::ProtocolParameters,
};
use serde::{Deserialize, Serialize};
use whisky_common::hex_to_bytes;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidationInputContextJS {
    pub utxo_set: Vec<UtxoInputContext>,
    pub protocol_parameters: ProtocolParameters,
    pub slot: u64,
    pub account_contexts: Vec<AccountInputContext>,
    pub drep_contexts: Vec<DrepInputContext>,
    pub pool_contexts: Vec<PoolInputContext>,
    pub gov_action_contexts: Vec<GovActionInputContextJS>,
    pub last_enacted_gov_action: Vec<GovActionInputContextJS>,
    pub current_committee_members: Vec<CommitteeInputContextJS>,
    pub potential_committee_members: Vec<CommitteeInputContextJS>,
    pub treasury_value: u64,
    pub network_type: NetworkType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GovActionInputContextJS {
    pub action_id: GovernanceActionIdJS,
    pub action_type: GovernanceActionType,
    pub is_active: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GovernanceActionIdJS {
    pub tx_hash: String, // Hex string
    pub index: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommitteeInputContextJS {
    pub committee_member_cold: LocalCredentialJS,
    pub committee_member_hot: Option<LocalCredentialJS>,
    pub is_resigned: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LocalCredentialJS {
    KeyHash(String),    // Hex string
    ScriptHash(String), // Hex string
}

// JS-friendly wrapper for Voter that accepts hex strings instead of Vec<u8>
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum VoterJS {
    ConstitutionalCommitteeHotScriptHash(String), // Hex string
    ConstitutionalCommitteeHotKeyHash(String),    // Hex string
    DRepScriptHash(String),                       // Hex string
    DRepKeyHash(String),                          // Hex string
    StakingPoolKeyHash(String),                   // Hex string
}

// Conversion implementations
impl From<VoterJS> for Voter {
    fn from(js: VoterJS) -> Self {
        match js {
            VoterJS::ConstitutionalCommitteeHotScriptHash(hex_string) => {
                Voter::ConstitutionalCommitteeHotScriptHash(
                    hex_to_bytes(&hex_string).unwrap_or_default(),
                )
            }
            VoterJS::ConstitutionalCommitteeHotKeyHash(hex_string) => {
                Voter::ConstitutionalCommitteeHotKeyHash(
                    hex_to_bytes(&hex_string).unwrap_or_default(),
                )
            }
            VoterJS::DRepScriptHash(hex_string) => {
                Voter::DRepScriptHash(hex_to_bytes(&hex_string).unwrap_or_default())
            }
            VoterJS::DRepKeyHash(hex_string) => {
                Voter::DRepKeyHash(hex_to_bytes(&hex_string).unwrap_or_default())
            }
            VoterJS::StakingPoolKeyHash(hex_string) => {
                Voter::StakingPoolKeyHash(hex_to_bytes(&hex_string).unwrap_or_default())
            }
        }
    }
}

impl From<LocalCredentialJS> for LocalCredential {
    fn from(js: LocalCredentialJS) -> Self {
        match js {
            LocalCredentialJS::KeyHash(hex_string) => {
                LocalCredential::KeyHash(hex_to_bytes(&hex_string).unwrap_or_default())
            }
            LocalCredentialJS::ScriptHash(hex_string) => {
                LocalCredential::ScriptHash(hex_to_bytes(&hex_string).unwrap_or_default())
            }
        }
    }
}

impl From<GovernanceActionIdJS> for GovernanceActionId {
    fn from(js: GovernanceActionIdJS) -> Self {
        Self {
            tx_hash: hex_to_bytes(&js.tx_hash).unwrap_or_default(),
            index: js.index,
        }
    }
}

impl From<GovActionInputContextJS> for GovActionInputContext {
    fn from(js: GovActionInputContextJS) -> Self {
        Self {
            action_id: js.action_id.into(),
            action_type: js.action_type,
            is_active: js.is_active,
        }
    }
}

impl From<CommitteeInputContextJS> for CommitteeInputContext {
    fn from(js: CommitteeInputContextJS) -> Self {
        Self {
            committee_member_cold: js.committee_member_cold.into(),
            committee_member_hot: js.committee_member_hot.map(|hot| hot.into()),
            is_resigned: js.is_resigned,
        }
    }
}

impl From<ValidationInputContextJS> for ValidationInputContext {
    fn from(js: ValidationInputContextJS) -> Self {
        Self {
            utxo_set: js.utxo_set,
            protocol_parameters: js.protocol_parameters,
            slot: js.slot,
            account_contexts: js.account_contexts,
            drep_contexts: js.drep_contexts,
            pool_contexts: js.pool_contexts,
            gov_action_contexts: js
                .gov_action_contexts
                .into_iter()
                .map(|ctx| ctx.into())
                .collect(),
            last_enacted_gov_action: js
                .last_enacted_gov_action
                .into_iter()
                .map(|ctx| ctx.into())
                .collect(),
            current_committee_members: js
                .current_committee_members
                .into_iter()
                .map(|ctx| ctx.into())
                .collect(),
            potential_committee_members: js
                .potential_committee_members
                .into_iter()
                .map(|ctx| ctx.into())
                .collect(),
            treasury_value: js.treasury_value,
            network_type: js.network_type,
        }
    }
}
