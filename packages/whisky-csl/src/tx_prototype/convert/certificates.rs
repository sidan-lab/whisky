use cardano_serialization_lib as csl;
use whisky_common::WError;

use super::primitives::{
    proto_to_anchor, proto_to_bignum, proto_to_credential, proto_to_drep, proto_to_ipv4,
    proto_to_ipv6, proto_to_unit_interval,
};
use crate::tx_prototype::types::*;

/// Convert CertificatePrototype to CSL Certificate
pub fn proto_to_certificate(cert: &CertificatePrototype) -> Result<csl::Certificate, WError> {
    match cert {
        CertificatePrototype::StakeRegistration { value } => proto_to_stake_registration(value),
        CertificatePrototype::StakeDeregistration { value } => proto_to_stake_deregistration(value),
        CertificatePrototype::StakeDelegation { value } => proto_to_stake_delegation(value),
        CertificatePrototype::PoolRegistration { value } => proto_to_pool_registration(value),
        CertificatePrototype::PoolRetirement { value } => proto_to_pool_retirement(value),
        CertificatePrototype::GenesisKeyDelegation { value } => {
            proto_to_genesis_key_delegation(value)
        }
        CertificatePrototype::MoveInstantaneousRewardsCert { value } => {
            proto_to_move_instantaneous_rewards_cert(value)
        }
        CertificatePrototype::CommitteeHotAuth { value } => proto_to_committee_hot_auth(value),
        CertificatePrototype::CommitteeColdResign { value } => {
            proto_to_committee_cold_resign(value)
        }
        CertificatePrototype::DRepDeregistration { value } => proto_to_drep_deregistration(value),
        CertificatePrototype::DRepRegistration { value } => proto_to_drep_registration(value),
        CertificatePrototype::DRepUpdate { value } => proto_to_drep_update(value),
        CertificatePrototype::StakeAndVoteDelegation { value } => {
            proto_to_stake_and_vote_delegation(value)
        }
        CertificatePrototype::StakeRegistrationAndDelegation { value } => {
            proto_to_stake_registration_and_delegation(value)
        }
        CertificatePrototype::StakeVoteRegistrationAndDelegation { value } => {
            proto_to_stake_vote_registration_and_delegation(value)
        }
        CertificatePrototype::VoteDelegation { value } => proto_to_vote_delegation(value),
        CertificatePrototype::VoteRegistrationAndDelegation { value } => {
            proto_to_vote_registration_and_delegation(value)
        }
    }
}

fn proto_to_stake_registration(
    reg: &StakeRegistrationPrototype,
) -> Result<csl::Certificate, WError> {
    let cred = proto_to_credential(&reg.stake_credential)?;
    let stake_reg = if let Some(coin) = &reg.coin {
        csl::StakeRegistration::new_with_explicit_deposit(&cred, &proto_to_bignum(coin)?)
    } else {
        csl::StakeRegistration::new(&cred)
    };
    Ok(csl::Certificate::new_stake_registration(&stake_reg))
}

fn proto_to_stake_deregistration(
    dereg: &StakeDeregistrationPrototype,
) -> Result<csl::Certificate, WError> {
    let cred = proto_to_credential(&dereg.stake_credential)?;
    let stake_dereg = if let Some(coin) = &dereg.coin {
        csl::StakeDeregistration::new_with_explicit_refund(&cred, &proto_to_bignum(coin)?)
    } else {
        csl::StakeDeregistration::new(&cred)
    };
    Ok(csl::Certificate::new_stake_deregistration(&stake_dereg))
}

fn proto_to_stake_delegation(deleg: &StakeDelegationPrototype) -> Result<csl::Certificate, WError> {
    let cred = proto_to_credential(&deleg.stake_credential)?;
    let pool_keyhash = csl::Ed25519KeyHash::from_hex(&deleg.pool_keyhash).map_err(
        WError::from_err("proto_to_stake_delegation - invalid pool_keyhash"),
    )?;
    Ok(csl::Certificate::new_stake_delegation(
        &csl::StakeDelegation::new(&cred, &pool_keyhash),
    ))
}

fn proto_to_pool_registration(
    pool_reg: &PoolRegistrationPrototype,
) -> Result<csl::Certificate, WError> {
    let params = &pool_reg.pool_params;

    let operator = csl::Ed25519KeyHash::from_hex(&params.operator).map_err(WError::from_err(
        "proto_to_pool_registration - invalid operator",
    ))?;
    let vrf_keyhash = csl::VRFKeyHash::from_hex(&params.vrf_keyhash).map_err(WError::from_err(
        "proto_to_pool_registration - invalid vrf_keyhash",
    ))?;
    let pledge = proto_to_bignum(&params.pledge)?;
    let cost = proto_to_bignum(&params.cost)?;
    let margin = proto_to_unit_interval(&params.margin)?;
    let reward_account = csl::RewardAddress::from_address(
        &csl::Address::from_bech32(&params.reward_account).map_err(WError::from_err(
            "proto_to_pool_registration - invalid reward_account",
        ))?,
    )
    .ok_or_else(|| WError::new("proto_to_pool_registration", "invalid reward_account"))?;

    let mut pool_owners = csl::Ed25519KeyHashes::new();
    for owner in &params.pool_owners {
        let owner_hash = csl::Ed25519KeyHash::from_hex(owner).map_err(WError::from_err(
            "proto_to_pool_registration - invalid pool_owner",
        ))?;
        pool_owners.add(&owner_hash);
    }

    let relays = proto_to_relays(&params.relays)?;

    let pool_metadata = if let Some(metadata) = &params.pool_metadata {
        Some(csl::PoolMetadata::new(
            &csl::URL::new(metadata.url.clone()).map_err(WError::from_err(
                "proto_to_pool_registration - invalid metadata url",
            ))?,
            &csl::PoolMetadataHash::from_hex(&metadata.pool_metadata_hash).map_err(
                WError::from_err("proto_to_pool_registration - invalid metadata hash"),
            )?,
        ))
    } else {
        None
    };

    let pool_params = csl::PoolParams::new(
        &operator,
        &vrf_keyhash,
        &pledge,
        &cost,
        &margin,
        &reward_account,
        &pool_owners,
        &relays,
        pool_metadata,
    );

    Ok(csl::Certificate::new_pool_registration(
        &csl::PoolRegistration::new(&pool_params),
    ))
}

fn proto_to_relays(relays: &[RelayPrototype]) -> Result<csl::Relays, WError> {
    let mut result = csl::Relays::new();
    for relay in relays {
        match relay {
            RelayPrototype::SingleHostAddr { value: addr } => {
                let ipv4 = addr.ipv4.as_ref().map(|ip| proto_to_ipv4(ip)).transpose()?;
                let ipv6 = addr.ipv6.as_ref().map(|ip| proto_to_ipv6(ip)).transpose()?;
                result.add(&csl::Relay::new_single_host_addr(
                    &csl::SingleHostAddr::new(addr.port, ipv4, ipv6),
                ));
            }
            RelayPrototype::SingleHostName { value: name } => {
                let dns_name = csl::DNSRecordAorAAAA::new(name.dns_name.clone())
                    .map_err(WError::from_err("proto_to_relays - invalid dns_name"))?;
                result.add(&csl::Relay::new_single_host_name(
                    &csl::SingleHostName::new(name.port, &dns_name),
                ));
            }
            RelayPrototype::MultiHostName { value: name } => {
                let dns_name = csl::DNSRecordSRV::new(name.dns_name.clone())
                    .map_err(WError::from_err("proto_to_relays - invalid dns_name"))?;
                result.add(&csl::Relay::new_multi_host_name(&csl::MultiHostName::new(
                    &dns_name,
                )));
            }
        }
    }
    Ok(result)
}

fn proto_to_pool_retirement(
    pool_ret: &PoolRetirementPrototype,
) -> Result<csl::Certificate, WError> {
    let pool_keyhash = csl::Ed25519KeyHash::from_hex(&pool_ret.pool_keyhash).map_err(
        WError::from_err("proto_to_pool_retirement - invalid pool_keyhash"),
    )?;
    Ok(csl::Certificate::new_pool_retirement(
        &csl::PoolRetirement::new(&pool_keyhash, pool_ret.epoch),
    ))
}

fn proto_to_genesis_key_delegation(
    genesis: &GenesisKeyDelegationPrototype,
) -> Result<csl::Certificate, WError> {
    let genesishash = csl::GenesisHash::from_hex(&genesis.genesishash).map_err(
        WError::from_err("proto_to_genesis_key_delegation - invalid genesishash"),
    )?;
    let genesis_delegate_hash = csl::GenesisDelegateHash::from_hex(&genesis.genesis_delegate_hash)
        .map_err(WError::from_err(
            "proto_to_genesis_key_delegation - invalid genesis_delegate_hash",
        ))?;
    let vrf_keyhash = csl::VRFKeyHash::from_hex(&genesis.vrf_keyhash).map_err(WError::from_err(
        "proto_to_genesis_key_delegation - invalid vrf_keyhash",
    ))?;
    Ok(csl::Certificate::new_genesis_key_delegation(
        &csl::GenesisKeyDelegation::new(&genesishash, &genesis_delegate_hash, &vrf_keyhash),
    ))
}

fn proto_to_move_instantaneous_rewards_cert(
    mir: &MoveInstantaneousRewardsCertPrototype,
) -> Result<csl::Certificate, WError> {
    let mir_inner = &mir.move_instantaneous_reward;
    let pot = match mir_inner.pot {
        MIRPotPrototype::Reserves => csl::MIRPot::Reserves,
        MIRPotPrototype::Treasury => csl::MIRPot::Treasury,
    };

    let mir_reward = match &mir_inner.variant {
        MIREnumPrototype::ToOtherPot { value: amount } => {
            csl::MoveInstantaneousReward::new_to_other_pot(pot, &proto_to_bignum(amount)?)
        }
        MIREnumPrototype::ToStakeCredentials { value: creds } => {
            let mut stake_creds = csl::MIRToStakeCredentials::new();
            for cred in creds {
                let stake_cred = proto_to_credential(&cred.stake_cred)?;
                let amount = csl::Int::from_str(&cred.amount)
                    .map_err(WError::from_err("proto_to_mir - invalid amount"))?;
                stake_creds.insert(&stake_cred, &amount);
            }
            csl::MoveInstantaneousReward::new_to_stake_creds(pot, &stake_creds)
        }
    };

    Ok(csl::Certificate::new_move_instantaneous_rewards_cert(
        &csl::MoveInstantaneousRewardsCert::new(&mir_reward),
    ))
}

fn proto_to_committee_hot_auth(
    auth: &CommitteeHotAuthPrototype,
) -> Result<csl::Certificate, WError> {
    let cold_cred = proto_to_credential(&auth.committee_cold_credential)?;
    let hot_cred = proto_to_credential(&auth.committee_hot_credential)?;
    Ok(csl::Certificate::new_committee_hot_auth(
        &csl::CommitteeHotAuth::new(&cold_cred, &hot_cred),
    ))
}

fn proto_to_committee_cold_resign(
    resign: &CommitteeColdResignPrototype,
) -> Result<csl::Certificate, WError> {
    let cold_cred = proto_to_credential(&resign.committee_cold_credential)?;
    let committee_resign = if let Some(anchor) = &resign.anchor {
        csl::CommitteeColdResign::new_with_anchor(&cold_cred, &proto_to_anchor(anchor)?)
    } else {
        csl::CommitteeColdResign::new(&cold_cred)
    };
    Ok(csl::Certificate::new_committee_cold_resign(
        &committee_resign,
    ))
}

fn proto_to_drep_deregistration(
    dereg: &DRepDeregistrationPrototype,
) -> Result<csl::Certificate, WError> {
    let voting_cred = proto_to_credential(&dereg.voting_credential)?;
    let coin = proto_to_bignum(&dereg.coin)?;
    Ok(csl::Certificate::new_drep_deregistration(
        &csl::DRepDeregistration::new(&voting_cred, &coin),
    ))
}

fn proto_to_drep_registration(reg: &DRepRegistrationPrototype) -> Result<csl::Certificate, WError> {
    let voting_cred = proto_to_credential(&reg.voting_credential)?;
    let coin = proto_to_bignum(&reg.coin)?;
    let drep_reg = if let Some(anchor) = &reg.anchor {
        csl::DRepRegistration::new_with_anchor(&voting_cred, &coin, &proto_to_anchor(anchor)?)
    } else {
        csl::DRepRegistration::new(&voting_cred, &coin)
    };
    Ok(csl::Certificate::new_drep_registration(&drep_reg))
}

fn proto_to_drep_update(update: &DRepUpdatePrototype) -> Result<csl::Certificate, WError> {
    let voting_cred = proto_to_credential(&update.voting_credential)?;
    let drep_update = if let Some(anchor) = &update.anchor {
        csl::DRepUpdate::new_with_anchor(&voting_cred, &proto_to_anchor(anchor)?)
    } else {
        csl::DRepUpdate::new(&voting_cred)
    };
    Ok(csl::Certificate::new_drep_update(&drep_update))
}

fn proto_to_stake_and_vote_delegation(
    deleg: &StakeAndVoteDelegationPrototype,
) -> Result<csl::Certificate, WError> {
    let stake_cred = proto_to_credential(&deleg.stake_credential)?;
    let pool_keyhash = csl::Ed25519KeyHash::from_hex(&deleg.pool_keyhash).map_err(
        WError::from_err("proto_to_stake_and_vote_delegation - invalid pool_keyhash"),
    )?;
    let drep = proto_to_drep(&deleg.drep)?;
    Ok(csl::Certificate::new_stake_and_vote_delegation(
        &csl::StakeAndVoteDelegation::new(&stake_cred, &pool_keyhash, &drep),
    ))
}

fn proto_to_stake_registration_and_delegation(
    reg_deleg: &StakeRegistrationAndDelegationPrototype,
) -> Result<csl::Certificate, WError> {
    let stake_cred = proto_to_credential(&reg_deleg.stake_credential)?;
    let pool_keyhash = csl::Ed25519KeyHash::from_hex(&reg_deleg.pool_keyhash).map_err(
        WError::from_err("proto_to_stake_registration_and_delegation - invalid pool_keyhash"),
    )?;
    let coin = proto_to_bignum(&reg_deleg.coin)?;
    Ok(csl::Certificate::new_stake_registration_and_delegation(
        &csl::StakeRegistrationAndDelegation::new(&stake_cred, &pool_keyhash, &coin),
    ))
}

fn proto_to_stake_vote_registration_and_delegation(
    reg_deleg: &StakeVoteRegistrationAndDelegationPrototype,
) -> Result<csl::Certificate, WError> {
    let stake_cred = proto_to_credential(&reg_deleg.stake_credential)?;
    let pool_keyhash = csl::Ed25519KeyHash::from_hex(&reg_deleg.pool_keyhash).map_err(
        WError::from_err("proto_to_stake_vote_registration_and_delegation - invalid pool_keyhash"),
    )?;
    let drep = proto_to_drep(&reg_deleg.drep)?;
    let coin = proto_to_bignum(&reg_deleg.coin)?;
    Ok(
        csl::Certificate::new_stake_vote_registration_and_delegation(
            &csl::StakeVoteRegistrationAndDelegation::new(&stake_cred, &pool_keyhash, &drep, &coin),
        ),
    )
}

fn proto_to_vote_delegation(deleg: &VoteDelegationPrototype) -> Result<csl::Certificate, WError> {
    let stake_cred = proto_to_credential(&deleg.stake_credential)?;
    let drep = proto_to_drep(&deleg.drep)?;
    Ok(csl::Certificate::new_vote_delegation(
        &csl::VoteDelegation::new(&stake_cred, &drep),
    ))
}

fn proto_to_vote_registration_and_delegation(
    reg_deleg: &VoteRegistrationAndDelegationPrototype,
) -> Result<csl::Certificate, WError> {
    let stake_cred = proto_to_credential(&reg_deleg.stake_credential)?;
    let drep = proto_to_drep(&reg_deleg.drep)?;
    let coin = proto_to_bignum(&reg_deleg.coin)?;
    Ok(csl::Certificate::new_vote_registration_and_delegation(
        &csl::VoteRegistrationAndDelegation::new(&stake_cred, &drep, &coin),
    ))
}

/// Convert Vec<CertificatePrototype> to CSL Certificates
pub fn proto_to_certificates(certs: &[CertificatePrototype]) -> Result<csl::Certificates, WError> {
    let mut result = csl::Certificates::new();
    for cert in certs {
        result.add(&proto_to_certificate(cert)?);
    }
    Ok(result)
}
