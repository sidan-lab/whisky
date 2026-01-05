use std::{collections::BTreeMap, str::FromStr};

use pallas::{
    codec::utils::Set,
    crypto::hash::Hash,
    ledger::primitives::{
        conway::GovAction as PallasGovAction, Fragment, StakeCredential as PallasStakeCredential,
    },
};

use crate::wrapper::transaction_body::{
    parse_rational_number, Constitution, GovActionId, ProtocolParamUpdate, RewardAccount,
    StakeCredential,
};

pub enum GovActionKind {
    ParameterChange {
        gov_action_id: Option<GovActionId>,
        protocol_param_update: ProtocolParamUpdate,
        script_hash: Option<String>,
    },
    HardForkInitiation {
        gov_action_id: Option<GovActionId>,
        protocol_version: (u64, u64),
    },
    TreasuryWithdrawals {
        withdrawals: Vec<(RewardAccount, u64)>,
        script_hash: Option<String>,
    },
    NoConfidence {
        gov_action_id: Option<GovActionId>,
    },
    UpdateCommittee {
        gov_action_id: Option<GovActionId>,
        cold_credentials: Vec<StakeCredential>,
        hot_credentials: Vec<(StakeCredential, u64)>,
        threshold: (u64, u64),
    },
    NewConstitution {
        gov_action_id: Option<GovActionId>,
        constitution: Constitution,
    },
    Information,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GovAction {
    pub inner: PallasGovAction,
}

impl GovAction {
    pub fn new(gov_action: GovActionKind) -> Result<Self, String> {
        let pallas_gov_action = match gov_action {
            GovActionKind::ParameterChange {
                gov_action_id,
                protocol_param_update,
                script_hash,
            } => PallasGovAction::ParameterChange(
                gov_action_id.map(|id| id.inner),
                Box::new(protocol_param_update.inner),
                script_hash.map(|hash_str| {
                    Hash::<28>::from_str(&hash_str).expect("Invalid script hash length")
                }),
            ),
            GovActionKind::HardForkInitiation {
                gov_action_id,
                protocol_version,
            } => PallasGovAction::HardForkInitiation(
                gov_action_id.map(|id| id.inner),
                protocol_version,
            ),
            GovActionKind::TreasuryWithdrawals {
                withdrawals,
                script_hash,
            } => {
                let mut pallas_withdrawals = BTreeMap::new();
                for (reward_account, amount) in withdrawals {
                    pallas_withdrawals.insert(reward_account.inner, amount);
                }
                PallasGovAction::TreasuryWithdrawals(
                    pallas_withdrawals,
                    script_hash.map(|hash_str| {
                        Hash::<28>::from_str(&hash_str).expect("Invalid script hash length")
                    }),
                )
            }
            GovActionKind::NoConfidence { gov_action_id } => {
                PallasGovAction::NoConfidence(gov_action_id.map(|id| id.inner))
            }
            GovActionKind::UpdateCommittee {
                gov_action_id,
                cold_credentials,
                hot_credentials,
                threshold,
            } => {
                let mut pallas_hot_credentials = BTreeMap::new();
                for (cred, epoch) in hot_credentials {
                    pallas_hot_credentials.insert(cred.inner, epoch);
                }
                PallasGovAction::UpdateCommittee(
                    gov_action_id.map(|id| id.inner),
                    Set::from(
                        cold_credentials
                            .into_iter()
                            .map(|cred| cred.inner)
                            .collect::<Vec<PallasStakeCredential>>(),
                    ),
                    pallas_hot_credentials,
                    parse_rational_number(threshold),
                )
            }
            GovActionKind::NewConstitution {
                gov_action_id,
                constitution,
            } => PallasGovAction::NewConstitution(
                gov_action_id.map(|id| id.inner),
                constitution.inner,
            ),
            GovActionKind::Information => PallasGovAction::Information,
        };

        Ok(Self {
            inner: pallas_gov_action,
        })
    }

    pub fn encode(&self) -> String {
        hex::encode(
            self.inner
                .encode_fragment()
                .expect("encoding failed at GovAction"),
        )
    }

    pub fn decode_bytes(bytes: &[u8]) -> Result<Self, String> {
        let inner = PallasGovAction::decode_fragment(&bytes)
            .map_err(|e| format!("Fragment decode error: {}", e.to_string()))?;
        Ok(Self { inner })
    }
}
