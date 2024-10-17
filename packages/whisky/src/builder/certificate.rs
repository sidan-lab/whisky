use sidan_csl_rs::model::*;

use super::{TxBuilder, WRedeemer};

impl TxBuilder {
    /// ## Transaction building method
    ///
    /// Add a pool registration certificate to the TxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `pool_params` - Parameters of pool to be registered
    ///
    /// ### Returns
    ///
    /// * `Self` - The TxBuilder instance
    pub fn register_pool_certificate(&mut self, pool_params: &PoolParams) -> &mut Self {
        self.core
            .tx_builder_body
            .certificates
            .push(Certificate::BasicCertificate(
                CertificateType::RegisterPool(RegisterPool {
                    pool_params: pool_params.clone(),
                }),
            ));
        self
    }

    /// ## Transaction building method
    ///
    /// Add a stake registration certificate to the TxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `stake_key_address` - Address of the stake key
    ///
    /// ### Returns
    ///
    /// * `Self` - The TxBuilder instance
    pub fn register_stake_certificate(&mut self, stake_key_address: &str) -> &mut Self {
        self.core
            .tx_builder_body
            .certificates
            .push(Certificate::BasicCertificate(
                CertificateType::RegisterStake(RegisterStake {
                    stake_key_address: stake_key_address.to_string(),
                    coin: Protocol::default().key_deposit,
                }),
            ));
        self
    }

    /// ## Transaction building method
    ///
    /// Add a stake delegation certificate to the TxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `stake_key_address` - Address of the stake key
    /// * `pool_id` - id of the pool that will be delegated to
    ///
    /// ### Returns
    ///
    /// * `Self` - The TxBuilder instance
    pub fn delegate_stake_certificate(
        &mut self,
        stake_key_address: &str,
        pool_id: &str,
    ) -> &mut Self {
        self.core
            .tx_builder_body
            .certificates
            .push(Certificate::BasicCertificate(
                CertificateType::DelegateStake(DelegateStake {
                    stake_key_address: stake_key_address.to_string(),
                    pool_id: pool_id.to_string(),
                }),
            ));
        self
    }

    /// ## Transaction building method
    ///
    /// Add a stake deregistration certificate to the TxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `stake_key_address` - Address of the stake key
    ///
    /// ### Returns
    ///
    /// * `Self` - The TxBuilder instance
    pub fn deregister_stake_certificate(&mut self, stake_key_address: &str) -> &mut Self {
        self.core
            .tx_builder_body
            .certificates
            .push(Certificate::BasicCertificate(
                CertificateType::DeregisterStake(DeregisterStake {
                    stake_key_address: stake_key_address.to_string(),
                }),
            ));
        self
    }

    /// ## Transaction building method
    ///
    /// Add a pool retire certificate to the TxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `pool_id` - id of the pool that will be retired
    /// * `epoch` - The epoch that the pool will be retired from
    ///
    /// ### Returns
    ///
    /// * `Self` - The TxBuilder instance
    pub fn retire_pool_certificate(&mut self, pool_id: &str, epoch: u32) -> &mut Self {
        self.core
            .tx_builder_body
            .certificates
            .push(Certificate::BasicCertificate(CertificateType::RetirePool(
                RetirePool {
                    pool_id: pool_id.to_string(),
                    epoch,
                },
            )));
        self
    }

    /// ## Transaction building method
    ///
    /// Add a vote delegation certificate to the TxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `stake_key_address` - Address of the stake key
    /// * `drep` - The drep that will be voted for, or always abstain / always no confidence
    ///
    /// ### Returns
    ///
    /// * `Self` - The TxBuilder instance
    pub fn vote_delegation_certificate(
        &mut self,
        stake_key_address: &str,
        drep: &DRep,
    ) -> &mut Self {
        self.core
            .tx_builder_body
            .certificates
            .push(Certificate::BasicCertificate(
                CertificateType::VoteDelegation(VoteDelegation {
                    stake_key_address: stake_key_address.to_string(),
                    drep: drep.clone(),
                }),
            ));
        self
    }

    /// ## Transaction building method
    ///
    /// Add a stake and vote delegation certificate to the TxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `stake_key_address` - Address of the stake key
    /// * `pool_key_hash` - Hash of pool key that will be delegated to, same as pool id
    /// * `drep` - The drep that will be voted for, or always abstain / always no confidence
    ///
    /// ### Returns
    ///
    /// * `Self` - The TxBuilder instance
    pub fn stake_and_vote_delegation_certificate(
        &mut self,
        stake_key_address: &str,
        pool_key_hash: &str,
        drep: &DRep,
    ) -> &mut Self {
        self.core
            .tx_builder_body
            .certificates
            .push(Certificate::BasicCertificate(
                CertificateType::StakeAndVoteDelegation(StakeAndVoteDelegation {
                    stake_key_address: stake_key_address.to_string(),
                    pool_key_hash: pool_key_hash.to_string(),
                    drep: drep.clone(),
                }),
            ));
        self
    }

    /// ## Transaction building method
    ///
    /// Add a stake registration and delegation certificate to the TxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `stake_key_address` - Address of the stake key
    /// * `pool_key_hash` - Hash of pool key that will be delegated to, same as pool id
    /// * `coin` - Deposit for certificate registration
    ///
    /// ### Returns
    ///
    /// * `Self` - The TxBuilder instance
    pub fn stake_registration_and_delegation(
        &mut self,
        stake_key_address: &str,
        pool_key_hash: &str,
        coin: u64,
    ) -> &mut Self {
        self.core
            .tx_builder_body
            .certificates
            .push(Certificate::BasicCertificate(
                CertificateType::StakeRegistrationAndDelegation(StakeRegistrationAndDelegation {
                    stake_key_address: stake_key_address.to_string(),
                    pool_key_hash: pool_key_hash.to_string(),
                    coin,
                }),
            ));
        self
    }

    /// ## Transaction building method
    ///
    /// Add a vote registration and delegation certificate to the TxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `stake_key_address` - Address of the stake key
    /// * `drep` - The drep that will be voted for, or always abstain / always no confidence
    /// * `coin` - Deposit for certificate registration
    ///
    /// ### Returns
    ///
    /// * `Self` - The TxBuilder instance
    pub fn vote_registration_and_delegation(
        &mut self,
        stake_key_address: &str,
        drep: &DRep,
        coin: u64,
    ) -> &mut Self {
        self.core
            .tx_builder_body
            .certificates
            .push(Certificate::BasicCertificate(
                CertificateType::VoteRegistrationAndDelegation(VoteRegistrationAndDelegation {
                    stake_key_address: stake_key_address.to_string(),
                    drep: drep.clone(),
                    coin,
                }),
            ));
        self
    }

    /// ## Transaction building method
    ///
    /// Add a stake vote registration and delegation certificate to the TxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `stake_key_address` - Address of the stake key
    /// * `pool_key_hash` - Hash of pool key that will be delegated to, same as pool id
    /// * `drep` - The drep that will be voted for, or always abstain / always no confidence
    /// * `coin` - Deposit for certificate registration
    ///
    /// ### Returns
    ///
    /// * `Self` - The TxBuilder instance
    pub fn stake_vote_registration_and_delegation(
        &mut self,
        stake_key_address: &str,
        pool_key_hash: &str,
        drep: &DRep,
        coin: u64,
    ) -> &mut Self {
        self.core
            .tx_builder_body
            .certificates
            .push(Certificate::BasicCertificate(
                CertificateType::StakeVoteRegistrationAndDelegation(
                    StakeVoteRegistrationAndDelegation {
                        stake_key_address: stake_key_address.to_string(),
                        pool_key_hash: pool_key_hash.to_string(),
                        drep: drep.clone(),
                        coin,
                    },
                ),
            ));
        self
    }

    /// ## Transaction building method
    ///
    /// Add commitee hot auth certificate to the TxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `committee_cold_key_address` - Address of the committee cold key
    /// * `committee_hot_key_address` - Address of the commitee hot key
    ///
    /// ### Returns
    ///
    /// * `Self` - The TxBuilder instance
    pub fn committee_hot_auth(
        &mut self,
        committee_cold_key_address: &str,
        committee_hot_key_address: &str,
    ) -> &mut Self {
        self.core
            .tx_builder_body
            .certificates
            .push(Certificate::BasicCertificate(
                CertificateType::CommitteeHotAuth(CommitteeHotAuth {
                    committee_cold_key_address: committee_cold_key_address.to_string(),
                    committee_hot_key_address: committee_hot_key_address.to_string(),
                }),
            ));
        self
    }

    /// ## Transaction building method
    ///
    /// Add commitee cold resign certificate to the TxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `committee_cold_key_address` - Address of the committee cold key
    /// * `anchor` - The Anchor, this is a URL and a hash of the doc at this URL
    ///
    /// ### Returns
    ///
    /// * `Self` - The TxBuilder instance
    pub fn commitee_cold_resign(
        &mut self,
        committee_cold_key_address: &str,
        anchor: Option<Anchor>,
    ) -> &mut Self {
        self.core
            .tx_builder_body
            .certificates
            .push(Certificate::BasicCertificate(
                CertificateType::CommitteeColdResign(CommitteeColdResign {
                    committee_cold_key_address: committee_cold_key_address.to_string(),
                    anchor,
                }),
            ));
        self
    }

    /// ## Transaction building method
    ///
    /// Add DRep registration certificate to the TxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `voting_key_address` - Address of the voting key
    /// * `coin` - Deposit for certificate registration
    /// * `anchor` - The Anchor, this is a URL and a hash of the doc at this URL
    ///
    /// ### Returns
    ///
    /// * `Self` - The TxBuilder instance
    pub fn drep_registration(
        &mut self,
        drep_id: &str,
        coin: u64,
        anchor: Option<Anchor>,
    ) -> &mut Self {
        self.core
            .tx_builder_body
            .certificates
            .push(Certificate::BasicCertificate(
                CertificateType::DRepRegistration(DRepRegistration {
                    drep_id: drep_id.to_string(),
                    coin,
                    anchor,
                }),
            ));
        self
    }

    /// ## Transaction building method
    ///
    /// Add DRep deregistration certificate to the TxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `voting_key_address` - Address of the voting key
    /// * `coin` - Deposit for certificate registration
    ///
    /// ### Returns
    ///
    /// * `Self` - The TxBuilder instance
    pub fn drep_deregistration(&mut self, drep_id: &str, coin: u64) -> &mut Self {
        self.core
            .tx_builder_body
            .certificates
            .push(Certificate::BasicCertificate(
                CertificateType::DRepDeregistration(DRepDeregistration {
                    drep_id: drep_id.to_string(),
                    coin,
                }),
            ));
        self
    }

    /// ## Transaction building method
    ///
    /// Add DRep update certificate to the TxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `voting_key_address` - Address of the voting key
    /// * `anchor` - The Anchor, this is a URL and a hash of the doc at this URL
    ///
    /// ### Returns
    ///
    /// * `Self` - The TxBuilder instance
    pub fn drep_update(&mut self, drep_id: &str, anchor: Option<Anchor>) -> &mut Self {
        self.core
            .tx_builder_body
            .certificates
            .push(Certificate::BasicCertificate(CertificateType::DRepUpdate(
                DRepUpdate {
                    drep_id: drep_id.to_string(),
                    anchor,
                },
            )));
        self
    }

    /// ## Transaction building method
    ///
    /// Add script witness to certificate
    ///
    /// ### Arguments
    ///
    /// * `script_cbor` - The script in CBOR format
    /// * `version` - The language version, if the language version is None, the script is assumed to be a Native Script
    ///
    /// ### Returns
    ///
    /// * `Self` - The TxBuilder instance
    pub fn certificate_script(
        &mut self,
        script_cbor: &str,
        version: Option<LanguageVersion>,
    ) -> &mut Self {
        let last_cert = self.core.tx_builder_body.certificates.pop();
        if last_cert.is_none() {
            panic!("Undefined certificate");
        }
        let last_cert = last_cert.unwrap();
        match last_cert {
            Certificate::BasicCertificate(basic_cert) => match version {
                Some(lang_ver) => self.core.tx_builder_body.certificates.push(
                    Certificate::ScriptCertificate(ScriptCertificate {
                        cert: basic_cert,
                        redeemer: None,
                        script_source: Some(ScriptSource::ProvidedScriptSource(
                            ProvidedScriptSource {
                                script_cbor: script_cbor.to_string(),
                                language_version: lang_ver,
                            },
                        )),
                    }),
                ),
                None => self.core.tx_builder_body.certificates.push(
                    Certificate::SimpleScriptCertificate(SimpleScriptCertificate {
                        cert: basic_cert,
                        simple_script_source: Some(SimpleScriptSource::ProvidedSimpleScriptSource(
                            ProvidedSimpleScriptSource {
                                script_cbor: script_cbor.to_string(),
                            },
                        )),
                    }),
                ),
            },
            Certificate::ScriptCertificate(script_cert) => match version {
                Some(lang_ver) => self.core.tx_builder_body.certificates.push(
                    Certificate::ScriptCertificate(ScriptCertificate {
                        cert: script_cert.cert,
                        redeemer: script_cert.redeemer,
                        script_source: Some(ScriptSource::ProvidedScriptSource(
                            ProvidedScriptSource {
                                script_cbor: script_cbor.to_string(),
                                language_version: lang_ver,
                            },
                        )),
                    }),
                ),
                None => panic!("Language version has to be defined for plutus certificates"),
            },
            Certificate::SimpleScriptCertificate(_) => {
                panic!("Native script cert had its script defined twice")
            }
        }

        self
    }

    /// ## Transaction building method
    ///
    /// Add a Certificate transaction input reference to the TxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `tx_hash` - The transaction hash
    /// * `tx_index` - The transaction index
    /// * `script_hash` - The script hash
    /// * `version` - The language version, if the language version is None, the script is assumed to be a Native Script
    /// * `script_size` - Size of the script
    ///
    /// ### Returns
    ///
    /// * `Self` - The TxBuilder instance
    pub fn certificate_tx_in_reference(
        &mut self,
        tx_hash: &str,
        tx_index: u32,
        script_hash: &str,
        version: Option<LanguageVersion>,
        script_size: usize,
    ) -> &mut Self {
        let last_cert = self.core.tx_builder_body.certificates.pop();
        if last_cert.is_none() {
            panic!("Undefined certificate");
        }
        let last_cert = last_cert.unwrap();
        match last_cert {
            Certificate::BasicCertificate(basic_cert) => match version {
                Some(lang_ver) => self.core.tx_builder_body.certificates.push(
                    Certificate::ScriptCertificate(ScriptCertificate {
                        cert: basic_cert,
                        redeemer: None,
                        script_source: Some(ScriptSource::InlineScriptSource(InlineScriptSource {
                            ref_tx_in: RefTxIn {
                                tx_hash: tx_hash.to_string(),
                                tx_index,
                            },
                            script_hash: script_hash.to_string(),
                            language_version: lang_ver,
                            script_size,
                        })),
                    }),
                ),
                None => self.core.tx_builder_body.certificates.push(
                    Certificate::SimpleScriptCertificate(SimpleScriptCertificate {
                        cert: basic_cert,
                        simple_script_source: Some(SimpleScriptSource::InlineSimpleScriptSource(
                            InlineSimpleScriptSource {
                                ref_tx_in: RefTxIn {
                                    tx_hash: tx_hash.to_string(),
                                    tx_index,
                                },
                                simple_script_hash: script_hash.to_string(),
                                script_size,
                            },
                        )),
                    }),
                ),
            },
            Certificate::ScriptCertificate(script_cert) => match version {
                Some(lang_ver) => self.core.tx_builder_body.certificates.push(
                    Certificate::ScriptCertificate(ScriptCertificate {
                        cert: script_cert.cert,
                        redeemer: script_cert.redeemer,
                        script_source: Some(ScriptSource::InlineScriptSource(InlineScriptSource {
                            ref_tx_in: RefTxIn {
                                tx_hash: tx_hash.to_string(),
                                tx_index,
                            },
                            script_hash: script_hash.to_string(),
                            language_version: lang_ver,
                            script_size,
                        })),
                    }),
                ),
                None => panic!("Language version has to be defined for plutus certificates"),
            },
            Certificate::SimpleScriptCertificate(_) => {
                panic!("Native script cert had its script defined twice")
            }
        }

        self
    }

    /// ## Transaction building method
    ///
    /// Add a Certificate Redeemer to the TxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `redeemer` - The redeemer value
    ///
    /// ### Returns
    ///
    /// * `Self` - The TxBuilder instance
    pub fn certificate_redeemer_value(&mut self, redeemer: &WRedeemer) -> &mut Self {
        let last_cert = self.core.tx_builder_body.certificates.pop();
        if last_cert.is_none() {
            panic!("Undefined certificate");
        }
        let last_cert = last_cert.unwrap();
        let current_redeemer = match redeemer.data.to_cbor() {
            Ok(raw_redeemer) => Some(Redeemer {
                data: raw_redeemer,
                ex_units: redeemer.clone().ex_units,
            }),
            Err(_) => {
                panic!("Error converting certificate redeemer to CBOR")
            }
        };
        match last_cert {
            Certificate::BasicCertificate(basic_cert) => self
                .core
                .tx_builder_body
                .certificates
                .push(Certificate::ScriptCertificate(ScriptCertificate {
                    cert: basic_cert,
                    redeemer: current_redeemer,
                    script_source: None,
                })),

            Certificate::ScriptCertificate(script_cert) => self
                .core
                .tx_builder_body
                .certificates
                .push(Certificate::ScriptCertificate(ScriptCertificate {
                    cert: script_cert.cert,
                    redeemer: current_redeemer,
                    script_source: script_cert.script_source,
                })),

            Certificate::SimpleScriptCertificate(_) => {
                panic!("Native script cert cannot use redeemers")
            }
        }

        self
    }
}
