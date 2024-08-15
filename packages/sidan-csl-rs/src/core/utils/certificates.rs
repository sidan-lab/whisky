use std::net::{Ipv4Addr, Ipv6Addr};

use cardano_serialization_lib as csl;
use csl::JsError;
use model::{
    Anchor, CertificateType, CommitteeColdResign, CommitteeHotAuth, DRep, DRepDeregistration,
    DRepRegistration, DRepUpdate, DelegateStake, DeregisterStake, RegisterPool, RegisterStake,
    Relay, RetirePool, StakeAndVoteDelegation, StakeRegistrationAndDelegation,
    StakeVoteRegistrationAndDelegation, VoteDelegation, VoteRegistrationAndDelegation,
};

use crate::*;

use super::to_bignum;

pub fn to_csl_drep(drep: &DRep) -> Result<csl::DRep, JsError> {
    match drep {
        DRep::KeyHash(key_hash) => Ok(csl::DRep::new_key_hash(&csl::Ed25519KeyHash::from_hex(
            key_hash,
        )?)),
        DRep::ScriptHash(script_hash) => Ok(csl::DRep::new_script_hash(
            &csl::ScriptHash::from_hex(script_hash)?,
        )),
        DRep::AlwaysAbstain => Ok(csl::DRep::new_always_abstain()),
        DRep::AlwaysNoConfidence => Ok(csl::DRep::new_always_no_confidence()),
    }
}

pub fn to_csl_anchor(anchor: &Anchor) -> Result<csl::Anchor, JsError> {
    Ok(csl::Anchor::new(
        &csl::URL::new(anchor.anchor_url.clone())?,
        &csl::AnchorDataHash::from_hex(&anchor.anchor_data_hash)?,
    ))
}

pub fn to_csl_cert(cert: CertificateType) -> Result<csl::Certificate, JsError> {
    match cert {
        CertificateType::RegisterPool(reg_pool_cert) => to_register_pool_cert(reg_pool_cert),
        CertificateType::RegisterStake(reg_stake_cert) => to_register_stake_cert(reg_stake_cert),
        CertificateType::DeregisterStake(dereg_stake_cert) => {
            to_deregister_stake_cert(dereg_stake_cert)
        }
        CertificateType::DelegateStake(deleg_stake_cert) => {
            to_delegate_stake_cert(deleg_stake_cert)
        }
        CertificateType::RetirePool(retire_pool_cert) => to_retire_pool_cert(retire_pool_cert),
        CertificateType::VoteDelegation(vote_deleg_cert) => {
            to_vote_delegation_cert(vote_deleg_cert)
        }
        CertificateType::StakeAndVoteDelegation(stake_and_vote_deleg_cert) => {
            to_stake_and_vote_delegation_cert(stake_and_vote_deleg_cert)
        }
        CertificateType::StakeRegistrationAndDelegation(stake_reg_and_deleg_cert) => {
            to_stake_registration_and_delegation_cert(stake_reg_and_deleg_cert)
        }
        CertificateType::VoteRegistrationAndDelegation(vote_reg_and_deleg_cert) => {
            to_vote_registration_and_delgation_cert(vote_reg_and_deleg_cert)
        }
        CertificateType::StakeVoteRegistrationAndDelegation(stake_vote_reg_and_deleg_cert) => {
            to_stake_vote_registration_and_delegation_cert(stake_vote_reg_and_deleg_cert)
        }
        CertificateType::CommitteeHotAuth(committee_hot_auth_cert) => {
            to_committee_hot_auth_cert(committee_hot_auth_cert)
        }
        CertificateType::CommitteeColdResign(committee_cold_resign_cert) => {
            to_commitee_cold_resign_cert(committee_cold_resign_cert)
        }
        CertificateType::DRepRegistration(drep_registration_cert) => {
            to_drep_registration_cert(drep_registration_cert)
        }
        CertificateType::DRepDeregistration(drep_deregistration_cert) => {
            to_drep_deregistration_cert(drep_deregistration_cert)
        }
        CertificateType::DRepUpdate(drep_update_cert) => to_drep_update_cert(drep_update_cert),
    }
}

fn to_register_pool_cert(register_pool: RegisterPool) -> Result<csl::Certificate, JsError> {
    let mut relays = csl::Relays::new();
    for relay in register_pool.pool_params.relays {
        match relay {
            Relay::SingleHostAddr(single_host_address_relay) => {
                let ipv4_bytes: Option<csl::Ipv4> =
                    single_host_address_relay.ipv4.map(|ipv4_str| {
                        let addr: Ipv4Addr = ipv4_str.parse().expect("ipv4 address parse failed");
                        csl::Ipv4::new(addr.octets().to_vec()).unwrap()
                    });

                let ipv6_bytes: Option<csl::Ipv6> =
                    single_host_address_relay.ipv6.map(|ipv6_str| {
                        let addr: Ipv6Addr = ipv6_str.parse().expect("ipv6 address parse failed");
                        csl::Ipv6::new(addr.octets().to_vec()).unwrap()
                    });
                relays.add(&csl::Relay::new_single_host_addr(
                    &csl::SingleHostAddr::new(
                        single_host_address_relay.port,
                        ipv4_bytes,
                        ipv6_bytes,
                    ),
                ));
            }
            Relay::SingleHostName(single_host_name_relay) => relays.add(
                &csl::Relay::new_single_host_name(&csl::SingleHostName::new(
                    single_host_name_relay.port,
                    &csl::DNSRecordAorAAAA::new(single_host_name_relay.domain_name)?,
                )),
            ),
            Relay::MultiHostName(multi_host_name_relay) => {
                relays.add(&csl::Relay::new_multi_host_name(&csl::MultiHostName::new(
                    &csl::DNSRecordSRV::new(multi_host_name_relay.domain_name)?,
                )))
            }
        }
    }

    let mut pool_owners = csl::Ed25519KeyHashes::new();
    for owner in register_pool.pool_params.owners {
        pool_owners.add(&csl::Ed25519KeyHash::from_hex(&owner)?);
    }
    Ok(csl::Certificate::new_pool_registration(
        &csl::PoolRegistration::new(&csl::PoolParams::new(
            &csl::Ed25519KeyHash::from_hex(&register_pool.pool_params.operator)?,
            &csl::VRFKeyHash::from_hex(&register_pool.pool_params.vrf_key_hash)?,
            &csl::BigNum::from_str(&register_pool.pool_params.pledge)?,
            &csl::BigNum::from_str(&register_pool.pool_params.cost)?,
            &csl::UnitInterval::new(
                &csl::BigNum::from_str(&register_pool.pool_params.margin.0.to_string())?,
                &csl::BigNum::from_str(&register_pool.pool_params.margin.1.to_string())?,
            ),
            &csl::RewardAddress::from_address(&csl::Address::from_bech32(
                &register_pool.pool_params.reward_address,
            )?)
            .unwrap(),
            &pool_owners,
            &relays,
            register_pool.pool_params.metadata.map(|data| {
                csl::PoolMetadata::new(
                    &csl::URL::new(data.url).unwrap(),
                    &csl::PoolMetadataHash::from_hex(&data.hash).unwrap(),
                )
            }),
        )),
    ))
}

fn to_register_stake_cert(register_stake: RegisterStake) -> Result<csl::Certificate, JsError> {
    Ok(csl::Certificate::new_stake_registration(
        &csl::StakeRegistration::new(
            &csl::Address::from_bech32(&register_stake.stake_key_address)?
                .payment_cred()
                .unwrap(),
        ),
    ))
}

fn to_delegate_stake_cert(delegate_stake: DelegateStake) -> Result<csl::Certificate, JsError> {
    Ok(csl::Certificate::new_stake_delegation(
        &csl::StakeDelegation::new(
            &csl::Address::from_bech32(&delegate_stake.stake_key_address)?
                .payment_cred()
                .unwrap(),
            &csl::Ed25519KeyHash::from_hex(&delegate_stake.pool_id)?,
        ),
    ))
}

fn to_deregister_stake_cert(
    deregister_stake: DeregisterStake,
) -> Result<csl::Certificate, JsError> {
    Ok(csl::Certificate::new_stake_deregistration(
        &csl::StakeDeregistration::new(
            &csl::Address::from_bech32(&deregister_stake.stake_key_address)?
                .payment_cred()
                .unwrap(),
        ),
    ))
}

fn to_retire_pool_cert(retire_pool: RetirePool) -> Result<csl::Certificate, JsError> {
    Ok(csl::Certificate::new_pool_retirement(
        &csl::PoolRetirement::new(
            &csl::Ed25519KeyHash::from_hex(&retire_pool.pool_id)?,
            retire_pool.epoch,
        ),
    ))
}

fn to_vote_delegation_cert(vote_delegation: VoteDelegation) -> Result<csl::Certificate, JsError> {
    Ok(csl::Certificate::new_vote_delegation(
        &csl::VoteDelegation::new(
            &csl::Address::from_bech32(&vote_delegation.stake_key_address)?
                .payment_cred()
                .unwrap(),
            &to_csl_drep(&vote_delegation.drep)?,
        ),
    ))
}

fn to_stake_and_vote_delegation_cert(
    stake_and_vote_delegation: StakeAndVoteDelegation,
) -> Result<csl::Certificate, JsError> {
    Ok(csl::Certificate::new_stake_and_vote_delegation(
        &csl::StakeAndVoteDelegation::new(
            &csl::Address::from_bech32(&stake_and_vote_delegation.stake_key_address)?
                .payment_cred()
                .unwrap(),
            &csl::Ed25519KeyHash::from_hex(&stake_and_vote_delegation.pool_key_hash)?,
            &to_csl_drep(&stake_and_vote_delegation.drep)?,
        ),
    ))
}

fn to_stake_registration_and_delegation_cert(
    stake_registration_and_delegation: StakeRegistrationAndDelegation,
) -> Result<csl::Certificate, JsError> {
    Ok(csl::Certificate::new_stake_registration_and_delegation(
        &csl::StakeRegistrationAndDelegation::new(
            &csl::Address::from_bech32(&stake_registration_and_delegation.stake_key_address)?
                .payment_cred()
                .unwrap(),
            &csl::Ed25519KeyHash::from_hex(&stake_registration_and_delegation.pool_key_hash)?,
            &to_bignum(stake_registration_and_delegation.coin),
        ),
    ))
}

fn to_vote_registration_and_delgation_cert(
    vote_registration_and_delgation: VoteRegistrationAndDelegation,
) -> Result<csl::Certificate, JsError> {
    Ok(csl::Certificate::new_vote_registration_and_delegation(
        &csl::VoteRegistrationAndDelegation::new(
            &csl::Address::from_bech32(&vote_registration_and_delgation.stake_key_address)?
                .payment_cred()
                .unwrap(),
            &to_csl_drep(&vote_registration_and_delgation.drep)?,
            &to_bignum(vote_registration_and_delgation.coin),
        ),
    ))
}

fn to_stake_vote_registration_and_delegation_cert(
    stake_vote_registration_and_delegation: StakeVoteRegistrationAndDelegation,
) -> Result<csl::Certificate, JsError> {
    Ok(
        csl::Certificate::new_stake_vote_registration_and_delegation(
            &csl::StakeVoteRegistrationAndDelegation::new(
                &csl::Address::from_bech32(
                    &stake_vote_registration_and_delegation.stake_key_address,
                )?
                .payment_cred()
                .unwrap(),
                &csl::Ed25519KeyHash::from_hex(
                    &stake_vote_registration_and_delegation.pool_key_hash,
                )?,
                &to_csl_drep(&stake_vote_registration_and_delegation.drep)?,
                &to_bignum(stake_vote_registration_and_delegation.coin),
            ),
        ),
    )
}

fn to_committee_hot_auth_cert(
    committee_hot_auth: CommitteeHotAuth,
) -> Result<csl::Certificate, JsError> {
    Ok(csl::Certificate::new_committee_hot_auth(
        &csl::CommitteeHotAuth::new(
            &csl::Address::from_bech32(&committee_hot_auth.committee_cold_key_address)?
                .payment_cred()
                .unwrap(),
            &csl::Address::from_bech32(&committee_hot_auth.committee_hot_key_address)?
                .payment_cred()
                .unwrap(),
        ),
    ))
}

fn to_commitee_cold_resign_cert(
    committee_cold_resign: CommitteeColdResign,
) -> Result<csl::Certificate, JsError> {
    let committee_cold_key =
        &csl::Address::from_bech32(&committee_cold_resign.committee_cold_key_address)?
            .payment_cred()
            .unwrap();
    match committee_cold_resign.anchor {
        Some(anchor) => Ok(csl::Certificate::new_committee_cold_resign(
            &csl::CommitteeColdResign::new_with_anchor(
                committee_cold_key,
                &to_csl_anchor(&anchor)?,
            ),
        )),
        None => Ok(csl::Certificate::new_committee_cold_resign(
            &csl::CommitteeColdResign::new(committee_cold_key),
        )),
    }
}

fn to_drep_registration_cert(
    drep_registration: DRepRegistration,
) -> Result<csl::Certificate, JsError> {
    Ok(csl::Certificate::new_drep_registration(
        &csl::DRepRegistration::new(
            &csl::Address::from_bech32(&drep_registration.voting_key_address)?
                .payment_cred()
                .unwrap(),
            &to_bignum(drep_registration.coin),
        ),
    ))
}

fn to_drep_deregistration_cert(
    drep_deregistration: DRepDeregistration,
) -> Result<csl::Certificate, JsError> {
    Ok(csl::Certificate::new_drep_deregistration(
        &csl::DRepDeregistration::new(
            &csl::Address::from_bech32(&drep_deregistration.voting_key_address)?
                .payment_cred()
                .unwrap(),
            &to_bignum(drep_deregistration.coin),
        ),
    ))
}

fn to_drep_update_cert(drep_update: DRepUpdate) -> Result<csl::Certificate, JsError> {
    match drep_update.anchor {
        Some(anchor) => Ok(csl::Certificate::new_drep_update(
            &csl::DRepUpdate::new_with_anchor(
                &csl::Address::from_bech32(&drep_update.voting_key_address)?
                    .payment_cred()
                    .unwrap(),
                &to_csl_anchor(&anchor)?,
            ),
        )),
        None => Ok(csl::Certificate::new_drep_update(&csl::DRepUpdate::new(
            &csl::Address::from_bech32(&drep_update.voting_key_address)?
                .payment_cred()
                .unwrap(),
        ))),
    }
}
