use std::collections::HashMap;

use pallas::ledger::primitives::conway::{
    LanguageView, Redeemer as PallasRedeemer, RedeemerTag as PallasRedeemerTag, ScriptData,
};
use pallas::ledger::primitives::Fragment;
use whisky_common::{get_cost_models_from_network, MintItem, Output, PubKeyTxIn, RefTxIn};
use whisky_common::{
    Certificate as WhiskyCertificate,
    Certificate::{BasicCertificate, ScriptCertificate, SimpleScriptCertificate},
    CertificateType,
    DatumSource::{self, InlineDatumSource, ProvidedDatumSource},
    LanguageVersion,
    ScriptSource::{self, InlineScriptSource, ProvidedScriptSource},
    SimpleScriptTxInParameter::{InlineSimpleScriptSource, ProvidedSimpleScriptSource},
    TxBuilderBody, TxIn, Vote as WhiskyVote, WError, Withdrawal as WhiskyWithdrawal,
    Withdrawal::{PlutusScriptWithdrawal, PubKeyWithdrawal, SimpleScriptWithdrawal},
};

use crate::{
    converter::{bytes_from_bech32, convert_value},
    wrapper::{
        transaction_body::{
            Anchor, Certificate, CertificateKind, DRep, DRepKind, Datum, DatumKind, GovActionId,
            MultiassetNonZeroInt, NetworkId, NetworkIdKind, PoolMetadata, Relay, RelayKind,
            RequiredSigners, RewardAccount, ScriptRef, ScriptRefKind, StakeCredential,
            StakeCredentialKind, Transaction, TransactionBody, TransactionInput, TransactionOutput,
            Value, Vote, VoteKind, Voter, VoterKind, VotingProdecedure,
        },
        witness_set::{
            native_script::NativeScript,
            plutus_data::PlutusData,
            plutus_script::PlutusScript,
            redeemer::{ExUnits, Redeemer, RedeemerTag},
            witness_set::WitnessSet,
        },
    },
};

#[derive(Clone, Debug)]
pub struct CorePallas {
    pub tx_evaluation_multiplier_percentage: u64,
    pub protocol_params: whisky_common::Protocol,

    // Required info for balancing transaction
    pub inputs_map: HashMap<TransactionInput, Value>,
    pub collaterals_map: HashMap<TransactionInput, Value>,
    pub script_source_ref_inputs: Vec<RefTxIn>,
    pub total_script_size: usize,

    // Required info for generating witness set
    pub native_scripts_vec: Vec<NativeScript>,
    pub plutus_v1_scripts_vec: Vec<PlutusScript<1>>,
    pub plutus_v2_scripts_vec: Vec<PlutusScript<2>>,
    pub plutus_v3_scripts_vec: Vec<PlutusScript<3>>,
    pub plutus_v1_used: bool,
    pub plutus_v2_used: bool,
    pub plutus_v3_used: bool,
    pub input_redeemers_vec: Vec<(TransactionInput, Redeemer)>,
    pub certificate_redeemers_vec: Vec<(Certificate, Redeemer)>,
    pub withdrawal_redeemers_vec: Vec<(RewardAccount, Redeemer)>,
    pub mint_redeemers_vec: Vec<(String, Redeemer)>,
    pub vote_redeemers_vec: Vec<(Voter, Redeemer)>,
    pub plutus_data_vec: Vec<PlutusData>,

    // Potential reference inputs (shouldn't overlap with actual inputs)
    pub ref_inputs_vec: Vec<TransactionInput>,
}

impl CorePallas {
    pub fn new(tx_evaluation_multiplier_percentage: u64) -> Self {
        Self {
            tx_evaluation_multiplier_percentage,
            protocol_params: whisky_common::Protocol::default(),
            inputs_map: HashMap::new(),
            collaterals_map: HashMap::new(),
            script_source_ref_inputs: vec![],
            total_script_size: 0,
            native_scripts_vec: vec![],
            plutus_v1_scripts_vec: vec![],
            plutus_v2_scripts_vec: vec![],
            plutus_v3_scripts_vec: vec![],
            plutus_v1_used: false,
            plutus_v2_used: false,
            plutus_v3_used: false,
            input_redeemers_vec: vec![],
            certificate_redeemers_vec: vec![],
            withdrawal_redeemers_vec: vec![],
            mint_redeemers_vec: vec![],
            vote_redeemers_vec: vec![],
            plutus_data_vec: vec![],
            ref_inputs_vec: vec![],
        }
    }

    fn process_inputs(
        &mut self,
        whisky_inputs: Vec<TxIn>,
    ) -> Result<Vec<TransactionInput>, WError> {
        let mut inputs: Vec<TransactionInput> = vec![];
        for tx_in in whisky_inputs.clone() {
            match tx_in {
                TxIn::PubKeyTxIn(pub_key_tx_in) => {
                    let input = TransactionInput::new(
                        &pub_key_tx_in.tx_in.tx_hash,
                        pub_key_tx_in.tx_in.tx_index.into(),
                    )?;
                    let asset_vec = pub_key_tx_in.tx_in.amount.clone().ok_or_else(|| {
                        WError::new("WhiskyPallas - Adding inputs:", "Input amount is missing")
                    })?;
                    let value = convert_value(&asset_vec)?;
                    self.inputs_map.insert(input.clone(), value);
                    inputs.push(input);
                }
                TxIn::SimpleScriptTxIn(simple_script_tx_in) => {
                    let input = TransactionInput::new(
                        &simple_script_tx_in.tx_in.tx_hash,
                        simple_script_tx_in.tx_in.tx_index.into(),
                    )?;
                    let asset_vec = simple_script_tx_in.tx_in.amount.clone().ok_or_else(|| {
                        WError::new("WhiskyPallas - Adding inputs:", "Input amount is missing")
                    })?;
                    let value = convert_value(&asset_vec)?;
                    self.inputs_map.insert(input.clone(), value);
                    inputs.push(input);

                    match &simple_script_tx_in.simple_script_tx_in {
                        ProvidedSimpleScriptSource(provided_simple_script_source) => {
                            self.native_scripts_vec.push(NativeScript::new_from_hex(
                                &provided_simple_script_source.script_cbor.clone(),
                            )?);
                        }
                        InlineSimpleScriptSource(inline_simple_script_source) => {
                            self.ref_inputs_vec.push(TransactionInput::new(
                                &inline_simple_script_source.ref_tx_in.tx_hash,
                                inline_simple_script_source.ref_tx_in.tx_index.into(),
                            )?)
                        }
                    }
                }
                TxIn::ScriptTxIn(script_tx_in) => {
                    let input = TransactionInput::new(
                        &script_tx_in.tx_in.tx_hash,
                        script_tx_in.tx_in.tx_index.into(),
                    )?;
                    let asset_vec = script_tx_in.tx_in.amount.clone().ok_or_else(|| {
                        WError::new("WhiskyPallas - Adding inputs:", "Input amount is missing")
                    })?;
                    let value = convert_value(&asset_vec)?;
                    self.inputs_map.insert(input.clone(), value);
                    inputs.push(input.clone());

                    let script_source = script_tx_in
                        .script_tx_in
                        .script_source
                        .clone()
                        .ok_or_else(|| {
                            WError::new(
                                "WhiskyPallas - Adding inputs",
                                "Script source is missing from script input",
                            )
                        })?;

                    let datum_source =
                        script_tx_in
                            .script_tx_in
                            .datum_source
                            .clone()
                            .ok_or_else(|| {
                                WError::new(
                                    "WhiskyPallas - Adding inputs",
                                    "Datum source is missing from script input",
                                )
                            })?;

                    let redeemer = script_tx_in.script_tx_in.redeemer.clone().ok_or_else(|| {
                        WError::new(
                            "WhiskyPallas - Adding inputs",
                            "Redeemer is missing from script input",
                        )
                    })?;

                    self.process_script_source(script_source)?;

                    self.process_datum_source(datum_source)?;

                    self.input_redeemers_vec.push((
                        input.clone(),
                        Redeemer::new(
                            RedeemerTag::Spend,
                            0,
                            PlutusData::new(redeemer.data)?,
                            ExUnits {
                                mem: redeemer.ex_units.mem,
                                steps: redeemer.ex_units.steps,
                            },
                        )?,
                    ));
                }
            }
        }
        inputs.sort_by(|a, b| {
            a.inner
                .transaction_id
                .cmp(&b.inner.transaction_id)
                .then(a.inner.index.cmp(&b.inner.index))
        });
        Ok(inputs)
    }

    fn process_outputs(
        &mut self,
        whisky_outputs: Vec<Output>,
    ) -> Result<Vec<TransactionOutput<'static>>, WError> {
        let mut outputs: Vec<TransactionOutput> = vec![];
        let whisky_outputs = whisky_outputs.clone();
        for output in &whisky_outputs {
            let datum: Option<Datum> = match &output.datum {
                Some(datum_source) => match datum_source {
                    whisky_common::Datum::Inline(datum_str) => Some(Datum::new(DatumKind::Data {
                        plutus_data_hex: datum_str.to_string(),
                    })?),
                    whisky_common::Datum::Hash(datum_str) => {
                        let datum = Datum::new(DatumKind::Data {
                            plutus_data_hex: datum_str.to_string(),
                        })?;

                        let datum_hash_str = datum.hash()?;
                        Some(Datum::new(DatumKind::Hash {
                            datum_hash: datum_hash_str,
                        })?)
                    }
                    whisky_common::Datum::Embedded(datum_str) => {
                        let datum = Datum::new(DatumKind::Data {
                            plutus_data_hex: datum_str.to_string(),
                        })?;
                        self.plutus_data_vec
                            .push(PlutusData::new(datum_str.to_string())?);

                        let datum_hash_str = datum.hash()?;
                        Some(Datum::new(DatumKind::Hash {
                            datum_hash: datum_hash_str,
                        })?)
                    }
                },
                None => None,
            };

            let script_ref = match &output.reference_script {
                Some(script_source) => match script_source {
                    whisky_common::OutputScriptSource::ProvidedScriptSource(
                        provided_script_source,
                    ) => {
                        let plutus_script = match provided_script_source.language_version {
                            LanguageVersion::V1 => ScriptRef::new(ScriptRefKind::PlutusV1Script {
                                plutus_v1_script_hex: provided_script_source.script_cbor.clone(),
                            })?,
                            LanguageVersion::V2 => ScriptRef::new(ScriptRefKind::PlutusV2Script {
                                plutus_v2_script_hex: provided_script_source.script_cbor.clone(),
                            })?,
                            LanguageVersion::V3 => ScriptRef::new(ScriptRefKind::PlutusV3Script {
                                plutus_v3_script_hex: provided_script_source.script_cbor.clone(),
                            })?,
                        };
                        Some(plutus_script)
                    }
                    whisky_common::OutputScriptSource::ProvidedSimpleScriptSource(
                        provided_simple_script_source,
                    ) => {
                        let native_script = ScriptRef::new(ScriptRefKind::NativeScript {
                            native_script_hex: provided_simple_script_source.script_cbor.clone(),
                        })?;
                        Some(native_script)
                    }
                },
                None => None,
            };
            outputs.push(TransactionOutput::new(
                &bytes_from_bech32(&output.address)?,
                convert_value(&output.amount.clone())?,
                datum,
                script_ref,
            )?);
        }
        Ok(outputs)
    }

    fn process_fee(&mut self, whisky_fee: Option<String>) -> Result<u64, WError> {
        whisky_fee
            .ok_or_else(|| {
                WError::new(
                    "WhiskyPallas - Adding fee:",
                    "Fee is missing from TxBuilderBody",
                )
            })?
            .parse::<u64>()
            .map_err(|e| {
                WError::new(
                    "WhiskyPallas - Adding fee:",
                    &format!("Failed to parse fee: {}", e.to_string()),
                )
            })
    }

    fn process_certificates(
        &mut self,
        whisky_certificates: Vec<WhiskyCertificate>,
    ) -> Result<Option<Vec<Certificate>>, WError> {
        let mut certificates: Vec<Certificate> = vec![];

        fn process_certificate_type(cert_type: &CertificateType) -> Result<Certificate, WError> {
            match cert_type {
                CertificateType::RegisterStake(register_stake) => {
                    Ok(Certificate::new(CertificateKind::StakeRegistration {
                        stake_credential: StakeCredential::from_bech32(
                            &register_stake.stake_key_address,
                        )?,
                    }))?
                }
                CertificateType::DeregisterStake(deregister_stake) => {
                    Ok(Certificate::new(CertificateKind::StakeDeregistration {
                        stake_credential: StakeCredential::from_bech32(
                            &deregister_stake.stake_key_address,
                        )?,
                    }))?
                }
                CertificateType::DelegateStake(delegate_stake) => {
                    Ok(Certificate::new(CertificateKind::StakeDelegation {
                        stake_credential: StakeCredential::from_bech32(
                            &delegate_stake.stake_key_address,
                        )?,
                        pool_key_hash: delegate_stake.pool_id.clone(),
                    }))?
                }
                CertificateType::RegisterPool(register_pool) => {
                    let mut relays: Vec<Relay> = vec![];
                    for relay in &register_pool.pool_params.relays {
                        match relay {
                            whisky_common::Relay::SingleHostAddr(single_host_addr) => {
                                relays.push(Relay::new(RelayKind::SingleHostAddr(
                                    single_host_addr.port.map(|p| p.into()),
                                    single_host_addr.ipv4.clone(),
                                    single_host_addr.ipv6.clone(),
                                ))?);
                            }
                            whisky_common::Relay::SingleHostName(single_host_name) => {
                                relays.push(Relay::new(RelayKind::SingleHostName(
                                    single_host_name.port.map(|p| p.into()),
                                    single_host_name.domain_name.clone(),
                                ))?);
                            }
                            whisky_common::Relay::MultiHostName(multi_host_name) => {
                                relays.push(Relay::new(RelayKind::MultiHostName(
                                    multi_host_name.domain_name.clone(),
                                ))?);
                            }
                        }
                    }
                    Ok(Certificate::new(CertificateKind::PoolRegistration {
                        operator: register_pool.pool_params.operator.clone(),
                        vrf_keyhash: register_pool.pool_params.vrf_key_hash.clone(),
                        pledge: register_pool
                            .pool_params
                            .pledge
                            .clone()
                            .parse::<u64>()
                            .map_err(|e| {
                                WError::new(
                                    "Certificate - Pool Registration: Invalid pledge amount",
                                    &e.to_string(),
                                )
                            })?,
                        cost: register_pool
                            .pool_params
                            .cost
                            .clone()
                            .parse::<u64>()
                            .map_err(|e| {
                                WError::new(
                                    "Certificate - Pool Registration: Invalid cost amount",
                                    &e.to_string(),
                                )
                            })?,
                        margin: register_pool.pool_params.margin,
                        reward_account: RewardAccount::new(bytes_from_bech32(
                            &register_pool.pool_params.reward_address,
                        )?)?,
                        pool_owners: register_pool.pool_params.owners.clone(),
                        relays,
                        pool_metadata: register_pool.pool_params.metadata.clone().map(|metadata| {
                            PoolMetadata::new(metadata.url, metadata.hash).unwrap()
                        }),
                    }))?
                }
                CertificateType::RetirePool(retire_pool) => {
                    Ok(Certificate::new(CertificateKind::PoolRetirement {
                        pool_key_hash: retire_pool.pool_id.clone(),
                        epoch: retire_pool.epoch.into(),
                    }))?
                }
                CertificateType::VoteDelegation(vote_delegation) => {
                    let drep: DRep = match &vote_delegation.drep {
                        whisky_common::DRep::DRepId(drep_id) => DRep::from_bech32(&drep_id)?,
                        whisky_common::DRep::AlwaysAbstain => DRep::new(DRepKind::Abstain)?,
                        whisky_common::DRep::AlwaysNoConfidence => {
                            DRep::new(DRepKind::NoConfidence)?
                        }
                    };
                    Ok(Certificate::new(CertificateKind::VoteDeleg {
                        stake_credential: StakeCredential::from_bech32(
                            &vote_delegation.stake_key_address,
                        )?,
                        drep,
                    }))?
                }
                CertificateType::StakeAndVoteDelegation(stake_and_vote_delegation) => {
                    let drep: DRep = match &stake_and_vote_delegation.drep {
                        whisky_common::DRep::DRepId(drep_id) => DRep::from_bech32(&drep_id)?,
                        whisky_common::DRep::AlwaysAbstain => DRep::new(DRepKind::Abstain)?,
                        whisky_common::DRep::AlwaysNoConfidence => {
                            DRep::new(DRepKind::NoConfidence)?
                        }
                    };
                    Ok(Certificate::new(CertificateKind::StakeVoteDeleg {
                        stake_credential: StakeCredential::from_bech32(
                            &stake_and_vote_delegation.stake_key_address,
                        )?,
                        pool_key_hash: stake_and_vote_delegation.pool_key_hash.clone(),
                        drep,
                    }))?
                }
                CertificateType::StakeRegistrationAndDelegation(
                    stake_registration_and_delegation,
                ) => Ok(Certificate::new(CertificateKind::StakeRegDeleg {
                    stake_credential: StakeCredential::from_bech32(
                        &stake_registration_and_delegation.stake_key_address,
                    )?,
                    pool_key_hash: stake_registration_and_delegation.pool_key_hash.clone(),
                    amount: stake_registration_and_delegation.coin.into(),
                }))?,
                CertificateType::VoteRegistrationAndDelegation(
                    vote_registration_and_delegation,
                ) => {
                    let drep: DRep = match &vote_registration_and_delegation.drep {
                        whisky_common::DRep::DRepId(drep_id) => DRep::from_bech32(&drep_id)?,
                        whisky_common::DRep::AlwaysAbstain => DRep::new(DRepKind::Abstain)?,
                        whisky_common::DRep::AlwaysNoConfidence => {
                            DRep::new(DRepKind::NoConfidence)?
                        }
                    };
                    Ok(Certificate::new(CertificateKind::VoteRegDeleg {
                        stake_credential: StakeCredential::from_bech32(
                            &vote_registration_and_delegation.stake_key_address,
                        )?,
                        drep,
                        amount: vote_registration_and_delegation.coin.into(),
                    }))?
                }
                CertificateType::StakeVoteRegistrationAndDelegation(
                    stake_vote_registration_and_delegation,
                ) => {
                    let drep: DRep = match &stake_vote_registration_and_delegation.drep {
                        whisky_common::DRep::DRepId(drep_id) => DRep::from_bech32(&drep_id)?,
                        whisky_common::DRep::AlwaysAbstain => DRep::new(DRepKind::Abstain)?,
                        whisky_common::DRep::AlwaysNoConfidence => {
                            DRep::new(DRepKind::NoConfidence)?
                        }
                    };
                    Ok(Certificate::new(CertificateKind::StakeVoteRegDeleg {
                        stake_credential: StakeCredential::from_bech32(
                            &stake_vote_registration_and_delegation.stake_key_address,
                        )?,
                        pool_key_hash: stake_vote_registration_and_delegation.pool_key_hash.clone(),
                        drep,
                        amount: stake_vote_registration_and_delegation.coin.into(),
                    }))?
                }
                CertificateType::CommitteeHotAuth(committee_hot_auth) => {
                    Ok(Certificate::new(CertificateKind::AuthCommitteeHot {
                        committee_cold_cred: StakeCredential::from_bech32(
                            &committee_hot_auth.committee_cold_key_address,
                        )?,
                        committee_hot_cred: StakeCredential::from_bech32(
                            &committee_hot_auth.committee_hot_key_address,
                        )?,
                    }))?
                }
                CertificateType::CommitteeColdResign(committee_cold_resign) => {
                    let anchor: Option<Anchor> = match &committee_cold_resign.anchor {
                        Some(anchor_data) => Some(Anchor::new(
                            anchor_data.anchor_url.clone(),
                            anchor_data.anchor_data_hash.clone(),
                        )?),
                        None => None,
                    };

                    Ok(Certificate::new(CertificateKind::ResignCommitteeCold {
                        committee_cold_cred: StakeCredential::from_bech32(
                            &committee_cold_resign.committee_cold_key_address,
                        )?,
                        anchor,
                    }))?
                }
                CertificateType::DRepRegistration(drep_registration) => {
                    let drep: DRep = DRep::from_bech32(&drep_registration.drep_id)?;
                    let drep_cred = match &drep.inner {
                        pallas::ledger::primitives::conway::DRep::Key(hash) => {
                            StakeCredential::new(StakeCredentialKind::KeyHash {
                                key_hash_hex: hash.to_string(),
                            })?
                        }
                        pallas::ledger::primitives::conway::DRep::Script(hash) => {
                            StakeCredential::new(StakeCredentialKind::ScriptHash {
                                script_hash_hex: hash.to_string(),
                            })?
                        }
                        _ => {
                            return Err(WError::new(
                                "Certificate - DRep Registration:",
                                "DRep must be either Key or Script type",
                            ));
                        }
                    };
                    Ok(Certificate::new(CertificateKind::RegDRepCert {
                        drep_cred,
                        amount: drep_registration.coin.into(),
                        anchor: drep_registration.anchor.clone().map(|anchor_data| {
                            Anchor::new(anchor_data.anchor_url, anchor_data.anchor_data_hash)
                                .unwrap()
                        }),
                    }))?
                }
                CertificateType::DRepDeregistration(drep_deregistration) => {
                    let drep: DRep = DRep::from_bech32(&drep_deregistration.drep_id)?;
                    let drep_cred = match &drep.inner {
                        pallas::ledger::primitives::conway::DRep::Key(hash) => {
                            StakeCredential::new(StakeCredentialKind::KeyHash {
                                key_hash_hex: hash.to_string(),
                            })?
                        }
                        pallas::ledger::primitives::conway::DRep::Script(hash) => {
                            StakeCredential::new(StakeCredentialKind::ScriptHash {
                                script_hash_hex: hash.to_string(),
                            })?
                        }
                        _ => {
                            return Err(WError::new(
                                "Certificate - DRep Deregistration:",
                                "DRep must be either Key or Script type",
                            ));
                        }
                    };
                    Ok(Certificate::new(CertificateKind::UnRegDRepCert {
                        drep_cred,
                        amount: drep_deregistration.coin.into(),
                    }))?
                }
                CertificateType::DRepUpdate(drep_update) => {
                    let drep: DRep = DRep::from_bech32(&drep_update.drep_id)?;
                    let drep_cred = match &drep.inner {
                        pallas::ledger::primitives::conway::DRep::Key(hash) => {
                            StakeCredential::new(StakeCredentialKind::KeyHash {
                                key_hash_hex: hash.to_string(),
                            })?
                        }
                        pallas::ledger::primitives::conway::DRep::Script(hash) => {
                            StakeCredential::new(StakeCredentialKind::ScriptHash {
                                script_hash_hex: hash.to_string(),
                            })?
                        }
                        _ => {
                            return Err(WError::new(
                                "Certificate - DRep Update:",
                                "DRep must be either Key or Script type",
                            ));
                        }
                    };

                    let anchor: Option<Anchor> = match &drep_update.anchor {
                        Some(anchor_data) => Some(Anchor::new(
                            anchor_data.anchor_url.clone(),
                            anchor_data.anchor_data_hash.clone(),
                        )?),
                        None => None,
                    };
                    Ok(Certificate::new(CertificateKind::UpdateDRepCert {
                        drep_cred,
                        anchor,
                    }))?
                }
            }
        }

        for cert in whisky_certificates.clone() {
            match cert {
                BasicCertificate(certificate_type) => {
                    certificates.push(process_certificate_type(&certificate_type)?);
                }
                ScriptCertificate(script_certificate) => {
                    let cert = process_certificate_type(&script_certificate.cert)?;
                    let script_source =
                        script_certificate.script_source.clone().ok_or_else(|| {
                            WError::new(
                                "WhiskyPallas - Processing certificates:",
                                "Script source is missing from script certificate",
                            )
                        })?;
                    self.process_script_source(script_source)?;

                    let redeemer = script_certificate.redeemer.clone().ok_or_else(|| {
                        WError::new(
                            "WhiskyPallas - Processing certificates:",
                            "Redeemer is missing from script certificate",
                        )
                    })?;
                    self.certificate_redeemers_vec.push((
                        cert.clone(),
                        Redeemer::new(
                            RedeemerTag::Cert,
                            0,
                            PlutusData::new(redeemer.data)?,
                            ExUnits {
                                mem: redeemer.ex_units.mem,
                                steps: redeemer.ex_units.steps,
                            },
                        )?,
                    ));

                    certificates.push(cert);
                }
                SimpleScriptCertificate(simple_script_certificate) => {
                    match simple_script_certificate.simple_script_source {
                        Some(simple_script_source) => match simple_script_source {
                            whisky_common::SimpleScriptSource::ProvidedSimpleScriptSource(
                                provided_simple_script_source,
                            ) => {
                                self.native_scripts_vec.push(NativeScript::new_from_hex(
                                    &provided_simple_script_source.script_cbor,
                                )?);
                            }
                            whisky_common::SimpleScriptSource::InlineSimpleScriptSource(
                                inline_simple_script_source,
                            ) => self.ref_inputs_vec.push(TransactionInput::new(
                                &inline_simple_script_source.ref_tx_in.tx_hash,
                                inline_simple_script_source.ref_tx_in.tx_index.into(),
                            )?),
                        },
                        None => {
                            return Err(WError::new(
                                "WhiskyPallas - Processing certificates:",
                                "Simple script source is missing from simple script certificate",
                            ));
                        }
                    };
                    certificates.push(process_certificate_type(&simple_script_certificate.cert)?);
                }
            }
        }
        if certificates.is_empty() {
            Ok(None)
        } else {
            Ok(Some(certificates))
        }
    }

    fn process_withdrawals(
        &mut self,
        whisky_withdrawals: Vec<WhiskyWithdrawal>,
    ) -> Result<Option<Vec<(RewardAccount, u64)>>, WError> {
        let mut withdrawals: Vec<(RewardAccount, u64)> = vec![];
        for withdrawal in whisky_withdrawals.clone() {
            match withdrawal {
                PubKeyWithdrawal(pub_key_withdrawal) => {
                    let reward_account_bytes = bytes_from_bech32(&pub_key_withdrawal.address)?;
                    let reward_account = RewardAccount::new(reward_account_bytes)?;
                    withdrawals.push((reward_account, pub_key_withdrawal.coin));
                }
                PlutusScriptWithdrawal(plutus_script_withdrawal) => {
                    let reward_account_bytes =
                        bytes_from_bech32(&plutus_script_withdrawal.address)?;
                    let reward_account = RewardAccount::new(reward_account_bytes)?;
                    withdrawals.push((reward_account.clone(), plutus_script_withdrawal.coin));
                    let script_source =
                        plutus_script_withdrawal
                            .script_source
                            .clone()
                            .ok_or_else(|| {
                                WError::new(
                                    "WhiskyPallas - Processing withdrawals:",
                                    "Script source is missing from plutus script withdrawal",
                                )
                            })?;
                    self.process_script_source(script_source)?;

                    let redeemer = plutus_script_withdrawal.redeemer.clone().ok_or_else(|| {
                        WError::new(
                            "WhiskyPallas - Processing withdrawals:",
                            "Redeemer is missing from plutus script withdrawal",
                        )
                    })?;
                    self.withdrawal_redeemers_vec.push((
                        reward_account.clone(),
                        Redeemer::new(
                            RedeemerTag::Reward,
                            0,
                            PlutusData::new(redeemer.data)?,
                            ExUnits {
                                mem: redeemer.ex_units.mem,
                                steps: redeemer.ex_units.steps,
                            },
                        )?,
                    ));
                }
                SimpleScriptWithdrawal(simple_script_withdrawal) => {
                    let reward_account_bytes =
                        bytes_from_bech32(&simple_script_withdrawal.address)?;
                    let reward_account = RewardAccount::new(reward_account_bytes)?;
                    withdrawals.push((reward_account.clone(), simple_script_withdrawal.coin));
                    match &simple_script_withdrawal.script_source {
                        Some(simple_script_source) => match simple_script_source {
                            whisky_common::SimpleScriptSource::ProvidedSimpleScriptSource(
                                provided_simple_script_source,
                            ) => {
                                self.native_scripts_vec.push(NativeScript::new_from_hex(
                                    &provided_simple_script_source.script_cbor,
                                )?);
                            }
                            whisky_common::SimpleScriptSource::InlineSimpleScriptSource(
                                inline_simple_script_source,
                            ) => self.ref_inputs_vec.push(TransactionInput::new(
                                &inline_simple_script_source.ref_tx_in.tx_hash,
                                inline_simple_script_source.ref_tx_in.tx_index.into(),
                            )?),
                        },
                        None => {
                            return Err(WError::new(
                                "WhiskyPallas - Processing withdrawals:",
                                "Simple script source is missing from simple script withdrawal",
                            ));
                        }
                    };
                }
            }
        }
        Ok(if withdrawals.is_empty() {
            None
        } else {
            Some(withdrawals)
        })
    }

    fn process_mints(
        &mut self,
        whisky_mints: Vec<MintItem>,
    ) -> Result<Option<MultiassetNonZeroInt>, WError> {
        let mut mints: Vec<(String, Vec<(String, i64)>)> = vec![];
        for mint in whisky_mints.clone() {
            match mint {
                whisky_common::MintItem::ScriptMint(script_mint) => {
                    let mint_param = script_mint.mint;
                    let existing_policy =
                        mints.iter_mut().find(|mint| mint.0 == mint_param.policy_id);
                    if existing_policy.is_some() {
                        let policy_mint = existing_policy.unwrap();
                        policy_mint.1.push((
                            mint_param.asset_name,
                            mint_param.amount.try_into().map_err(|_| {
                                WError::new(
                                    "WhiskyPallas - Processing mints:",
                                    "Invalid mint amount",
                                )
                            })?,
                        ));
                    } else {
                        mints.push((
                            mint_param.policy_id.clone(),
                            vec![(
                                mint_param.asset_name,
                                mint_param.amount.try_into().map_err(|_| {
                                    WError::new(
                                        "WhiskyPallas - Processing mints:",
                                        "Invalid mint amount",
                                    )
                                })?,
                            )],
                        ));
                    }

                    let script_source = script_mint.script_source.clone().ok_or_else(|| {
                        WError::new(
                            "WhiskyPallas - Processing mints:",
                            "Script source is missing from script mint",
                        )
                    })?;
                    self.process_script_source(script_source)?;

                    let redeemer = script_mint.redeemer.clone().ok_or_else(|| {
                        WError::new(
                            "WhiskyPallas - Processing mints:",
                            "Redeemer is missing from script mint",
                        )
                    })?;
                    self.mint_redeemers_vec.push((
                        mint_param.policy_id.clone(),
                        Redeemer::new(
                            RedeemerTag::Mint,
                            0,
                            PlutusData::new(redeemer.data)?,
                            ExUnits {
                                mem: redeemer.ex_units.mem,
                                steps: redeemer.ex_units.steps,
                            },
                        )?,
                    ));
                }
                whisky_common::MintItem::SimpleScriptMint(simple_script_mint) => {
                    let mint_param = simple_script_mint.mint;
                    let existing_policy =
                        mints.iter_mut().find(|mint| mint.0 == mint_param.policy_id);
                    if existing_policy.is_some() {
                        let policy_mint = existing_policy.unwrap();
                        policy_mint.1.push((
                            mint_param.asset_name,
                            mint_param.amount.try_into().map_err(|_| {
                                WError::new(
                                    "WhiskyPallas - Processing mints:",
                                    "Invalid mint amount",
                                )
                            })?,
                        ));
                    } else {
                        mints.push((
                            mint_param.policy_id.clone(),
                            vec![(
                                mint_param.asset_name,
                                mint_param.amount.try_into().map_err(|_| {
                                    WError::new(
                                        "WhiskyPallas - Processing mints:",
                                        "Invalid mint amount",
                                    )
                                })?,
                            )],
                        ));
                    }

                    match &simple_script_mint.script_source {
                        Some(simple_script_source) => match simple_script_source {
                            whisky_common::SimpleScriptSource::ProvidedSimpleScriptSource(
                                provided_simple_script_source,
                            ) => {
                                self.native_scripts_vec.push(NativeScript::new_from_hex(
                                    &provided_simple_script_source.script_cbor,
                                )?);
                            }
                            whisky_common::SimpleScriptSource::InlineSimpleScriptSource(
                                inline_simple_script_source,
                            ) => self.ref_inputs_vec.push(TransactionInput::new(
                                &inline_simple_script_source.ref_tx_in.tx_hash,
                                inline_simple_script_source.ref_tx_in.tx_index.into(),
                            )?),
                        },
                        None => {
                            return Err(WError::new(
                                "WhiskyPallas - Processing mints:",
                                "Simple script source is missing from simple script mint",
                            ));
                        }
                    };
                }
            }
        }
        Ok(if mints.is_empty() {
            None
        } else {
            Some(MultiassetNonZeroInt::new(mints)?)
        })
    }

    fn process_collaterals(
        &mut self,
        whisky_collaterals: Vec<PubKeyTxIn>,
    ) -> Result<Option<Vec<TransactionInput>>, WError> {
        let mut collaterals: Vec<TransactionInput> = vec![];
        for collateral in whisky_collaterals.clone() {
            let transaction_input =
                TransactionInput::new(&collateral.tx_in.tx_hash, collateral.tx_in.tx_index.into())?;
            collaterals.push(transaction_input.clone());
            self.collaterals_map.insert(
                transaction_input.clone(),
                convert_value(&collateral.tx_in.amount.unwrap())?,
            );
        }
        Ok(if collaterals.is_empty() {
            None
        } else {
            Some(collaterals)
        })
    }

    fn process_required_signers(
        &mut self,
        whisky_required_signers: Vec<String>,
    ) -> Result<Option<RequiredSigners>, WError> {
        let mut required_signers: Vec<String> = vec![];
        for signer in whisky_required_signers.clone() {
            required_signers.push(signer);
        }
        Ok(if required_signers.is_empty() {
            None
        } else {
            Some(RequiredSigners::new(required_signers)?)
        })
    }

    fn process_total_collateral(
        &mut self,
        whisky_total_collateral: Option<String>,
    ) -> Result<Option<u64>, WError> {
        if let Some(total_collateral) = whisky_total_collateral.clone() {
            Ok(Some(total_collateral.parse::<u64>().map_err(|e| {
                WError::new(
                    "WhiskyPallas - Processing total collateral:",
                    &format!("Failed to parse total collateral: {}", e.to_string()),
                )
            })?))
        } else {
            Ok(None)
        }
    }

    fn process_script_source(&mut self, script_source: ScriptSource) -> Result<(), WError> {
        match script_source {
            ProvidedScriptSource(provided_script_source) => {
                match provided_script_source.language_version {
                    LanguageVersion::V1 => {
                        self.plutus_v1_scripts_vec
                            .push(PlutusScript::<1>::new(provided_script_source.script_cbor)?);
                        self.plutus_v1_used = true;
                    }
                    LanguageVersion::V2 => {
                        self.plutus_v2_scripts_vec
                            .push(PlutusScript::<2>::new(provided_script_source.script_cbor)?);
                        self.plutus_v2_used = true;
                    }
                    LanguageVersion::V3 => {
                        self.plutus_v3_scripts_vec
                            .push(PlutusScript::<3>::new(provided_script_source.script_cbor)?);
                        self.plutus_v3_used = true;
                    }
                }
            }
            InlineScriptSource(inline_script_source) => {
                self.ref_inputs_vec.push(TransactionInput::new(
                    &inline_script_source.ref_tx_in.tx_hash,
                    inline_script_source.ref_tx_in.tx_index.into(),
                )?);
                self.script_source_ref_inputs
                    .push(inline_script_source.ref_tx_in.clone());
                match inline_script_source.language_version {
                    LanguageVersion::V1 => {
                        self.plutus_v1_used = true;
                    }
                    LanguageVersion::V2 => {
                        self.plutus_v2_used = true;
                    }
                    LanguageVersion::V3 => {
                        self.plutus_v3_used = true;
                    }
                }
            }
        };
        Ok(())
    }

    fn process_voting_procedures(
        &mut self,
        whisky_votes: Vec<WhiskyVote>,
    ) -> Result<Option<Vec<(Voter, Vec<(GovActionId, VotingProdecedure)>)>>, WError> {
        let mut voting_procedures: Vec<(Voter, Vec<(GovActionId, VotingProdecedure)>)> = vec![];

        fn process_vote_type(
            vote_type: &whisky_common::VoteType,
        ) -> Result<(Voter, Vec<(GovActionId, VotingProdecedure)>), WError> {
            let voter = match &vote_type.voter {
                whisky_common::Voter::ConstitutionalCommitteeHotCred(credential) => {
                    match credential {
                        whisky_common::Credential::KeyHash(key_hash_hex) => {
                            Voter::new(VoterKind::ConstitutionalCommitteKey {
                                key_hash: key_hash_hex.clone(),
                            })
                        }
                        whisky_common::Credential::ScriptHash(script_hash_hex) => {
                            Voter::new(VoterKind::ConstitutionalCommitteScript {
                                script_hash: script_hash_hex.clone(),
                            })
                        }
                    }
                }?,
                whisky_common::Voter::DRepId(drep_id) => {
                    let drep = DRep::from_bech32(&drep_id)?;
                    match drep.inner {
                        pallas::ledger::primitives::conway::DRep::Key(hash) => {
                            Voter::new(VoterKind::DrepKey {
                                key_hash: hash.to_string(),
                            })
                        }
                        pallas::ledger::primitives::conway::DRep::Script(hash) => {
                            Voter::new(VoterKind::DrepScript {
                                script_hash: hash.to_string(),
                            })
                        }
                        _ => {
                            return Err(WError::new(
                                "Voting Procedure - Voter:",
                                "DRep must be either Key or Script type",
                            ));
                        }
                    }
                }?,
                whisky_common::Voter::StakingPoolKeyHash(stake_credential) => {
                    let stake_cred = StakeCredential::from_bech32(&stake_credential)?;
                    match stake_cred.inner {
                        pallas::ledger::primitives::StakeCredential::ScriptHash(hash) => {
                            Voter::new(VoterKind::StakePoolKey {
                                pool_key_hash: hash.to_string(),
                            })
                        }
                        pallas::ledger::primitives::StakeCredential::AddrKeyhash(hash) => {
                            Voter::new(VoterKind::StakePoolKey {
                                pool_key_hash: hash.to_string(),
                            })
                        }
                    }
                }?,
            };

            let gov_action_id = GovActionId::new(
                &vote_type.gov_action_id.tx_hash,
                vote_type.gov_action_id.tx_index.into(),
            )?;

            let voting_procedure = VotingProdecedure::new(
                match &vote_type.voting_procedure.vote_kind {
                    whisky_common::VoteKind::No => Vote::new(VoteKind::No)?,
                    whisky_common::VoteKind::Yes => Vote::new(VoteKind::Yes)?,
                    whisky_common::VoteKind::Abstain => Vote::new(VoteKind::Abstain)?,
                },
                match &vote_type.voting_procedure.anchor {
                    Some(anchor_data) => Some(Anchor::new(
                        anchor_data.anchor_url.clone(),
                        anchor_data.anchor_data_hash.clone(),
                    )?),
                    None => None,
                },
            );
            Ok((voter, vec![(gov_action_id, voting_procedure)]))
        }

        for vote in whisky_votes.clone() {
            match vote {
                whisky_common::Vote::BasicVote(vote_type) => {
                    let (voter, procedures) = process_vote_type(&vote_type)?;
                    // Check if voter already exists in voting_procedures
                    if let Some(existing_voter) =
                        voting_procedures.iter_mut().find(|(v, _)| *v == voter)
                    {
                        existing_voter.1.extend(procedures);
                    } else {
                        voting_procedures.push((voter, procedures));
                    }
                }
                whisky_common::Vote::ScriptVote(script_vote) => {
                    let (voter, procedures) = process_vote_type(&script_vote.vote)?;
                    // Check if voter already exists in voting_procedures
                    if let Some(existing_voter) =
                        voting_procedures.iter_mut().find(|(v, _)| *v == voter)
                    {
                        existing_voter.1.extend(procedures);
                    } else {
                        voting_procedures.push((voter.clone(), procedures));
                    }

                    let script_source = script_vote.script_source.clone().ok_or_else(|| {
                        WError::new(
                            "WhiskyPallas - Processing voting procedures:",
                            "Script source is missing from script vote",
                        )
                    })?;
                    self.process_script_source(script_source)?;

                    let redeemer = script_vote.redeemer.clone().ok_or_else(|| {
                        WError::new(
                            "WhiskyPallas - Processing voting procedures:",
                            "Redeemer is missing from script vote",
                        )
                    })?;
                    self.vote_redeemers_vec.push((
                        voter.clone(),
                        Redeemer::new(
                            RedeemerTag::Vote,
                            0,
                            PlutusData::new(redeemer.data)?,
                            ExUnits {
                                mem: redeemer.ex_units.mem,
                                steps: redeemer.ex_units.steps,
                            },
                        )?,
                    ));
                }
                whisky_common::Vote::SimpleScriptVote(simple_script_vote) => {
                    let (voter, procedures) = process_vote_type(&simple_script_vote.vote)?;
                    // Check if voter already exists in voting_procedures
                    if let Some(existing_voter) =
                        voting_procedures.iter_mut().find(|(v, _)| *v == voter)
                    {
                        existing_voter.1.extend(procedures);
                    } else {
                        voting_procedures.push((voter, procedures));
                    }

                    match &simple_script_vote.simple_script_source {
                        Some(simple_script_source) => match simple_script_source {
                            whisky_common::SimpleScriptSource::ProvidedSimpleScriptSource(
                                provided_simple_script_source,
                            ) => {
                                self.native_scripts_vec.push(NativeScript::new_from_hex(
                                    &provided_simple_script_source.script_cbor,
                                )?);
                            }
                            whisky_common::SimpleScriptSource::InlineSimpleScriptSource(
                                inline_simple_script_source,
                            ) => self.ref_inputs_vec.push(TransactionInput::new(
                                &inline_simple_script_source.ref_tx_in.tx_hash,
                                inline_simple_script_source.ref_tx_in.tx_index.into(),
                            )?),
                        },
                        None => {
                            return Err(WError::new(
                                "WhiskyPallas - Processing voting procedures:",
                                "Simple script source is missing from simple script vote",
                            ));
                        }
                    };
                }
            }
        }

        if voting_procedures.is_empty() {
            Ok(None)
        } else {
            Ok(Some(voting_procedures))
        }
    }

    fn process_reference_inputs(
        &mut self,
        whisky_ref_inputs: Vec<RefTxIn>,
        whisky_inputs: Vec<TxIn>,
    ) -> Result<Option<Vec<TransactionInput>>, WError> {
        for ref_input in whisky_ref_inputs.clone() {
            self.ref_inputs_vec.push(TransactionInput::new(
                &ref_input.tx_hash,
                ref_input.tx_index.into(),
            )?);
        }
        let final_ref_inputs: Vec<TransactionInput> = self
            .ref_inputs_vec
            .clone()
            .iter()
            .filter(|ref_input| {
                // Check if the input exists in tx_builder_body
                whisky_inputs
                    .iter()
                    .find(|input| {
                        input.to_utxo().input.tx_hash == ref_input.inner.transaction_id.to_string()
                            && input.to_utxo().input.output_index == ref_input.inner.index as u32
                    })
                    .is_none()
            })
            .cloned()
            .collect();
        for pallas_ref_input in final_ref_inputs.iter() {
            let Some(script_source_tx_in) =
                self.script_source_ref_inputs
                    .iter()
                    .find(|script_source_tx_in| {
                        script_source_tx_in.tx_hash
                            == pallas_ref_input.inner.transaction_id.to_string()
                            && script_source_tx_in.tx_index == pallas_ref_input.inner.index as u32
                    })
            else {
                continue;
            };
            let Some(script_size) = script_source_tx_in.script_size else {
                continue;
            };
            self.total_script_size += script_size;
        }
        Ok(Some(final_ref_inputs))
    }

    fn process_datum_source(&mut self, datum_source: DatumSource) -> Result<(), WError> {
        match datum_source {
            ProvidedDatumSource(provided_datum_source) => {
                self.plutus_data_vec
                    .push(PlutusData::new(provided_datum_source.data)?);
            }
            InlineDatumSource(inline_datum_source) => {
                self.ref_inputs_vec.push(TransactionInput::new(
                    &inline_datum_source.tx_hash,
                    inline_datum_source.tx_index.into(),
                )?)
            }
        };
        Ok(())
    }

    fn process_witness_set(
        &'_ mut self,
        tx_inputs: Vec<TransactionInput>,
        certificates: Option<Vec<Certificate>>,
        withdrawals: Option<Vec<(RewardAccount, u64)>>,
        mints: Option<MultiassetNonZeroInt>,
        votes: Option<Vec<(Voter, Vec<(GovActionId, VotingProdecedure)>)>>,
    ) -> Result<WitnessSet<'_>, WError> {
        let native_scripts = if self.native_scripts_vec.is_empty() {
            None
        } else {
            Some(self.native_scripts_vec.clone())
        };
        let plutus_v1_scripts = if self.plutus_v1_scripts_vec.is_empty() {
            None
        } else {
            Some(self.plutus_v1_scripts_vec.clone())
        };
        let plutus_v2_scripts = if self.plutus_v2_scripts_vec.is_empty() {
            None
        } else {
            Some(self.plutus_v2_scripts_vec.clone())
        };
        let plutus_v3_scripts = if self.plutus_v3_scripts_vec.is_empty() {
            None
        } else {
            Some(self.plutus_v3_scripts_vec.clone())
        };
        let plutus_data = if self.plutus_data_vec.is_empty() {
            None
        } else {
            Some(self.plutus_data_vec.clone())
        };
        let mut redeemers: Vec<PallasRedeemer> = vec![];
        // Update redeemer indexes for input redeemers
        for (input, redeemer) in self.input_redeemers_vec.clone() {
            // Find the index of the input in the transaction inputs
            let index = tx_inputs.iter().position(|tx_input| {
                tx_input.inner.transaction_id.to_string() == input.inner.transaction_id.to_string()
                    && tx_input.inner.index == input.inner.index
            });
            if let Some(idx) = index {
                redeemers.push(PallasRedeemer {
                    tag: PallasRedeemerTag::Spend,
                    index: idx as u32,
                    data: redeemer.inner.data.clone(),
                    ex_units: redeemer.inner.ex_units.clone(),
                })
            } else {
                return Err(WError::new(
                    "WhiskyPallas - Processing witness set:",
                    "Input for redeemer not found in transaction inputs",
                ));
            }
        }
        // Update redeemer indexes for certificate redeemers
        let certificates = certificates.unwrap_or_default();
        for (cert, redeemer) in self.certificate_redeemers_vec.clone() {
            // Find the index of the certificate in the transaction body certificates
            let index = certificates.iter().position(|c| c == &cert);
            if let Some(idx) = index {
                redeemers.push(PallasRedeemer {
                    tag: PallasRedeemerTag::Cert,
                    index: idx as u32,
                    data: redeemer.inner.data.clone(),
                    ex_units: redeemer.inner.ex_units.clone(),
                })
            } else {
                return Err(WError::new(
                    "WhiskyPallas - Processing witness set:",
                    "Certificate for redeemer not found in transaction certificates",
                ));
            }
        }
        let withdrawals = withdrawals.unwrap_or_default();
        // Update redeemer indexes for withdrawal redeemers
        for (reward_account, redeemer) in self.withdrawal_redeemers_vec.clone() {
            // Find the index of the withdrawal in the transaction body withdrawals
            let index = withdrawals.iter().position(|w| w.0 == reward_account);
            if let Some(idx) = index {
                redeemers.push(PallasRedeemer {
                    tag: PallasRedeemerTag::Reward,
                    index: idx as u32,
                    data: redeemer.inner.data.clone(),
                    ex_units: redeemer.inner.ex_units.clone(),
                })
            } else {
                return Err(WError::new(
                    "WhiskyPallas - Processing witness set:",
                    "Withdrawal for redeemer not found in transaction withdrawals",
                ));
            }
        }
        let mints = mints.unwrap_or(MultiassetNonZeroInt::new(vec![])?);
        // Update redeemer indexes for mint redeemers
        for (policy_id, redeemer) in self.mint_redeemers_vec.clone() {
            // Find the index of the mint in the transaction body mints
            let index = mints
                .inner
                .keys()
                .position(|pid| pid.to_string() == policy_id);

            if let Some(idx) = index {
                redeemers.push(PallasRedeemer {
                    tag: PallasRedeemerTag::Mint,
                    index: idx as u32,
                    data: redeemer.inner.data.clone(),
                    ex_units: redeemer.inner.ex_units.clone(),
                })
            } else {
                return Err(WError::new(
                    "WhiskyPallas - Processing witness set:",
                    "Mint for redeemer not found in transaction mints",
                ));
            }
        }

        // Update redeemer indexes for vote redeemers
        let votes = votes.unwrap_or_default();
        for (voter, redeemer) in self.vote_redeemers_vec.clone() {
            // Find the index of the vote in the transaction body votes
            let index = votes.iter().position(|(v, _)| *v == voter);
            if let Some(idx) = index {
                redeemers.push(PallasRedeemer {
                    tag: PallasRedeemerTag::Vote,
                    index: idx as u32,
                    data: redeemer.inner.data.clone(),
                    ex_units: redeemer.inner.ex_units.clone(),
                })
            } else {
                return Err(WError::new(
                    "WhiskyPallas - Processing witness set:",
                    "Vote for redeemer not found in transaction votes",
                ));
            }
        }

        WitnessSet::new(
            None,
            native_scripts,
            None,
            plutus_v1_scripts,
            plutus_data,
            if redeemers.is_empty() {
                None
            } else {
                Some(
                    redeemers
                        .iter()
                        .map(|redeemer| Redeemer {
                            inner: redeemer.clone(),
                        })
                        .collect(),
                )
            },
            plutus_v2_scripts,
            plutus_v3_scripts,
        )
    }

    pub fn build_tx(&mut self, tx_builder_body: TxBuilderBody) -> Result<String, WError> {
        let inputs = self.process_inputs(tx_builder_body.inputs.clone())?;
        let outputs = self.process_outputs(tx_builder_body.outputs)?;
        let fee = self.process_fee(tx_builder_body.fee)?;
        let ttl = tx_builder_body.validity_range.invalid_hereafter;
        let certificates = self.process_certificates(tx_builder_body.certificates)?;
        let withdrawals = self.process_withdrawals(tx_builder_body.withdrawals)?;
        let validity_interval_start = tx_builder_body.validity_range.invalid_before;
        let mints = self.process_mints(tx_builder_body.mints)?;
        let collaterals = self.process_collaterals(tx_builder_body.collaterals)?;
        let required_signers =
            self.process_required_signers(tx_builder_body.required_signatures)?;
        let network = tx_builder_body.network.clone().unwrap();
        let network_id = match tx_builder_body.network.clone() {
            Some(network) => match network {
                whisky_common::Network::Mainnet => Some(NetworkId::new(NetworkIdKind::Mainnet)),
                _ => Some(NetworkId::new(NetworkIdKind::Testnet)),
            },
            None => None,
        };
        let total_collateral = self.process_total_collateral(tx_builder_body.total_collateral)?;
        let reference_inputs = self
            .process_reference_inputs(tx_builder_body.reference_inputs, tx_builder_body.inputs)?;
        let voting_procedures = self.process_voting_procedures(tx_builder_body.votes)?;
        let cost_models = get_cost_models_from_network(&network);
        let plutus_version: Option<u8> = if self.plutus_v3_used {
            Some(2)
        } else if self.plutus_v2_used {
            Some(1)
        } else if self.plutus_v1_used {
            Some(0)
        } else {
            None
        };
        let witness_set = self.process_witness_set(
            inputs.clone(),
            certificates.clone(),
            withdrawals.clone(),
            mints.clone(),
            voting_procedures.clone(),
        )?;
        let script_data_hash = match plutus_version {
            Some(version) => {
                let cost_model = match version {
                    0 => cost_models.get(0),
                    1 => cost_models.get(1),
                    2 => cost_models.get(2),
                    _ => None,
                };
                let language_view = cost_model.map(|cm| LanguageView(version, cm.clone()));
                Some(
                    ScriptData::build_for(&witness_set.inner, &language_view)
                        .unwrap()
                        .hash()
                        .to_string(),
                )
            }
            None => None,
        };

        let tx_body = TransactionBody::new(
            inputs,
            outputs,
            fee,
            ttl,
            certificates,
            withdrawals,
            None,
            validity_interval_start,
            mints,
            script_data_hash,
            collaterals,
            required_signers,
            network_id,
            None,
            total_collateral,
            reference_inputs,
            voting_procedures,
            None, // Proposals are currently not supported
            None, // Treasury donations are currently not supported
            None, // Treasury donations are currently not supported
        )?;
        let transaction_bytes = Transaction::new(tx_body, witness_set, true, None)?
            .inner
            .encode_fragment()
            .map_err(|e| {
                WError::new(
                    "WhiskyPallas - Building transaction:",
                    &format!("Encoding failed at Transaction: {}", e.to_string()),
                )
            })?;
        Ok(hex::encode(transaction_bytes))
    }
}
