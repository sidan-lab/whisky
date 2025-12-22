use pallas::ledger::primitives::conway::Certificate as PallasCertificate;

use crate::wrapper::transaction_body::{Anchor, DRep, StakeCredential};

#[derive(Debug, PartialEq, Eq, Clone)]
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
    // PoolRegistration {
    //     operator: String,    // pool key hash
    //     vrf_keyhash: String, // vrf key hash
    //     pledge: u64,
    //     cost: u64,
    //     margin_nominator: u64,
    //     margin_denominator: u64,
    //     reward_account: RewardAccountWrapper,
    //     pool_owners: Vec<String>, // set of pool owner addr key hashes
    //     relays: Vec<RelayWrapper>,
    //     pool_metadata: Option<PoolMetadataWrapper>, // Nullable PoolMetadata
    // },
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
