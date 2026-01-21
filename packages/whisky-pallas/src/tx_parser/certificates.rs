use std::net::{Ipv4Addr, Ipv6Addr};

use pallas::{
    codec::utils::Set,
    ledger::primitives::{
        conway::Tx, AddrKeyhash, Coin, PoolKeyhash, PoolMetadata as PallasPoolMetadata, Relay,
        RewardAccount as PallasRewardAccount, UnitInterval, VrfKeyhash,
    },
};
use whisky_common::{
    Certificate, CertificateType, CommitteeColdResign, CommitteeHotAuth, DRep as WhiskyDRep,
    DRepDeregistration, DRepRegistration, DRepUpdate, DelegateStake, DeregisterStake,
    MultiHostName, PoolMetadata, PoolParams, RegisterPool, RegisterStake, Relay as WhiskyRelay,
    RetirePool, ScriptCertificate, ScriptSource, SimpleScriptCertificate, SimpleScriptSource,
    SingleHostAddr, SingleHostName, StakeAndVoteDelegation, StakeRegistrationAndDelegation,
    StakeVoteRegistrationAndDelegation, VoteDelegation, VoteRegistrationAndDelegation, WError,
};

use crate::{
    tx_parser::context::{ParserContext, RedeemerIndex},
    wrapper::{
        transaction_body::{DRep, DRepKind, RewardAccount},
        witness_set::redeemer::RedeemerTag,
    },
};

fn handle_stake_credential(
    stake_credential: &pallas::ledger::primitives::StakeCredential,
    certificate_type: CertificateType,
    context: &ParserContext,
    index: usize,
) -> Result<Certificate, WError> {
    match stake_credential {
        pallas::ledger::primitives::StakeCredential::ScriptHash(hash) => {
            let script_source = context
                .script_witnesses
                .scripts
                .get(&hash.to_string())
                .ok_or(WError::new(
                    "WhiskyPallas - Extracting certificates:",
                    &format!(
                        "No script found for stake credential with script hash: {}",
                        hash.to_string()
                    ),
                ))?;
            match script_source {
                crate::tx_parser::context::Script::ProvidedNative(
                    provided_simple_script_source,
                ) => Ok(Certificate::SimpleScriptCertificate(
                    SimpleScriptCertificate {
                        cert: certificate_type,
                        simple_script_source: Some(SimpleScriptSource::ProvidedSimpleScriptSource(
                            provided_simple_script_source.clone(),
                        )),
                    },
                )),
                crate::tx_parser::context::Script::ProvidedPlutus(provided_script_source) => {
                    let redeemer = context.script_witnesses.redeemers.get(&RedeemerIndex {
                        tag: RedeemerTag::Reward,
                        index: index.try_into().unwrap(),
                    });
                    Ok(Certificate::ScriptCertificate(ScriptCertificate {
                        cert: certificate_type,
                        script_source: Some(ScriptSource::ProvidedScriptSource(
                            provided_script_source.clone(),
                        )),
                        redeemer: redeemer.cloned(),
                    }))
                }
                crate::tx_parser::context::Script::ReferencedNative(
                    inline_simple_script_source,
                ) => Ok(Certificate::SimpleScriptCertificate(
                    SimpleScriptCertificate {
                        cert: certificate_type,
                        simple_script_source: Some(SimpleScriptSource::InlineSimpleScriptSource(
                            inline_simple_script_source.clone(),
                        )),
                    },
                )),
                crate::tx_parser::context::Script::ReferencedPlutus(inline_script_source) => {
                    let redeemer = context.script_witnesses.redeemers.get(&RedeemerIndex {
                        tag: RedeemerTag::Reward,
                        index: index.try_into().unwrap(),
                    });
                    Ok(Certificate::ScriptCertificate(ScriptCertificate {
                        cert: certificate_type,
                        script_source: Some(ScriptSource::InlineScriptSource(
                            inline_script_source.clone(),
                        )),
                        redeemer: redeemer.cloned(),
                    }))
                }
            }
        }
        pallas::ledger::primitives::StakeCredential::AddrKeyhash(_) => {
            Ok(Certificate::BasicCertificate(certificate_type.clone()))
        }
    }
}

fn script_reward_account_from_hash(
    hash: &pallas::crypto::hash::Hash<28>,
) -> Result<RewardAccount, WError> {
    let mut bytes = hash.to_vec();
    // append the byte 11110001 at the beginning (for mainnet key hash)
    // the network is meaningless here, because once encoded, we only keep the hash anyways
    bytes.insert(0, 0b11110001);
    RewardAccount::from_bytes(&bytes)
}

fn key_reward_account_from_hash(
    hash: &pallas::crypto::hash::Hash<28>,
) -> Result<RewardAccount, WError> {
    let mut bytes = hash.to_vec();
    // append the byte 11100001 at the beginning (for mainnet script hash)
    // the network is meaningless here, because once encoded, we only keep the hash anyways
    bytes.insert(0, 0b11100001);
    RewardAccount::from_bytes(&bytes)
}

fn pallas_pool_params_to_whisky_pool_params(
    operator: PoolKeyhash,
    vrf_keyhash: VrfKeyhash,
    pledge: Coin,
    cost: Coin,
    margin: UnitInterval,
    reward_account: PallasRewardAccount,
    pool_owners: Set<AddrKeyhash>,
    relays: Vec<Relay>,
    pool_metadata: Option<PallasPoolMetadata>,
) -> Result<PoolParams, WError> {
    let mut whisky_relays: Vec<WhiskyRelay> = vec![];
    for relay in relays {
        match relay {
            Relay::SingleHostAddr(port, ipv4, ipv6) => {
                whisky_relays.push(WhiskyRelay::SingleHostAddr(SingleHostAddr {
                    ipv4: ipv4.map(|addr| {
                        let octets = addr.to_vec();
                        let ipv4_bytes = <[u8; 4]>::try_from(&octets[..4]).unwrap_or([0, 0, 0, 0]);
                        Ipv4Addr::from(ipv4_bytes).to_string()
                    }),
                    ipv6: ipv6.map(|addr| {
                        let octets = addr.to_vec();
                        let ipv6_bytes = <[u8; 16]>::try_from(&octets[..16]).unwrap_or([0; 16]);
                        Ipv6Addr::from(ipv6_bytes).to_string()
                    }),
                    port: port.map(|p| p as u16),
                }))
            }
            Relay::SingleHostName(port, dns_name) => {
                whisky_relays.push(WhiskyRelay::SingleHostName(SingleHostName {
                    domain_name: dns_name,
                    port: port.map(|p| p as u16),
                }))
            }
            Relay::MultiHostName(dns_name) => {
                whisky_relays.push(WhiskyRelay::MultiHostName(MultiHostName {
                    domain_name: dns_name,
                }))
            }
        }
    }

    Ok(PoolParams {
        vrf_key_hash: vrf_keyhash.to_string(),
        operator: operator.to_string(),
        pledge: pledge.to_string(),
        cost: cost.to_string(),
        margin: (margin.numerator, margin.denominator),
        relays: whisky_relays,
        owners: pool_owners.iter().map(|owner| owner.to_string()).collect(),
        reward_address: RewardAccount {
            inner: reward_account,
        }
        .to_bech32()?,
        metadata: pool_metadata.map(|metadata| PoolMetadata {
            url: metadata.url,
            hash: metadata.hash.to_string(),
        }),
    })
}

fn pallas_anchor_to_whisky_anchor(
    anchor: &pallas::ledger::primitives::conway::Anchor,
) -> whisky_common::Anchor {
    whisky_common::Anchor {
        anchor_url: anchor.url.clone(),
        anchor_data_hash: anchor.content_hash.to_string(),
    }
}

pub fn extract_certificates(
    pallas_tx: &Tx,
    context: &ParserContext,
) -> Result<Vec<Certificate>, WError> {
    let mut certs_vec: Vec<Certificate> = Vec::new();
    let pallas_certs = &pallas_tx.transaction_body.certificates;
    if let Some(certs) = pallas_certs {
        for (index, cert) in certs.iter().enumerate() {
            match cert {
                pallas::ledger::primitives::conway::Certificate::StakeRegistration(
                    stake_credential,
                ) => {
                    let stake_address = match stake_credential {
                        pallas::ledger::primitives::StakeCredential::ScriptHash(hash) => {
                            script_reward_account_from_hash(hash)?
                        }
                        pallas::ledger::primitives::StakeCredential::AddrKeyhash(hash) => {
                            key_reward_account_from_hash(hash)?
                        }
                    };
                    let cert_type: CertificateType =
                        CertificateType::RegisterStake(RegisterStake {
                            stake_key_address: stake_address.to_bech32()?,
                            coin: 0,
                        });

                    let cert =
                        handle_stake_credential(stake_credential, cert_type, context, index)?;
                    certs_vec.push(cert);
                }
                pallas::ledger::primitives::conway::Certificate::StakeDeregistration(
                    stake_credential,
                ) => {
                    let stake_address = match stake_credential {
                        pallas::ledger::primitives::StakeCredential::ScriptHash(hash) => {
                            script_reward_account_from_hash(hash)?
                        }
                        pallas::ledger::primitives::StakeCredential::AddrKeyhash(hash) => {
                            key_reward_account_from_hash(hash)?
                        }
                    };
                    let cert_type: CertificateType =
                        CertificateType::DeregisterStake(DeregisterStake {
                            stake_key_address: stake_address.to_bech32()?,
                        });
                    let cert =
                        handle_stake_credential(stake_credential, cert_type, context, index)?;
                    certs_vec.push(cert);
                }
                pallas::ledger::primitives::conway::Certificate::StakeDelegation(
                    stake_credential,
                    hash,
                ) => {
                    let stake_address = match stake_credential {
                        pallas::ledger::primitives::StakeCredential::ScriptHash(stake_hash) => {
                            script_reward_account_from_hash(stake_hash)?
                        }
                        pallas::ledger::primitives::StakeCredential::AddrKeyhash(stake_hash) => {
                            key_reward_account_from_hash(stake_hash)?
                        }
                    };
                    let cert_type: CertificateType =
                        CertificateType::DelegateStake(DelegateStake {
                            stake_key_address: stake_address.to_bech32()?,
                            pool_id: bech32::encode::<bech32::Bech32>(
                                bech32::Hrp::parse("pool").unwrap(),
                                &hex::decode(hash.to_string()).unwrap(),
                            )
                            .map_err(|_| {
                                WError::new(
                                    "WhiskyPallas Parser - Extracting certificates:",
                                    &format!(
                                        "Failed to encode pool id from hash: {}",
                                        hash.to_string()
                                    ),
                                )
                            })?,
                        });

                    let cert =
                        handle_stake_credential(stake_credential, cert_type, context, index)?;
                    certs_vec.push(cert);
                }
                pallas::ledger::primitives::conway::Certificate::PoolRegistration {
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
                    let pool_params: PoolParams = pallas_pool_params_to_whisky_pool_params(
                        operator.clone(),
                        vrf_keyhash.clone(),
                        pledge.clone(),
                        cost.clone(),
                        margin.clone(),
                        reward_account.clone(),
                        pool_owners.clone(),
                        relays.clone(),
                        pool_metadata.clone(),
                    )?;

                    let cert_type = CertificateType::RegisterPool(RegisterPool { pool_params });
                    certs_vec.push(Certificate::BasicCertificate(cert_type));
                }
                pallas::ledger::primitives::conway::Certificate::PoolRetirement(hash, epoch) => {
                    let cert =
                        Certificate::BasicCertificate(CertificateType::RetirePool(RetirePool {
                            pool_id: bech32::encode::<bech32::Bech32>(
                                bech32::Hrp::parse("pool").unwrap(),
                                &hex::decode(hash.to_string()).unwrap(),
                            )
                            .map_err(|_| {
                                WError::new(
                                    "WhiskyPallas Parser - Extracting certificates:",
                                    &format!(
                                        "Failed to encode pool id from hash: {}",
                                        hash.to_string()
                                    ),
                                )
                            })?,
                            epoch: *epoch as u32,
                        }));
                    certs_vec.push(cert);
                }
                pallas::ledger::primitives::conway::Certificate::Reg(stake_credential, coin) => {
                    let stake_address = match stake_credential {
                        pallas::ledger::primitives::StakeCredential::ScriptHash(hash) => {
                            script_reward_account_from_hash(hash)?
                        }
                        pallas::ledger::primitives::StakeCredential::AddrKeyhash(hash) => {
                            key_reward_account_from_hash(hash)?
                        }
                    };
                    let cert_type = CertificateType::RegisterStake(RegisterStake {
                        stake_key_address: stake_address.to_bech32()?,
                        coin: coin.clone(),
                    });

                    let cert =
                        handle_stake_credential(stake_credential, cert_type, context, index)?;
                    certs_vec.push(cert);
                }
                pallas::ledger::primitives::conway::Certificate::UnReg(stake_credential, _) => {
                    let stake_address = match stake_credential {
                        pallas::ledger::primitives::StakeCredential::ScriptHash(hash) => {
                            script_reward_account_from_hash(hash)?
                        }
                        pallas::ledger::primitives::StakeCredential::AddrKeyhash(hash) => {
                            key_reward_account_from_hash(hash)?
                        }
                    };
                    let cert_type = CertificateType::DeregisterStake(DeregisterStake {
                        stake_key_address: stake_address.to_bech32()?,
                    });

                    let cert =
                        handle_stake_credential(stake_credential, cert_type, context, index)?;
                    certs_vec.push(cert);
                }
                pallas::ledger::primitives::conway::Certificate::VoteDeleg(
                    stake_credential,
                    drep,
                ) => {
                    let whisky_drep: WhiskyDRep = match drep {
                        pallas::ledger::primitives::conway::DRep::Key(_hash) => WhiskyDRep::DRepId(
                            DRep {
                                inner: drep.clone(),
                            }
                            .to_bech32_cip129()?,
                        ),
                        pallas::ledger::primitives::conway::DRep::Script(_hash) => {
                            WhiskyDRep::DRepId(
                                DRep {
                                    inner: drep.clone(),
                                }
                                .to_bech32_cip129()?,
                            )
                        }
                        pallas::ledger::primitives::conway::DRep::Abstain => {
                            WhiskyDRep::AlwaysAbstain
                        }
                        pallas::ledger::primitives::conway::DRep::NoConfidence => {
                            WhiskyDRep::AlwaysNoConfidence
                        }
                    };
                    let stake_address = match stake_credential {
                        pallas::ledger::primitives::StakeCredential::ScriptHash(hash) => {
                            script_reward_account_from_hash(hash)?
                        }
                        pallas::ledger::primitives::StakeCredential::AddrKeyhash(hash) => {
                            key_reward_account_from_hash(hash)?
                        }
                    };
                    let cert_type = CertificateType::VoteDelegation(VoteDelegation {
                        stake_key_address: stake_address.to_bech32()?,
                        drep: whisky_drep,
                    });

                    let cert =
                        handle_stake_credential(stake_credential, cert_type, context, index)?;
                    certs_vec.push(cert);
                }
                pallas::ledger::primitives::conway::Certificate::StakeVoteDeleg(
                    stake_credential,
                    hash,
                    drep,
                ) => {
                    let whisky_drep: WhiskyDRep = match drep {
                        pallas::ledger::primitives::conway::DRep::Key(_hash) => WhiskyDRep::DRepId(
                            DRep {
                                inner: drep.clone(),
                            }
                            .to_bech32_cip129()?,
                        ),
                        pallas::ledger::primitives::conway::DRep::Script(_hash) => {
                            WhiskyDRep::DRepId(
                                DRep {
                                    inner: drep.clone(),
                                }
                                .to_bech32_cip129()?,
                            )
                        }
                        pallas::ledger::primitives::conway::DRep::Abstain => {
                            WhiskyDRep::AlwaysAbstain
                        }
                        pallas::ledger::primitives::conway::DRep::NoConfidence => {
                            WhiskyDRep::AlwaysNoConfidence
                        }
                    };
                    let stake_address = match stake_credential {
                        pallas::ledger::primitives::StakeCredential::ScriptHash(hash) => {
                            script_reward_account_from_hash(hash)?
                        }
                        pallas::ledger::primitives::StakeCredential::AddrKeyhash(hash) => {
                            key_reward_account_from_hash(hash)?
                        }
                    };
                    let cert_type =
                        CertificateType::StakeAndVoteDelegation(StakeAndVoteDelegation {
                            stake_key_address: stake_address.to_bech32()?,
                            pool_key_hash: hash.to_string(),
                            drep: whisky_drep,
                        });

                    let cert =
                        handle_stake_credential(stake_credential, cert_type, context, index)?;
                    certs_vec.push(cert);
                }
                pallas::ledger::primitives::conway::Certificate::StakeRegDeleg(
                    stake_credential,
                    hash,
                    coin,
                ) => {
                    let stake_address = match stake_credential {
                        pallas::ledger::primitives::StakeCredential::ScriptHash(stake_hash) => {
                            script_reward_account_from_hash(stake_hash)?
                        }
                        pallas::ledger::primitives::StakeCredential::AddrKeyhash(stake_hash) => {
                            key_reward_account_from_hash(stake_hash)?
                        }
                    };
                    let cert_type = CertificateType::StakeRegistrationAndDelegation(
                        StakeRegistrationAndDelegation {
                            stake_key_address: stake_address.to_bech32()?,
                            pool_key_hash: hash.to_string(),
                            coin: coin.clone(),
                        },
                    );
                    let cert =
                        handle_stake_credential(stake_credential, cert_type, context, index)?;
                    certs_vec.push(cert);
                }
                pallas::ledger::primitives::conway::Certificate::VoteRegDeleg(
                    stake_credential,
                    drep,
                    coin,
                ) => {
                    let whisky_drep: WhiskyDRep = match drep {
                        pallas::ledger::primitives::conway::DRep::Key(_hash) => WhiskyDRep::DRepId(
                            DRep {
                                inner: drep.clone(),
                            }
                            .to_bech32_cip129()?,
                        ),
                        pallas::ledger::primitives::conway::DRep::Script(_hash) => {
                            WhiskyDRep::DRepId(
                                DRep {
                                    inner: drep.clone(),
                                }
                                .to_bech32_cip129()?,
                            )
                        }
                        pallas::ledger::primitives::conway::DRep::Abstain => {
                            WhiskyDRep::AlwaysAbstain
                        }
                        pallas::ledger::primitives::conway::DRep::NoConfidence => {
                            WhiskyDRep::AlwaysNoConfidence
                        }
                    };
                    let stake_address = match stake_credential {
                        pallas::ledger::primitives::StakeCredential::ScriptHash(hash) => {
                            script_reward_account_from_hash(hash)?
                        }
                        pallas::ledger::primitives::StakeCredential::AddrKeyhash(hash) => {
                            key_reward_account_from_hash(hash)?
                        }
                    };
                    let cert_type = CertificateType::VoteRegistrationAndDelegation(
                        VoteRegistrationAndDelegation {
                            stake_key_address: stake_address.to_bech32()?,
                            drep: whisky_drep,
                            coin: coin.clone(),
                        },
                    );

                    let cert =
                        handle_stake_credential(stake_credential, cert_type, context, index)?;
                    certs_vec.push(cert);
                }
                pallas::ledger::primitives::conway::Certificate::StakeVoteRegDeleg(
                    stake_credential,
                    hash,
                    drep,
                    coin,
                ) => {
                    let whisky_drep: WhiskyDRep = match drep {
                        pallas::ledger::primitives::conway::DRep::Key(_hash) => WhiskyDRep::DRepId(
                            DRep {
                                inner: drep.clone(),
                            }
                            .to_bech32_cip129()?,
                        ),
                        pallas::ledger::primitives::conway::DRep::Script(_hash) => {
                            WhiskyDRep::DRepId(
                                DRep {
                                    inner: drep.clone(),
                                }
                                .to_bech32_cip129()?,
                            )
                        }
                        pallas::ledger::primitives::conway::DRep::Abstain => {
                            WhiskyDRep::AlwaysAbstain
                        }
                        pallas::ledger::primitives::conway::DRep::NoConfidence => {
                            WhiskyDRep::AlwaysNoConfidence
                        }
                    };
                    let stake_address = match stake_credential {
                        pallas::ledger::primitives::StakeCredential::ScriptHash(stake_hash) => {
                            script_reward_account_from_hash(stake_hash)?
                        }
                        pallas::ledger::primitives::StakeCredential::AddrKeyhash(stake_hash) => {
                            key_reward_account_from_hash(stake_hash)?
                        }
                    };
                    let cert_type = CertificateType::StakeVoteRegistrationAndDelegation(
                        StakeVoteRegistrationAndDelegation {
                            stake_key_address: stake_address.to_bech32()?,
                            pool_key_hash: hash.to_string(),
                            coin: coin.clone(),
                            drep: whisky_drep,
                        },
                    );

                    let cert =
                        handle_stake_credential(stake_credential, cert_type, context, index)?;
                    certs_vec.push(cert);
                }
                pallas::ledger::primitives::conway::Certificate::AuthCommitteeHot(
                    cold_stake_cred,
                    hot_stake_cred,
                ) => {
                    let cold_stake_address = match cold_stake_cred {
                        pallas::ledger::primitives::StakeCredential::ScriptHash(hash) => {
                            script_reward_account_from_hash(hash)?.to_bech32()?
                        }
                        pallas::ledger::primitives::StakeCredential::AddrKeyhash(hash) => {
                            key_reward_account_from_hash(hash)?.to_bech32()?
                        }
                    };
                    let hot_stake_address = match hot_stake_cred {
                        pallas::ledger::primitives::StakeCredential::ScriptHash(hash) => {
                            script_reward_account_from_hash(hash)?.to_bech32()?
                        }
                        pallas::ledger::primitives::StakeCredential::AddrKeyhash(hash) => {
                            key_reward_account_from_hash(hash)?.to_bech32()?
                        }
                    };
                    let cert_type = CertificateType::CommitteeHotAuth(CommitteeHotAuth {
                        committee_cold_key_address: cold_stake_address,
                        committee_hot_key_address: hot_stake_address,
                    });
                    let cert = handle_stake_credential(cold_stake_cred, cert_type, context, index)?;
                    certs_vec.push(cert);
                }
                pallas::ledger::primitives::conway::Certificate::ResignCommitteeCold(
                    stake_credential,
                    anchor,
                ) => {
                    let stake_address = match stake_credential {
                        pallas::ledger::primitives::StakeCredential::ScriptHash(hash) => {
                            script_reward_account_from_hash(hash)?
                        }
                        pallas::ledger::primitives::StakeCredential::AddrKeyhash(hash) => {
                            key_reward_account_from_hash(hash)?
                        }
                    };
                    let cert_type = CertificateType::CommitteeColdResign(CommitteeColdResign {
                        committee_cold_key_address: stake_address.to_bech32()?,
                        anchor: anchor.clone().map(|a| pallas_anchor_to_whisky_anchor(&a)),
                    });
                    let cert =
                        handle_stake_credential(stake_credential, cert_type, context, index)?;
                    certs_vec.push(cert);
                }
                pallas::ledger::primitives::conway::Certificate::RegDRepCert(
                    stake_credential,
                    coin,
                    anchor,
                ) => {
                    let drep_id = match stake_credential {
                        pallas::ledger::primitives::StakeCredential::ScriptHash(hash) => {
                            DRep::new(DRepKind::Script {
                                script_hash: hash.to_string(),
                            })
                        }
                        pallas::ledger::primitives::StakeCredential::AddrKeyhash(hash) => {
                            DRep::new(DRepKind::Key {
                                addr_key_hash: hash.to_string(),
                            })
                        }
                    }?;
                    let cert_type = CertificateType::DRepRegistration(DRepRegistration {
                        drep_id: drep_id.to_bech32_cip129()?,
                        anchor: anchor.clone().map(|a| pallas_anchor_to_whisky_anchor(&a)),
                        coin: coin.clone(),
                    });
                    let cert =
                        handle_stake_credential(stake_credential, cert_type, context, index)?;
                    certs_vec.push(cert);
                }
                pallas::ledger::primitives::conway::Certificate::UnRegDRepCert(
                    stake_credential,
                    coin,
                ) => {
                    let drep_id = match stake_credential {
                        pallas::ledger::primitives::StakeCredential::ScriptHash(hash) => {
                            DRep::new(DRepKind::Script {
                                script_hash: hash.to_string(),
                            })
                        }
                        pallas::ledger::primitives::StakeCredential::AddrKeyhash(hash) => {
                            DRep::new(DRepKind::Key {
                                addr_key_hash: hash.to_string(),
                            })
                        }
                    }?;
                    let cert_type = CertificateType::DRepDeregistration(DRepDeregistration {
                        drep_id: drep_id.to_bech32_cip129()?,
                        coin: coin.clone(),
                    });
                    let cert =
                        handle_stake_credential(stake_credential, cert_type, context, index)?;
                    certs_vec.push(cert);
                }
                pallas::ledger::primitives::conway::Certificate::UpdateDRepCert(
                    stake_credential,
                    anchor,
                ) => {
                    let drep_id = match stake_credential {
                        pallas::ledger::primitives::StakeCredential::ScriptHash(hash) => {
                            DRep::new(DRepKind::Script {
                                script_hash: hash.to_string(),
                            })
                        }
                        pallas::ledger::primitives::StakeCredential::AddrKeyhash(hash) => {
                            DRep::new(DRepKind::Key {
                                addr_key_hash: hash.to_string(),
                            })
                        }
                    }?;
                    let cert_type = CertificateType::DRepUpdate(DRepUpdate {
                        drep_id: drep_id.to_bech32_cip129()?,
                        anchor: anchor.clone().map(|a| pallas_anchor_to_whisky_anchor(&a)),
                    });
                    let cert =
                        handle_stake_credential(stake_credential, cert_type, context, index)?;
                    certs_vec.push(cert);
                }
            }
        }
    }
    Ok(certs_vec)
}
