use std::net::{Ipv4Addr, Ipv6Addr};

use cardano_serialization_lib::{self as csl};
use whisky_common::{
    Anchor, Certificate, CertificateType, CommitteeColdResign, CommitteeHotAuth, DRep,
    DRepDeregistration, DRepRegistration, DRepUpdate, DelegateStake, DeregisterStake,
    MultiHostName, PoolMetadata, PoolParams, RegisterPool, RegisterStake, Relay, RetirePool,
    ScriptCertificate, ScriptSource, SimpleScriptCertificate, SimpleScriptSource, SingleHostAddr,
    SingleHostName, StakeAndVoteDelegation, StakeRegistrationAndDelegation,
    StakeVoteRegistrationAndDelegation, VoteDelegation, VoteRegistrationAndDelegation, WError,
};

use crate::tx_parser::context::{RedeemerIndex, Script};

use super::CSLParser;

impl CSLParser {
    pub fn get_certificates(&self) -> &Vec<Certificate> {
        &self.tx_body.certificates
    }

    pub(super) fn extract_certificates(&mut self) -> Result<(), WError> {
        let certs = self.csl_tx_body.certs();
        if certs.is_none() {
            return Ok(());
        }

        let certs = certs.unwrap();
        let mut result = Vec::new();
        let len = certs.len();

        for i in 0..len {
            let cert = certs.get(i);

            let script_hash = get_script_credential_from_cert(&cert);
            let script_witness = script_hash
                .map(|sh| self.context.script_witness.scripts.get(&sh))
                .flatten()
                .cloned();
            let redeemer = self
                .context
                .script_witness
                .redeemers
                .get(&RedeemerIndex::Cert(i))
                .cloned();
            let certificate_type =
                csl_cert_to_certificate_type(&cert, &csl::NetworkInfo::mainnet())?;
            if let Some(certificate_type) = certificate_type {
                if let Some(script_witness) = script_witness {
                    match script_witness {
                        Script::ProvidedNative(native_script) => {
                            result.push(Certificate::SimpleScriptCertificate(
                                SimpleScriptCertificate {
                                    cert: certificate_type,
                                    simple_script_source: Some(
                                        SimpleScriptSource::ProvidedSimpleScriptSource(
                                            native_script.clone(),
                                        ),
                                    ),
                                },
                            ));
                        }
                        Script::ProvidedPlutus(plutus_script) => {
                            result.push(Certificate::ScriptCertificate(ScriptCertificate {
                                cert: certificate_type,
                                script_source: Some(ScriptSource::ProvidedScriptSource(
                                    plutus_script,
                                )),
                                redeemer,
                            }));
                        }
                        Script::ReferencedNative(inline_script) => {
                            result.push(Certificate::SimpleScriptCertificate(
                                SimpleScriptCertificate {
                                    cert: certificate_type,
                                    simple_script_source: Some(
                                        SimpleScriptSource::InlineSimpleScriptSource(
                                            inline_script.clone(),
                                        ),
                                    ),
                                },
                            ));
                        }
                        Script::ReferencedPlutus(inline_script) => {
                            result.push(Certificate::ScriptCertificate(ScriptCertificate {
                                cert: certificate_type,
                                script_source: Some(ScriptSource::InlineScriptSource(
                                    inline_script,
                                )),
                                redeemer,
                            }));
                        }
                    }
                } else {
                    result.push(Certificate::BasicCertificate(certificate_type));
                }
            };
        }
        self.tx_body.certificates = result;
        Ok(())
    }
}

fn get_script_credential_from_cert(cert: &csl::Certificate) -> Option<csl::ScriptHash> {
    match cert.kind() {
        csl::CertificateKind::StakeRegistration => cert
            .as_stake_registration()
            .map(|reg| reg.stake_credential())
            .map(|cred| cred.to_scripthash())
            .flatten(),
        csl::CertificateKind::StakeDeregistration => cert
            .as_stake_deregistration()
            .map(|dereg| dereg.stake_credential())
            .map(|cred| cred.to_scripthash())
            .flatten(),
        csl::CertificateKind::StakeDelegation => cert
            .as_stake_delegation()
            .map(|deleg| deleg.stake_credential())
            .map(|cred| cred.to_scripthash())
            .flatten(),
        csl::CertificateKind::StakeAndVoteDelegation => cert
            .as_stake_and_vote_delegation()
            .map(|deleg| deleg.stake_credential())
            .map(|cred| cred.to_scripthash())
            .flatten(),
        csl::CertificateKind::StakeRegistrationAndDelegation => cert
            .as_stake_registration_and_delegation()
            .map(|reg| reg.stake_credential())
            .map(|cred| cred.to_scripthash())
            .flatten(),
        csl::CertificateKind::StakeVoteRegistrationAndDelegation => cert
            .as_stake_vote_registration_and_delegation()
            .map(|reg| reg.stake_credential())
            .map(|cred| cred.to_scripthash())
            .flatten(),
        csl::CertificateKind::CommitteeHotAuth => cert
            .as_committee_hot_auth()
            .map(|auth| auth.committee_hot_credential())
            .map(|cred| cred.to_scripthash())
            .flatten(),
        csl::CertificateKind::CommitteeColdResign => cert
            .as_committee_cold_resign()
            .map(|resign| resign.committee_cold_credential())
            .map(|cred| cred.to_scripthash())
            .flatten(),
        csl::CertificateKind::DRepDeregistration => cert
            .as_drep_deregistration()
            .map(|dereg| dereg.voting_credential())
            .map(|cred| cred.to_scripthash())
            .flatten(),
        csl::CertificateKind::DRepRegistration => cert
            .as_drep_registration()
            .map(|reg| reg.voting_credential())
            .map(|cred| cred.to_scripthash())
            .flatten(),
        csl::CertificateKind::DRepUpdate => cert
            .as_drep_update()
            .map(|update| update.voting_credential())
            .map(|cred| cred.to_scripthash())
            .flatten(),
        csl::CertificateKind::VoteDelegation => cert
            .as_vote_delegation()
            .map(|deleg| deleg.stake_credential())
            .map(|cred| cred.to_scripthash())
            .flatten(),
        csl::CertificateKind::VoteRegistrationAndDelegation => cert
            .as_vote_registration_and_delegation()
            .map(|reg| reg.stake_credential())
            .map(|cred| cred.to_scripthash())
            .flatten(),
        csl::CertificateKind::PoolRegistration => None,
        csl::CertificateKind::PoolRetirement => None,
        csl::CertificateKind::GenesisKeyDelegation => None,
        csl::CertificateKind::MoveInstantaneousRewardsCert => None,
    }
}

fn csl_drep_to_drep(csl_drep: &csl::DRep) -> Result<DRep, WError> {
    match csl_drep.kind() {
        csl::DRepKind::KeyHash => {
            let drep_id = csl_drep.to_bech32(true).map_err(|e| {
                WError::new(
                    "csl_drep_to_drep",
                    &format!("Failed to convert drep to bech32: {:?}", e),
                )
            })?;
            Ok(DRep::DRepId(drep_id))
        }
        csl::DRepKind::AlwaysAbstain => Ok(DRep::AlwaysAbstain),
        csl::DRepKind::AlwaysNoConfidence => Ok(DRep::AlwaysNoConfidence),
        csl::DRepKind::ScriptHash => {
            let drep_id = csl_drep.to_bech32(true).map_err(|e| {
                WError::new(
                    "csl_drep_to_drep",
                    &format!("Failed to convert drep to bech32: {:?}", e),
                )
            })?;
            Ok(DRep::DRepId(drep_id))
        }
    }
}

fn csl_cert_to_certificate_type(
    cert: &csl::Certificate,
    network_id: &csl::NetworkInfo,
) -> Result<Option<CertificateType>, WError> {
    match cert.kind() {
        csl::CertificateKind::StakeRegistration => {
            let reg = cert.as_stake_registration().ok_or_else(|| {
                WError::new(
                    "csl_cert_to_certificate_type",
                    "Failed to get stake registration",
                )
            })?;
            let stake_cred = reg.stake_credential();
            let stake_key_address = csl::RewardAddress::new(network_id.network_id(), &stake_cred)
                .to_address()
                .to_bech32(None)
                .map_err(|e| {
                    WError::new(
                        "csl_cert_to_certificate_type",
                        &format!("Failed to convert address to bech32: {:?}", e),
                    )
                })?;
            Ok(Some(CertificateType::RegisterStake(RegisterStake {
                stake_key_address,
                coin: 0,
            })))
        }
        csl::CertificateKind::StakeDeregistration => {
            let dereg = cert.as_stake_deregistration().ok_or_else(|| {
                WError::new(
                    "csl_cert_to_certificate_type",
                    "Failed to get stake deregistration",
                )
            })?;
            let stake_cred = dereg.stake_credential();
            let stake_key_address = csl::RewardAddress::new(network_id.network_id(), &stake_cred)
                .to_address()
                .to_bech32(None)
                .map_err(|e| {
                    WError::new(
                        "csl_cert_to_certificate_type",
                        &format!("Failed to convert address to bech32: {:?}", e),
                    )
                })?;
            Ok(Some(CertificateType::DeregisterStake(DeregisterStake {
                stake_key_address,
            })))
        }
        csl::CertificateKind::StakeDelegation => {
            let deleg = cert.as_stake_delegation().ok_or_else(|| {
                WError::new(
                    "csl_cert_to_certificate_type",
                    "Failed to get stake delegation",
                )
            })?;
            let stake_cred = deleg.stake_credential();
            let stake_key_address = csl::RewardAddress::new(network_id.network_id(), &stake_cred)
                .to_address()
                .to_bech32(None)
                .map_err(|e| {
                    WError::new(
                        "csl_cert_to_certificate_type",
                        &format!("Failed to convert address to bech32: {:?}", e),
                    )
                })?;
            let pool_id = deleg.pool_keyhash().to_hex();
            Ok(Some(CertificateType::DelegateStake(DelegateStake {
                stake_key_address,
                pool_id,
            })))
        }
        csl::CertificateKind::PoolRegistration => {
            let pool_reg = cert.as_pool_registration().ok_or_else(|| {
                WError::new(
                    "csl_cert_to_certificate_type",
                    "Failed to get pool registration",
                )
            })?;
            let pool_params = pool_reg.pool_params();
            let mapped_pool_params = csl_pool_params_to_pool_params(&pool_params)?;
            Ok(Some(CertificateType::RegisterPool(RegisterPool {
                pool_params: mapped_pool_params,
            })))
        }
        csl::CertificateKind::PoolRetirement => {
            let pool_ret = cert.as_pool_retirement().ok_or_else(|| {
                WError::new(
                    "csl_cert_to_certificate_type",
                    "Failed to get pool retirement",
                )
            })?;
            let pool_id = pool_ret.pool_keyhash().to_hex();
            let epoch = pool_ret.epoch();
            Ok(Some(CertificateType::RetirePool(RetirePool {
                pool_id,
                epoch,
            })))
        }
        csl::CertificateKind::GenesisKeyDelegation => Ok(None),
        csl::CertificateKind::MoveInstantaneousRewardsCert => Ok(None),
        csl::CertificateKind::CommitteeHotAuth => {
            let committee_hot_auth = cert.as_committee_hot_auth().ok_or_else(|| {
                WError::new(
                    "csl_cert_to_certificate_type",
                    "Failed to get committee hot auth",
                )
            })?;
            let committee_cold_credential = committee_hot_auth.committee_cold_credential().to_hex();
            let committee_hot_credential = committee_hot_auth.committee_hot_credential().to_hex();
            Ok(Some(CertificateType::CommitteeHotAuth(CommitteeHotAuth {
                committee_cold_key_address: committee_cold_credential,
                committee_hot_key_address: committee_hot_credential,
            })))
        }
        csl::CertificateKind::CommitteeColdResign => {
            let committee_cold_resign = cert.as_committee_cold_resign().ok_or_else(|| {
                WError::new(
                    "csl_cert_to_certificate_type",
                    "Failed to get committee cold resign",
                )
            })?;
            let committee_cold_credential =
                committee_cold_resign.committee_cold_credential().to_hex();
            let anchor = committee_cold_resign.anchor().map(|a| Anchor {
                anchor_url: a.url().url(),
                anchor_data_hash: a.anchor_data_hash().to_hex(),
            });
            Ok(Some(CertificateType::CommitteeColdResign(
                CommitteeColdResign {
                    committee_cold_key_address: committee_cold_credential,
                    anchor,
                },
            )))
        }
        csl::CertificateKind::DRepDeregistration => {
            let drep_dereg = cert.as_drep_deregistration().ok_or_else(|| {
                WError::new(
                    "csl_cert_to_certificate_type",
                    "Failed to get drep deregistration",
                )
            })?;
            let drep = csl::DRep::new_from_credential(&drep_dereg.voting_credential());
            let drep_id = drep.to_bech32(true).map_err(|e| {
                WError::new(
                    "csl_cert_to_certificate_type",
                    &format!("Failed to convert drep to bech32: {:?}", e),
                )
            })?;
            let coin = drep_dereg.coin().to_str().parse::<u64>().unwrap_or(0);
            Ok(Some(CertificateType::DRepDeregistration(
                DRepDeregistration { drep_id, coin },
            )))
        }
        csl::CertificateKind::DRepRegistration => {
            let drep_reg = cert.as_drep_registration().ok_or_else(|| {
                WError::new(
                    "csl_cert_to_certificate_type",
                    "Failed to get drep registration",
                )
            })?;
            let drep = csl::DRep::new_from_credential(&drep_reg.voting_credential());
            let drep_id = drep.to_bech32(true).map_err(|e| {
                WError::new(
                    "csl_cert_to_certificate_type",
                    &format!("Failed to convert drep to bech32: {:?}", e),
                )
            })?;
            let coin = drep_reg.coin().to_str().parse::<u64>().unwrap_or(0);
            let anchor = drep_reg.anchor().map(|a| Anchor {
                anchor_url: a.url().url(),
                anchor_data_hash: a.anchor_data_hash().to_hex(),
            });
            Ok(Some(CertificateType::DRepRegistration(DRepRegistration {
                drep_id,
                coin,
                anchor,
            })))
        }
        csl::CertificateKind::DRepUpdate => {
            let drep_update = cert.as_drep_update().ok_or_else(|| {
                WError::new("csl_cert_to_certificate_type", "Failed to get drep update")
            })?;
            let drep = csl::DRep::new_from_credential(&drep_update.voting_credential());
            let drep_id = drep.to_bech32(true).map_err(|e| {
                WError::new(
                    "csl_cert_to_certificate_type",
                    &format!("Failed to convert drep to bech32: {:?}", e),
                )
            })?;
            let anchor = drep_update.anchor().map(|a| Anchor {
                anchor_url: a.url().url(),
                anchor_data_hash: a.anchor_data_hash().to_hex(),
            });
            Ok(Some(CertificateType::DRepUpdate(DRepUpdate {
                drep_id,
                anchor,
            })))
        }
        csl::CertificateKind::VoteDelegation => {
            let vote_deleg = cert.as_vote_delegation().ok_or_else(|| {
                WError::new(
                    "csl_cert_to_certificate_type",
                    "Failed to get vote delegation",
                )
            })?;
            let stake_cred = vote_deleg.stake_credential();
            let stake_key_address = csl::RewardAddress::new(network_id.network_id(), &stake_cred)
                .to_address()
                .to_bech32(None)
                .map_err(|e| {
                    WError::new(
                        "csl_cert_to_certificate_type",
                        &format!("Failed to convert address to bech32: {:?}", e),
                    )
                })?;
            let drep = csl_drep_to_drep(&vote_deleg.drep())?;
            Ok(Some(CertificateType::VoteDelegation(VoteDelegation {
                stake_key_address,
                drep,
            })))
        }
        csl::CertificateKind::StakeAndVoteDelegation => {
            let stake_and_vote_deleg = cert.as_stake_and_vote_delegation().ok_or_else(|| {
                WError::new(
                    "csl_cert_to_certificate_type",
                    "Failed to get stake and vote delegation",
                )
            })?;
            let stake_key_address = csl::RewardAddress::new(
                network_id.network_id(),
                &stake_and_vote_deleg.stake_credential(),
            )
            .to_address()
            .to_bech32(None)
            .map_err(|e| {
                WError::new(
                    "csl_cert_to_certificate_type",
                    &format!("Failed to convert address to bech32: {:?}", e),
                )
            })?;
            let pool_key_hash = stake_and_vote_deleg.pool_keyhash().to_hex();
            let drep = csl_drep_to_drep(&stake_and_vote_deleg.drep())?;
            Ok(Some(CertificateType::StakeAndVoteDelegation(
                StakeAndVoteDelegation {
                    stake_key_address,
                    drep,
                    pool_key_hash,
                },
            )))
        }
        csl::CertificateKind::StakeRegistrationAndDelegation => {
            let stake_reg_and_deleg =
                cert.as_stake_registration_and_delegation().ok_or_else(|| {
                    WError::new(
                        "csl_cert_to_certificate_type",
                        "Failed to get stake registration and delegation",
                    )
                })?;
            let stake_key_address = csl::RewardAddress::new(
                network_id.network_id(),
                &stake_reg_and_deleg.stake_credential(),
            )
            .to_address()
            .to_bech32(None)
            .map_err(|e| {
                WError::new(
                    "csl_cert_to_certificate_type",
                    &format!("Failed to convert address to bech32: {:?}", e),
                )
            })?;
            let pool_key_hash = stake_reg_and_deleg.pool_keyhash().to_hex();
            let coin = stake_reg_and_deleg
                .coin()
                .to_str()
                .parse::<u64>()
                .unwrap_or(0);
            Ok(Some(CertificateType::StakeRegistrationAndDelegation(
                StakeRegistrationAndDelegation {
                    stake_key_address,
                    pool_key_hash,
                    coin,
                },
            )))
        }
        csl::CertificateKind::StakeVoteRegistrationAndDelegation => {
            let stake_vote_reg_and_deleg = cert
                .as_stake_vote_registration_and_delegation()
                .ok_or_else(|| {
                    WError::new(
                        "csl_cert_to_certificate_type",
                        "Failed to get stake vote registration and delegation",
                    )
                })?;
            let stake_key_address = csl::RewardAddress::new(
                network_id.network_id(),
                &stake_vote_reg_and_deleg.stake_credential(),
            )
            .to_address()
            .to_bech32(None)
            .map_err(|e| {
                WError::new(
                    "csl_cert_to_certificate_type",
                    &format!("Failed to convert address to bech32: {:?}", e),
                )
            })?;
            let pool_key_hash = stake_vote_reg_and_deleg.pool_keyhash().to_hex();
            let coin = stake_vote_reg_and_deleg
                .coin()
                .to_str()
                .parse::<u64>()
                .unwrap_or(0);
            let drep = csl_drep_to_drep(&stake_vote_reg_and_deleg.drep())?;
            Ok(Some(CertificateType::StakeVoteRegistrationAndDelegation(
                StakeVoteRegistrationAndDelegation {
                    stake_key_address,
                    pool_key_hash,
                    coin,
                    drep,
                },
            )))
        }
        csl::CertificateKind::VoteRegistrationAndDelegation => {
            let vote_reg_and_deleg =
                cert.as_vote_registration_and_delegation().ok_or_else(|| {
                    WError::new(
                        "csl_cert_to_certificate_type",
                        "Failed to get vote registration and delegation",
                    )
                })?;
            let stake_key_address = csl::RewardAddress::new(
                network_id.network_id(),
                &vote_reg_and_deleg.stake_credential(),
            )
            .to_address()
            .to_bech32(None)
            .map_err(|e| {
                WError::new(
                    "csl_cert_to_certificate_type",
                    &format!("Failed to convert address to bech32: {:?}", e),
                )
            })?;
            let drep = csl_drep_to_drep(&vote_reg_and_deleg.drep())?;
            let coin = vote_reg_and_deleg
                .coin()
                .to_str()
                .parse::<u64>()
                .unwrap_or(0);
            Ok(Some(CertificateType::VoteRegistrationAndDelegation(
                VoteRegistrationAndDelegation {
                    stake_key_address,
                    drep,
                    coin,
                },
            )))
        }
    }
}

pub fn csl_pool_params_to_pool_params(pool_params: &csl::PoolParams) -> Result<PoolParams, WError> {
    let vrf_key_hash = pool_params.vrf_keyhash().to_hex();
    let operator = pool_params.operator().to_hex();
    let pledge = pool_params.pledge().to_str();
    let cost = pool_params.cost().to_str();
    let margin = (
        pool_params
            .margin()
            .numerator()
            .to_str()
            .parse::<u64>()
            .unwrap_or(0),
        pool_params
            .margin()
            .denominator()
            .to_str()
            .parse::<u64>()
            .unwrap_or(1),
    );
    let reward_address = pool_params
        .reward_account()
        .to_address()
        .to_bech32(None)
        .map_err(|e| {
            WError::new(
                "csl_pool_params_to_pool_params",
                &format!("Failed to convert reward address to bech32: {:?}", e),
            )
        })?;

    let pool_owners = (0..pool_params.pool_owners().len())
        .map(|i| pool_params.pool_owners().get(i).to_hex())
        .collect();

    let mut relays = Vec::new();
    let csl_relays = pool_params.relays();
    let relays_len = csl_relays.len();
    for i in 0..relays_len {
        let relay = csl_relays.get(i);
        let relay = csl_relay_to_relay(&relay)?;
        relays.push(relay);
    }

    let metadata = pool_params.pool_metadata().map(|metadata| PoolMetadata {
        url: metadata.url().url(),
        hash: metadata.pool_metadata_hash().to_hex(),
    });

    Ok(PoolParams {
        vrf_key_hash,
        operator,
        pledge,
        cost,
        margin,
        relays,
        owners: pool_owners,
        reward_address,
        metadata,
    })
}

pub fn csl_relay_to_relay(relay: &csl::Relay) -> Result<Relay, WError> {
    match relay.kind() {
        csl::RelayKind::SingleHostAddr => {
            let single_host_addr = relay.as_single_host_addr().ok_or_else(|| {
                WError::new("csl_relay_to_relay", "Failed to get single host addr")
            })?;
            Ok(Relay::SingleHostAddr(SingleHostAddr {
                ipv4: single_host_addr.ipv4().map(|ipv4| {
                    let octets = ipv4.ip();
                    let ipv4_bytes = <[u8; 4]>::try_from(&octets[..4]).unwrap_or([0, 0, 0, 0]);
                    Ipv4Addr::from(ipv4_bytes).to_string()
                }),
                ipv6: single_host_addr.ipv6().map(|ipv6| {
                    let octets = ipv6.ip();
                    let ipv6_bytes = <[u8; 16]>::try_from(&octets[..16]).unwrap_or([0; 16]);
                    Ipv6Addr::from(ipv6_bytes).to_string()
                }),
                port: single_host_addr.port(),
            }))
        }
        csl::RelayKind::SingleHostName => {
            let single_host_name = relay.as_single_host_name().ok_or_else(|| {
                WError::new("csl_relay_to_relay", "Failed to get single host name")
            })?;
            Ok(Relay::SingleHostName(SingleHostName {
                domain_name: single_host_name.dns_name().record(),
                port: single_host_name.port(),
            }))
        }
        csl::RelayKind::MultiHostName => {
            let multi_host_name = relay.as_multi_host_name().ok_or_else(|| {
                WError::new("csl_relay_to_relay", "Failed to get multi host name")
            })?;
            Ok(Relay::MultiHostName(MultiHostName {
                domain_name: multi_host_name.dns_name().record(),
            }))
        }
    }
}
