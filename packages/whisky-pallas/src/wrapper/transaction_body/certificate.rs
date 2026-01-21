use std::str::FromStr;

use pallas::{
    codec::utils::Set,
    ledger::primitives::{
        conway::Certificate::{
            self as PallasCertificate, PoolRegistration as PallasPoolRegistration,
        },
        Fragment,
    },
};
use whisky_common::WError;

use crate::wrapper::transaction_body::{
    parse_rational_number, Anchor, DRep, PoolMetadata, Relay, RewardAccount, StakeCredential,
};

pub enum CertificateKind {
    StakeRegistration {
        stake_credential: StakeCredential,
    },
    StakeDeregistration {
        stake_credential: StakeCredential,
    },
    StakeDelegation {
        stake_credential: StakeCredential,
        pool_key_hash: String,
    },
    PoolRegistration {
        operator: String,    // pool key hash
        vrf_keyhash: String, // vrf key hash
        pledge: u64,
        cost: u64,
        margin: (u64, u64), // (nominator, denominator)
        reward_account: RewardAccount,
        pool_owners: Vec<String>, // set of pool owner addr key hashes
        relays: Vec<Relay>,
        pool_metadata: Option<PoolMetadata>, // Nullable PoolMetadata
    },
    PoolRetirement {
        pool_key_hash: String,
        epoch: u64,
    },

    Reg {
        stake_credential: StakeCredential,
        amount: u64,
    },
    UnReg {
        stake_credential: StakeCredential,
        amount: u64,
    },
    VoteDeleg {
        stake_credential: StakeCredential,
        drep: DRep,
    },
    StakeVoteDeleg {
        stake_credential: StakeCredential,
        pool_key_hash: String,
        drep: DRep,
    },
    StakeRegDeleg {
        stake_credential: StakeCredential,
        pool_key_hash: String,
        amount: u64,
    },
    VoteRegDeleg {
        stake_credential: StakeCredential,
        drep: DRep,
        amount: u64,
    },
    StakeVoteRegDeleg {
        stake_credential: StakeCredential,
        pool_key_hash: String,
        drep: DRep,
        amount: u64,
    },
    AuthCommitteeHot {
        committee_cold_cred: StakeCredential,
        committee_hot_cred: StakeCredential,
    },
    ResignCommitteeCold {
        committee_cold_cred: StakeCredential,
        anchor: Option<Anchor>,
    },
    RegDRepCert {
        drep_cred: StakeCredential,
        amount: u64,
        anchor: Option<Anchor>,
    },
    UnRegDRepCert {
        drep_cred: StakeCredential,
        amount: u64,
    },
    UpdateDRepCert {
        drep_cred: StakeCredential,
        anchor: Option<Anchor>,
    },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Certificate {
    pub inner: PallasCertificate,
}

impl Certificate {
    pub fn new(certificate_kind: CertificateKind) -> Result<Self, WError> {
        let pallas_certificate = match certificate_kind {
            CertificateKind::StakeRegistration { stake_credential } => {
                PallasCertificate::StakeRegistration(stake_credential.inner)
            }
            CertificateKind::StakeDeregistration { stake_credential } => {
                PallasCertificate::StakeDeregistration(stake_credential.inner)
            }
            CertificateKind::StakeDelegation {
                stake_credential,
                pool_key_hash,
            } => {
                let pallas_pool_key_hash = pallas::ledger::primitives::Hash::<28>::from_str(
                    &pool_key_hash,
                )
                .map_err(|_| {
                    WError::new(
                        "WhiskyPallas - Creating Certificate",
                        "invalid pool key hash",
                    )
                })?;
                PallasCertificate::StakeDelegation(stake_credential.inner, pallas_pool_key_hash)
            }
            CertificateKind::PoolRegistration {
                operator,
                vrf_keyhash,
                pledge,
                cost,
                margin,
                reward_account,
                pool_owners,
                relays,
                pool_metadata,
            } => {
                let pallas_operator = pallas::ledger::primitives::Hash::<28>::from_str(&operator)
                    .map_err(|_| {
                    WError::new(
                        "WhiskyPallas - Creating Certificate",
                        "invalid operator pool key hash",
                    )
                })?;
                let pallas_vrf_keyhash = pallas::ledger::primitives::Hash::<32>::from_str(
                    &vrf_keyhash,
                )
                .map_err(|_| {
                    WError::new(
                        "WhiskyPallas - Creating Certificate",
                        "invalid vrf key hash",
                    )
                })?;
                let mut pallas_pool_owners_vec = Vec::new();
                for owner in pool_owners {
                    let pallas_owner = pallas::ledger::primitives::Hash::<28>::from_str(&owner)
                        .map_err(|_| {
                            WError::new(
                                "WhiskyPallas - Creating Certificate",
                                "invalid pool owner key hash",
                            )
                        })?;
                    pallas_pool_owners_vec.push(pallas_owner);
                }
                let pallas_relays: Vec<pallas::ledger::primitives::conway::Relay> =
                    relays.into_iter().map(|r| r.inner).collect();
                let pallas_pool_metadata = match pool_metadata {
                    Some(pm) => Some(pm.inner),
                    None => None,
                };
                PallasPoolRegistration {
                    operator: pallas_operator,
                    vrf_keyhash: pallas_vrf_keyhash,
                    pledge,
                    cost,
                    margin: parse_rational_number(margin),
                    reward_account: reward_account.inner,
                    pool_owners: Set::from(pallas_pool_owners_vec),
                    relays: pallas_relays,
                    pool_metadata: pallas_pool_metadata,
                }
            }
            CertificateKind::PoolRetirement {
                pool_key_hash,
                epoch,
            } => {
                let pallas_pool_key_hash = pallas::ledger::primitives::Hash::<28>::from_str(
                    &pool_key_hash,
                )
                .map_err(|_| {
                    WError::new(
                        "WhiskyPallas - Creating Certificate",
                        "invalid pool key hash",
                    )
                })?;
                PallasCertificate::PoolRetirement(pallas_pool_key_hash, epoch)
            }
            CertificateKind::Reg {
                stake_credential,
                amount,
            } => PallasCertificate::Reg(stake_credential.inner, amount),
            CertificateKind::UnReg {
                stake_credential,
                amount,
            } => PallasCertificate::UnReg(stake_credential.inner, amount),
            CertificateKind::VoteDeleg {
                stake_credential,
                drep,
            } => PallasCertificate::VoteDeleg(stake_credential.inner, drep.inner),
            CertificateKind::StakeVoteDeleg {
                stake_credential,
                pool_key_hash,
                drep,
            } => {
                let pallas_pool_key_hash = pallas::ledger::primitives::Hash::<28>::from_str(
                    &pool_key_hash,
                )
                .map_err(|_| {
                    WError::new(
                        "WhiskyPallas - Creating Certificate",
                        "invalid pool key hash",
                    )
                })?;
                PallasCertificate::StakeVoteDeleg(
                    stake_credential.inner,
                    pallas_pool_key_hash,
                    drep.inner,
                )
            }
            CertificateKind::StakeRegDeleg {
                stake_credential,
                pool_key_hash,
                amount,
            } => {
                let pallas_pool_key_hash = pallas::ledger::primitives::Hash::<28>::from_str(
                    &pool_key_hash,
                )
                .map_err(|_| {
                    WError::new(
                        "WhiskyPallas - Creating Certificate",
                        "invalid pool key hash",
                    )
                })?;
                PallasCertificate::StakeRegDeleg(
                    stake_credential.inner,
                    pallas_pool_key_hash,
                    amount,
                )
            }
            CertificateKind::VoteRegDeleg {
                stake_credential,
                drep,
                amount,
            } => PallasCertificate::VoteRegDeleg(stake_credential.inner, drep.inner, amount),
            CertificateKind::StakeVoteRegDeleg {
                stake_credential,
                pool_key_hash,
                drep,
                amount,
            } => {
                let pallas_pool_key_hash = pallas::ledger::primitives::Hash::<28>::from_str(
                    &pool_key_hash,
                )
                .map_err(|_| {
                    WError::new(
                        "WhiskyPallas - Creating Certificate",
                        "invalid pool key hash",
                    )
                })?;
                PallasCertificate::StakeVoteRegDeleg(
                    stake_credential.inner,
                    pallas_pool_key_hash,
                    drep.inner,
                    amount,
                )
            }
            CertificateKind::AuthCommitteeHot {
                committee_cold_cred,
                committee_hot_cred,
            } => PallasCertificate::AuthCommitteeHot(
                committee_cold_cred.inner,
                committee_hot_cred.inner,
            ),
            CertificateKind::ResignCommitteeCold {
                committee_cold_cred,
                anchor,
            } => PallasCertificate::ResignCommitteeCold(
                committee_cold_cred.inner,
                anchor.map(|a| a.inner),
            ),
            CertificateKind::RegDRepCert {
                drep_cred,
                amount,
                anchor,
            } => PallasCertificate::RegDRepCert(drep_cred.inner, amount, anchor.map(|a| a.inner)),
            CertificateKind::UnRegDRepCert { drep_cred, amount } => {
                PallasCertificate::UnRegDRepCert(drep_cred.inner, amount)
            }
            CertificateKind::UpdateDRepCert { drep_cred, anchor } => {
                PallasCertificate::UpdateDRepCert(drep_cred.inner, anchor.map(|a| a.inner))
            }
        };

        Ok(Self {
            inner: pallas_certificate,
        })
    }

    pub fn encode(&self) -> String {
        hex::encode(
            self.inner
                .encode_fragment()
                .expect("encoding failed at Certificate"),
        )
    }

    pub fn decode_bytes(bytes: &[u8]) -> Result<Self, String> {
        let inner = PallasCertificate::decode_fragment(&bytes)
            .map_err(|e| format!("Fragment decode error: {}", e.to_string()))?;
        Ok(Self { inner })
    }
}
