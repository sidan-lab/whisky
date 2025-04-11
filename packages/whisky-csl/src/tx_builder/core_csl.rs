use crate::*;
use cardano_serialization_lib as csl;
use whisky_common::*;

#[derive(Clone, Debug)]
pub struct CoreCSL {
    pub tx_hex: String,
    pub tx_builder: csl::TransactionBuilder,
    pub tx_inputs_builder: csl::TxInputsBuilder,
    pub collateral_builder: csl::TxInputsBuilder,
    pub mint_builder: csl::MintBuilder,
    pub certificates_builder: csl::CertificatesBuilder,
    pub vote_builder: csl::VotingBuilder,
    pub tx_withdrawals_builder: csl::WithdrawalsBuilder,
    pub protocol_params: Protocol,
}

impl CoreCSL {
    pub fn new(params: Option<Protocol>) -> Result<CoreCSL, WError> {
        Ok(CoreCSL {
            tx_hex: String::new(),
            tx_builder: build_tx_builder(params.clone())
                .map_err(WError::add_err_trace("CoreCSL - new - build_tx_builder"))?,
            tx_inputs_builder: csl::TxInputsBuilder::new(),
            collateral_builder: csl::TxInputsBuilder::new(),
            mint_builder: csl::MintBuilder::new(),
            certificates_builder: csl::CertificatesBuilder::new(),
            vote_builder: csl::VotingBuilder::new(),
            tx_withdrawals_builder: csl::WithdrawalsBuilder::new(),
            protocol_params: params.unwrap_or_default(),
        })
    }

    pub fn reset_after_build(&mut self) {
        self.tx_builder = build_tx_builder(Some(self.protocol_params.clone())).unwrap();
        self.tx_inputs_builder = csl::TxInputsBuilder::new();
        self.collateral_builder = csl::TxInputsBuilder::new();
        self.mint_builder = csl::MintBuilder::new();
        self.certificates_builder = csl::CertificatesBuilder::new();
        self.vote_builder = csl::VotingBuilder::new();
        self.tx_withdrawals_builder = csl::WithdrawalsBuilder::new();
    }
}

impl CoreCSL {
    pub fn add_tx_in(&mut self, input: PubKeyTxIn) -> Result<(), WError> {
        self.tx_inputs_builder
            .add_regular_input(
                &csl::Address::from_bech32(&input.tx_in.address.unwrap())
                    .map_err(WError::from_err("CoreCSL - add_tx_in - invalid address"))?,
                &csl::TransactionInput::new(
                    &csl::TransactionHash::from_hex(&input.tx_in.tx_hash)
                        .map_err(WError::from_err("CoreCSL - add_tx_in - invalid tx_hash"))?,
                    input.tx_in.tx_index,
                ),
                &to_value(&input.tx_in.amount.unwrap())?,
            )
            .map_err(WError::from_err(
                "CoreCSL - add_tx_in - invalid regular input",
            ))?;
        Ok(())
    }

    pub fn add_simple_script_tx_in(&mut self, input: SimpleScriptTxIn) -> Result<(), WError> {
        match input.simple_script_tx_in {
            SimpleScriptTxInParameter::ProvidedSimpleScriptSource(script) => {
                self.tx_inputs_builder.add_native_script_input(
                    &csl::NativeScriptSource::new(
                        &csl::NativeScript::from_hex(&script.script_cbor).map_err(
                            WError::from_err(
                                "CoreCSL - add_simple_script_tx_in - invalid script_cbor",
                            ),
                        )?,
                    ),
                    &csl::TransactionInput::new(
                        &csl::TransactionHash::from_hex(&input.tx_in.tx_hash).map_err(
                            WError::from_err(
                                "CoreCSL - add_simple_script_tx_in - invalid tx_hash (1)",
                            ),
                        )?,
                        input.tx_in.tx_index,
                    ),
                    &to_value(&input.tx_in.amount.unwrap())?,
                );
                Ok(())
            }
            SimpleScriptTxInParameter::InlineSimpleScriptSource(script) => {
                self.tx_inputs_builder.add_native_script_input(
                    &csl::NativeScriptSource::new_ref_input(
                        &csl::ScriptHash::from_hex(&script.simple_script_hash).map_err(
                            WError::from_err(
                                "CoreCSL - add_simple_script_tx_in - invalid simple_script_hash",
                            ),
                        )?,
                        &csl::TransactionInput::new(
                            &csl::TransactionHash::from_hex(&script.ref_tx_in.tx_hash).map_err(
                                WError::from_err(
                                    "CoreCSL - add_simple_script_tx_in - invalid tx_hash (2)",
                                ),
                            )?,
                            script.ref_tx_in.tx_index,
                        ),
                        script.script_size,
                    ),
                    &csl::TransactionInput::new(
                        &csl::TransactionHash::from_hex(&input.tx_in.tx_hash).map_err(
                            WError::from_err(
                                "CoreCSL - add_simple_script_tx_in - invalid tx_hash (3)",
                            ),
                        )?,
                        input.tx_in.tx_index,
                    ),
                    &to_value(&input.tx_in.amount.unwrap())?,
                );
                Ok(())
            }
        }
    }

    pub fn add_script_tx_in(&mut self, input: ScriptTxIn) -> Result<(), WError> {
        let datum_source = input.script_tx_in.datum_source.unwrap();
        let script_source = input.script_tx_in.script_source.unwrap();
        let redeemer = input.script_tx_in.redeemer.unwrap();
        let csl_datum: Option<csl::DatumSource> = match datum_source {
            DatumSource::ProvidedDatumSource(datum) => Some(csl::DatumSource::new(
                &csl::PlutusData::from_hex(&datum.data).map_err(WError::from_err(
                    "CoreCSL - add_script_tx_in - invalid datum.data",
                ))?,
            )),
            DatumSource::InlineDatumSource(datum) => {
                let ref_input = csl::TransactionInput::new(
                    &csl::TransactionHash::from_hex(&datum.tx_hash).map_err(WError::from_err(
                        "CoreCSL - add_script_tx_in - invalid tx_hash",
                    ))?,
                    datum.tx_index,
                );
                if input.tx_in.tx_hash == datum.tx_hash && input.tx_in.tx_index == datum.tx_index {
                    None
                } else {
                    Some(csl::DatumSource::new_ref_input(&ref_input))
                }
            }
        };

        let csl_script: csl::PlutusScriptSource = to_csl_script_source(script_source)?;

        let csl_redeemer: csl::Redeemer = csl::Redeemer::new(
            &csl::RedeemerTag::new_spend(),
            &to_bignum(0).unwrap(),
            &csl::PlutusData::from_hex(&redeemer.data).map_err(WError::from_err(
                "CoreCSL - add_script_tx_in - invalid redeemer.data",
            ))?,
            &csl::ExUnits::new(
                &to_bignum(redeemer.ex_units.mem).map_err(WError::add_err_trace(
                    "CoreCSL - add_script_tx_in - invalid redeemer memory",
                ))?,
                &to_bignum(redeemer.ex_units.steps).map_err(WError::add_err_trace(
                    "CoreCSL - add_script_tx_in - invalid redeemer steps",
                ))?,
            ),
        );

        let csl_plutus_witness = match csl_datum {
            Some(datum) => csl::PlutusWitness::new_with_ref(&csl_script, &datum, &csl_redeemer),
            None => csl::PlutusWitness::new_with_ref_without_datum(&csl_script, &csl_redeemer),
        };

        self.tx_inputs_builder.add_plutus_script_input(
            &csl_plutus_witness,
            &csl::TransactionInput::new(
                &csl::TransactionHash::from_hex(&input.tx_in.tx_hash).map_err(WError::from_err(
                    "CoreCSL - add_script_tx_in - invalid tx_hash (2)",
                ))?,
                input.tx_in.tx_index,
            ),
            &to_value(&input.tx_in.amount.unwrap())?,
        );
        Ok(())
    }

    pub fn add_output(&mut self, output: Output) -> Result<(), WError> {
        let mut output_address = csl::Address::from_bech32(&output.address);
        // If the address is not in bech32 format, it might be a Byron address
        match output_address {
            Ok(_) => {}
            Err(_) => {
                output_address = csl::ByronAddress::from_base58(&output.address)
                    .map(|byron_addr| byron_addr.to_address());
            }
        };
        let mut output_builder =
            csl::TransactionOutputBuilder::new().with_address(&output_address.map_err(
                WError::from_err("CoreCSL - add_output - invalid output_address"),
            )?);
        if output.datum.is_some() {
            let datum = output.datum.unwrap();

            match datum {
                Datum::Hash(data) => {
                    output_builder = output_builder.with_data_hash(&csl::hash_plutus_data(
                        &csl::PlutusData::from_hex(&data).map_err(WError::from_err(
                            "CoreCSL - add_output - invalid datum hash",
                        ))?,
                    ));
                }
                Datum::Inline(data) => {
                    output_builder = output_builder.with_plutus_data(
                        &csl::PlutusData::from_hex(&data).map_err(WError::from_err(
                            "CoreCSL - add_output - invalid inline datum",
                        ))?,
                    );
                }
                Datum::Embedded(data) => {
                    let datum = &csl::PlutusData::from_hex(&data).map_err(WError::from_err(
                        "CoreCSL - add_output - invalid embedded datum",
                    ))?;
                    output_builder = output_builder.with_data_hash(&csl::hash_plutus_data(datum));
                    self.tx_builder.add_extra_witness_datum(datum);
                }
            };
        }

        if output.reference_script.is_some() {
            let output_script = output.reference_script.unwrap();
            match output_script {
                OutputScriptSource::ProvidedScriptSource(script) => {
                    let language_version: csl::Language = match script.language_version {
                        LanguageVersion::V1 => csl::Language::new_plutus_v1(),
                        LanguageVersion::V2 => csl::Language::new_plutus_v2(),
                        LanguageVersion::V3 => csl::Language::new_plutus_v3(),
                    };
                    output_builder =
                        output_builder.with_script_ref(&csl::ScriptRef::new_plutus_script(
                            &csl::PlutusScript::from_hex_with_version(
                                &script.script_cbor,
                                &language_version,
                            )
                            .map_err(WError::from_err(
                                "CoreCSL - add_output - invalid script_cbor",
                            ))?,
                        ))
                }
                OutputScriptSource::ProvidedSimpleScriptSource(script) => {
                    output_builder =
                        output_builder.with_script_ref(&csl::ScriptRef::new_native_script(
                            &csl::NativeScript::from_hex(&script.script_cbor).map_err(
                                WError::from_err(
                                    "CoreCSL - add_output - invalid simple script_cbor",
                                ),
                            )?,
                        ))
                }
            }
        }

        let tx_value = to_value(&output.amount)?;
        let amount_builder = output_builder
            .next()
            .map_err(WError::from_err("CoreCSL - add_output - next output"))?;
        let built_output: csl::TransactionOutput = if tx_value.multiasset().is_some() {
            if tx_value.coin().is_zero() {
                amount_builder
                    .with_asset_and_min_required_coin_by_utxo_cost(
                        &tx_value.multiasset().unwrap(),
                        &csl::DataCost::new_coins_per_byte(
                            &to_bignum(self.protocol_params.coins_per_utxo_size).map_err(
                                WError::add_err_trace(
                                    "CoreCSL - add_output - invalid coins_per_utxo_size",
                                ),
                            )?,
                        ),
                    )
                    .map_err(WError::from_err(
                        "CoreCSL - add_output - with_asset_and_min_required_coin_by_utxo_cost",
                    ))?
                    .build()
                    .map_err(WError::from_err("CoreCSL - add_output - build() (1)"))?
            } else {
                amount_builder
                    .with_coin_and_asset(&tx_value.coin(), &tx_value.multiasset().unwrap())
                    .build()
                    .map_err(WError::from_err("CoreCSL - add_output - build() (2)"))?
            }
        } else {
            amount_builder
                .with_coin(&tx_value.coin())
                .build()
                .map_err(WError::from_err("CoreCSL - add_output - build() (3)"))?
        };
        self.tx_builder
            .add_output(&built_output)
            .map_err(WError::from_err("CoreCSL - add_output - add_output"))?;
        Ok(())
    }

    pub fn add_collateral(&mut self, collateral: PubKeyTxIn) -> Result<(), WError> {
        self.collateral_builder
            .add_regular_input(
                &csl::Address::from_bech32(&collateral.tx_in.address.unwrap()).map_err(
                    WError::from_err("CoreCSL - add_collateral - invalid address"),
                )?,
                &csl::TransactionInput::new(
                    &csl::TransactionHash::from_hex(&collateral.tx_in.tx_hash).map_err(
                        WError::from_err("CoreCSL - add_collateral - invalid tx_hash"),
                    )?,
                    collateral.tx_in.tx_index,
                ),
                &to_value(&collateral.tx_in.amount.unwrap())?,
            )
            .map_err(WError::from_err(
                "CoreCSL - add_collateral - add_regular_input",
            ))?;
        Ok(())
    }

    pub fn add_reference_input(&mut self, ref_input: RefTxIn) -> Result<(), WError> {
        let csl_ref_input = csl::TransactionInput::new(
            &csl::TransactionHash::from_hex(&ref_input.tx_hash).map_err(WError::from_err(
                "CoreCSL - add_reference_input - invalid_tx_hash",
            ))?,
            ref_input.tx_index,
        );
        if ref_input.script_size.is_some() {
            self.tx_builder
                .add_script_reference_input(&csl_ref_input, ref_input.script_size.unwrap());
        } else {
            self.tx_builder.add_reference_input(&csl_ref_input);
        }
        Ok(())
    }

    pub fn add_pub_key_withdrawal(&mut self, withdrawal: PubKeyWithdrawal) -> Result<(), WError> {
        self.tx_withdrawals_builder
            .add(
                &csl::RewardAddress::from_address(
                    &csl::Address::from_bech32(&withdrawal.address).map_err(WError::from_err(
                        "CoreCSL - add_pub_key_withdrawal - invalid address",
                    ))?,
                )
                .unwrap(),
                &csl::BigNum::from_str(&withdrawal.coin.to_string()).map_err(WError::from_err(
                    "CoreCSL - add_collateral - invalid coin as BigNum",
                ))?,
            )
            .map_err(WError::from_err("CoreCSL - add_pub_key_withdrawal - add()"))?;
        Ok(())
    }

    pub fn add_plutus_withdrawal(
        &mut self,
        withdrawal: PlutusScriptWithdrawal,
    ) -> Result<(), WError> {
        let script_source = withdrawal.script_source.unwrap();
        let redeemer = withdrawal.redeemer.unwrap();

        let csl_script: csl::PlutusScriptSource = to_csl_script_source(script_source)?;

        let csl_redeemer: csl::Redeemer = csl::Redeemer::new(
            &csl::RedeemerTag::new_spend(),
            &to_bignum(0).unwrap(),
            &csl::PlutusData::from_hex(&redeemer.data).map_err(WError::from_err(
                "CoreCSL - add_plutus_withdrawal - invalid redeemer.data",
            ))?,
            &csl::ExUnits::new(
                &to_bignum(redeemer.ex_units.mem).map_err(WError::add_err_trace(
                    "CoreCSL - add_plutus_withdrawal - invalid redeemer memory",
                ))?,
                &to_bignum(redeemer.ex_units.steps).map_err(WError::add_err_trace(
                    "CoreCSL - add_plutus_withdrawal - invalid redeemer steps",
                ))?,
            ),
        );

        self.tx_withdrawals_builder
            .add_with_plutus_witness(
                &csl::RewardAddress::from_address(
                    &csl::Address::from_bech32(&withdrawal.address).map_err(WError::from_err(
                        "CoreCSL - add_plutus_withdrawal - invalid address",
                    ))?,
                )
                .unwrap(),
                &csl::BigNum::from_str(&withdrawal.coin.to_string()).map_err(WError::from_err(
                    "CoreCSL - add_plutus_withdrawal - invalid coin as BigNum",
                ))?,
                &csl::PlutusWitness::new_with_ref_without_datum(&csl_script, &csl_redeemer),
            )
            .map_err(WError::from_err(
                "CoreCSL - add_plutus_withdrawal - add_with_plutus_witness",
            ))?;
        Ok(())
    }

    pub fn add_simple_script_withdrawal(
        &mut self,
        withdrawal: SimpleScriptWithdrawal,
    ) -> Result<(), WError> {
        let csl_native_script_source = match withdrawal.script_source {
            Some(script_source) => match script_source {
                SimpleScriptSource::ProvidedSimpleScriptSource(ProvidedSimpleScriptSource {
                    script_cbor: provided_script,
                }) => csl::NativeScriptSource::new(
                    &csl::NativeScript::from_hex(&provided_script).unwrap(),
                ),
                SimpleScriptSource::InlineSimpleScriptSource(InlineSimpleScriptSource {
                    ref_tx_in,
                    simple_script_hash,
                    script_size,
                }) => csl::NativeScriptSource::new_ref_input(
                    &csl::ScriptHash::from_hex(&simple_script_hash).unwrap(),
                    &csl::TransactionInput::new(
                        &csl::TransactionHash::from_hex(&ref_tx_in.tx_hash).unwrap(),
                        ref_tx_in.tx_index,
                    ),
                    script_size,
                ),
            },
            None => {
                return Err(WError::new(
                    "add_simple_script_withdrawal",
                    "Missing script source for native script withdrawal",
                ))
            }
        };

        self.tx_withdrawals_builder
            .add_with_native_script(
                &csl::RewardAddress::from_address(
                    &csl::Address::from_bech32(&withdrawal.address).map_err(WError::from_err(
                        "CoreCSL - add_simple_script_withdrawal - invalid reward address",
                    ))?,
                )
                .unwrap(),
                &csl::BigNum::from_str(&withdrawal.coin.to_string()).map_err(WError::from_err(
                    "CoreCSL - add_simple_script_withdrawal - invalid coin as BigNum",
                ))?,
                &csl_native_script_source,
            )
            .map_err(WError::from_err(
                "CoreCSL - add_simple_script_withdrawal - add_with_native_script",
            ))
    }

    pub fn add_plutus_mint(&mut self, script_mint: ScriptMint, index: u64) -> Result<(), WError> {
        let redeemer_info = script_mint.redeemer.unwrap();
        let mint_redeemer = csl::Redeemer::new(
            &csl::RedeemerTag::new_mint(),
            &to_bignum(index).map_err(WError::from_err(
                "CoreCSL - add_plutus_mint - invalid redeemer index",
            ))?,
            &csl::PlutusData::from_hex(&redeemer_info.data).map_err(WError::from_err(
                "CoreCSL - add_plutus_mint - invalid redeemer_info.data",
            ))?,
            &csl::ExUnits::new(
                &to_bignum(redeemer_info.ex_units.mem).map_err(WError::from_err(
                    "CoreCSL - add_plutus_mint - invalid redeemer_info memory",
                ))?,
                &to_bignum(redeemer_info.ex_units.steps).map_err(WError::from_err(
                    "CoreCSL - add_plutus_mint - invalid redeemer_info steps",
                ))?,
            ),
        );
        let script_source_info = script_mint.script_source.unwrap();
        let mint_script = to_csl_script_source(script_source_info)?;
        self.mint_builder
            .add_asset(
                &csl::MintWitness::new_plutus_script(&mint_script, &mint_redeemer),
                &csl::AssetName::new(hex::decode(script_mint.mint.asset_name).map_err(
                    WError::from_err("CoreCSL - add_plutus_mint - Invalid asset name found"),
                )?)
                .map_err(WError::from_err(
                    "CoreCSL - add_plutus_mint - invalid asset name",
                ))?,
                &csl::Int::from_str(&script_mint.mint.amount.to_string()).map_err(
                    WError::from_err("CoreCSL - add_plutus_mint - invalid mint amount "),
                )?,
            )
            .map_err(WError::from_err("CoreCSL - add_plutus_mint - add_asset"))?;
        Ok(())
    }

    pub fn add_native_mint(&mut self, native_mint: SimpleScriptMint) -> Result<(), WError> {
        let script_info = native_mint.script_source.unwrap();
        match script_info {
            SimpleScriptSource::ProvidedSimpleScriptSource(script) => self
                .mint_builder
                .add_asset(
                    &csl::MintWitness::new_native_script(&csl::NativeScriptSource::new(
                        &csl::NativeScript::from_hex(&script.script_cbor).map_err(
                            WError::from_err("CoreCSL - add_native_mint - invalid script_cbor"),
                        )?,
                    )),
                    &csl::AssetName::new(hex::decode(native_mint.mint.asset_name).map_err(
                        WError::from_err(
                            "CoreCSL - add_native_mint - Invalid asset name found (1)",
                        ),
                    )?)
                    .map_err(WError::from_err(
                        "CoreCSL - add_native_mint - invalid asset name (1)",
                    ))?,
                    &csl::Int::from_str(&native_mint.mint.amount.to_string()).unwrap(),
                )
                .map_err(WError::from_err(
                    "CoreCSL - add_native_mint - add_asset (1)",
                ))?,
            SimpleScriptSource::InlineSimpleScriptSource(script) => self
                .mint_builder
                .add_asset(
                    &csl::MintWitness::new_native_script(&csl::NativeScriptSource::new_ref_input(
                        &csl::ScriptHash::from_hex(&script.simple_script_hash).map_err(
                            WError::from_err(
                                "CoreCSL - add_native_mint - invalid simple_script_hash",
                            ),
                        )?,
                        &csl::TransactionInput::new(
                            &csl::TransactionHash::from_hex(&script.ref_tx_in.tx_hash).map_err(
                                WError::from_err("CoreCSL - add_native_mint - invalid tx_hash"),
                            )?,
                            script.ref_tx_in.tx_index,
                        ),
                        script.script_size,
                    )),
                    &csl::AssetName::new(hex::decode(native_mint.mint.asset_name).map_err(
                        WError::from_err(
                            "CoreCSL - add_native_mint - Invalid asset name found (2)",
                        ),
                    )?)
                    .map_err(WError::from_err(
                        "CoreCSL - add_native_mint - invalid asset name (2)",
                    ))?,
                    &csl::Int::from_str(&native_mint.mint.amount.to_string()).unwrap(),
                )
                .map_err(WError::from_err(
                    "CoreCSL - add_native_mint - add_asset (2)",
                ))?,
        };
        Ok(())
    }

    pub fn add_cert(&mut self, cert: Certificate, index: u64) -> Result<(), WError> {
        match cert {
            Certificate::BasicCertificate(basic_cert) => self
                .certificates_builder
                .add(&to_csl_cert(basic_cert)?)
                .map_err(WError::from_err("CoreCSL - add_cert - add (1)"))?,
            Certificate::ScriptCertificate(script_cert) => {
                let cert_script_source: csl::PlutusScriptSource = match script_cert.script_source {
                    Some(script_source) => to_csl_script_source(script_source)?,
                    None => {
                        return Err(WError::new(
                            "CoreCSL - add_cert",
                            "Missing Plutus Script Source in Plutus Cert",
                        ))
                    }
                };
                let cert_redeemer = match script_cert.redeemer {
                    Some(redeemer) => to_csl_redeemer(RedeemerTag::Cert, redeemer, index)?,
                    None => {
                        return Err(WError::new(
                            "CoreCSL - add_cert",
                            "Missing Redeemer in Plutus Cert",
                        ))
                    }
                };
                let csl_plutus_witness: csl::PlutusWitness =
                    csl::PlutusWitness::new_with_ref_without_datum(
                        &cert_script_source,
                        &cert_redeemer,
                    );
                self.certificates_builder
                    .add_with_plutus_witness(&to_csl_cert(script_cert.cert)?, &csl_plutus_witness)
                    .map_err(WError::from_err(
                        "CoreCSL - add_cert - add_with_plutus_witness",
                    ))?
            }
            Certificate::SimpleScriptCertificate(simple_script_cert) => {
                let script_info = simple_script_cert.simple_script_source;
                let script_source: csl::NativeScriptSource = match script_info {
                    Some(simple_script_source) => {
                        to_csl_simple_script_source(simple_script_source)?
                    }
                    None => {
                        return Err(WError::new(
                            "CoreCSL - add_cert",
                            "Missing Native Script Source in Native Cert",
                        ))
                    }
                };
                self.certificates_builder
                    .add_with_native_script(&to_csl_cert(simple_script_cert.cert)?, &script_source)
                    .map_err(WError::from_err(
                        "CoreCSL - add_cert - add_with_native_script",
                    ))?
            }
        };
        Ok(())
    }

    pub fn add_vote(&mut self, vote: Vote, index: u64) -> Result<(), WError> {
        match vote {
            Vote::BasicVote(vote_type) => {
                let voter = to_csl_voter(vote_type.voter)
                    .map_err(WError::from_err("CoreCSL - add_vote - invalid voter"))?;
                let vote_kind = to_csl_vote_kind(vote_type.voting_procedure.vote_kind);
                let voting_procedure = match vote_type.voting_procedure.anchor {
                    Some(anchor) => {
                        csl::VotingProcedure::new_with_anchor(vote_kind, &to_csl_anchor(&anchor)?)
                    }
                    None => csl::VotingProcedure::new(vote_kind),
                };
                self.vote_builder
                    .add(
                        &voter,
                        &csl::GovernanceActionId::new(
                            &csl::TransactionHash::from_hex(&vote_type.gov_action_id.tx_hash)
                                .map_err(WError::from_err(
                                    "CoreCSL - add_vote - invalid tx_hash (1)",
                                ))?,
                            vote_type.gov_action_id.tx_index,
                        ),
                        &voting_procedure,
                    )
                    .map_err(WError::from_err("CoreCSL - add_vote - add"))?;
            }
            Vote::ScriptVote(script_vote) => {
                let voter = to_csl_voter(script_vote.vote.voter)?;
                let vote_kind = to_csl_vote_kind(script_vote.vote.voting_procedure.vote_kind);
                let voting_procedure = match script_vote.vote.voting_procedure.anchor {
                    Some(anchor) => {
                        csl::VotingProcedure::new_with_anchor(vote_kind, &to_csl_anchor(&anchor)?)
                    }
                    None => csl::VotingProcedure::new(vote_kind),
                };
                let vote_script_source: csl::PlutusScriptSource = match script_vote.script_source {
                    Some(script_source) => to_csl_script_source(script_source)?,
                    None => {
                        return Err(WError::new(
                            "CoreCSL - add_vote",
                            "Missing Plutus Script Source in Plutus Vote",
                        ))
                    }
                };
                let vote_redeemer = match script_vote.redeemer {
                    Some(redeemer) => to_csl_redeemer(RedeemerTag::Vote, redeemer, index)?,
                    None => {
                        return Err(WError::new(
                            "CoreCSL - add_vote",
                            "Missing Redeemer in Plutus Vote",
                        ))
                    }
                };
                let csl_plutus_witness: csl::PlutusWitness =
                    csl::PlutusWitness::new_with_ref_without_datum(
                        &vote_script_source,
                        &vote_redeemer,
                    );
                self.vote_builder
                    .add_with_plutus_witness(
                        &voter,
                        &csl::GovernanceActionId::new(
                            &csl::TransactionHash::from_hex(
                                &script_vote.vote.gov_action_id.tx_hash,
                            )
                            .map_err(WError::from_err(
                                "CoreCSL - add_vote - invalid tx_hash (2)",
                            ))?,
                            script_vote.vote.gov_action_id.tx_index,
                        ),
                        &voting_procedure,
                        &csl_plutus_witness,
                    )
                    .map_err(WError::from_err(
                        "CoreCSL - add_vote - add_with_plutus_witness",
                    ))?;
            }
            Vote::SimpleScriptVote(simple_script_vote) => {
                let voter = to_csl_voter(simple_script_vote.vote.voter)?;
                let vote_kind =
                    to_csl_vote_kind(simple_script_vote.vote.voting_procedure.vote_kind);
                let voting_procedure = match simple_script_vote.vote.voting_procedure.anchor {
                    Some(anchor) => {
                        csl::VotingProcedure::new_with_anchor(vote_kind, &to_csl_anchor(&anchor)?)
                    }
                    None => csl::VotingProcedure::new(vote_kind),
                };
                let csl_simple_script_source = match simple_script_vote.simple_script_source {
                    Some(simple_script_source) => {
                        to_csl_simple_script_source(simple_script_source)?
                    }
                    None => {
                        return Err(WError::new(
                            "CoreCSL - add_vote",
                            "Missing Native Script Source in Native Vote",
                        ))
                    }
                };
                self.vote_builder
                    .add_with_native_script(
                        &voter,
                        &csl::GovernanceActionId::new(
                            &csl::TransactionHash::from_hex(
                                &simple_script_vote.vote.gov_action_id.tx_hash,
                            )
                            .map_err(WError::from_err(
                                "CoreCSL - add_vote - invalid tx_hash (3)",
                            ))?,
                            simple_script_vote.vote.gov_action_id.tx_index,
                        ),
                        &voting_procedure,
                        &csl_simple_script_source,
                    )
                    .map_err(WError::from_err(
                        "CoreCSL - add_vote - add_with_native_script",
                    ))?;
            }
        };
        Ok(())
    }

    pub fn add_invalid_before(&mut self, invalid_before: u64) -> Result<(), WError> {
        self.tx_builder
            .set_validity_start_interval_bignum(to_bignum(invalid_before).map_err(
                WError::from_err("CoreCSL - add_invalid_before - invalid invalid_before"),
            )?);
        Ok(())
    }

    pub fn add_invalid_hereafter(&mut self, invalid_hereafter: u64) -> Result<(), WError> {
        self.tx_builder
            .set_ttl_bignum(&to_bignum(invalid_hereafter).map_err(WError::from_err(
                "CoreCSL - add_invalid_hereafter - invalid invalid_hereafter",
            ))?);
        Ok(())
    }

    pub fn set_fee(&mut self, fee: String) {
        self.tx_builder
            .set_fee(&csl::BigNum::from_str(&fee).expect("Error parsing fee amount"));
    }

    pub fn add_change(
        &mut self,
        change_address: String,
        change_datum: Option<Datum>,
    ) -> Result<(), WError> {
        let mut output_address = csl::Address::from_bech32(&change_address);
        // If the address is not in bech32 format, it might be a Byron address
        match output_address {
            Ok(_) => {}
            Err(_) => {
                output_address = csl::ByronAddress::from_base58(&change_address)
                    .map(|byron_addr| byron_addr.to_address());
            }
        };
        if let Some(change_datum) = change_datum {
            self.tx_builder
                .add_change_if_needed_with_datum(
                    &output_address.map_err(WError::from_err(
                        "CoreCSL - add_change - invalid change_address (1)",
                    ))?,
                    &csl::OutputDatum::new_data(
                        &csl::PlutusData::from_hex(change_datum.get_inner()).map_err(
                            WError::from_err("CoreCSL - add_change - invalid change_datum"),
                        )?,
                    ),
                )
                .map_err(WError::from_err(
                    "CoreCSL - add_change - add_change_if_needed_with_datum",
                ))?;
        } else {
            self.tx_builder
                .add_change_if_needed(&output_address.map_err(WError::from_err(
                    "CoreCSL - add_change - invalid change_address (2)",
                ))?)
                .map_err(WError::from_err(
                    "CoreCSL - add_change - add_change_if_needed",
                ))?;
        }
        Ok(())
    }

    pub fn add_signing_keys(&mut self, signing_keys: &[&str]) -> Result<(), WError> {
        self.tx_hex = sign_transaction(&self.tx_hex, signing_keys)?;
        Ok(())
    }

    pub fn add_required_signature(&mut self, pub_key_hash: &str) -> Result<(), WError> {
        self.tx_builder
            .add_required_signer(&csl::Ed25519KeyHash::from_hex(pub_key_hash).map_err(
                WError::from_err("CoreCSL - add_required_signature - invalid pub_key_hash"),
            )?);
        Ok(())
    }

    pub fn add_metadata(&mut self, metadata: Metadata) -> Result<(), WError> {
        self.tx_builder
            .add_json_metadatum(
                &csl::BigNum::from_str(&metadata.tag).map_err(WError::from_err(
                    "CoreCSL - add_metadata - invalid metadata tag",
                ))?,
                metadata.metadata,
            )
            .map_err(WError::from_err(
                "CoreCSL - add_metadata - add_json_metadatum",
            ))?;
        Ok(())
    }

    pub fn add_script_hash(&mut self, network: Network) -> Result<(), WError> {
        self.tx_builder
            .calc_script_data_hash(&build_csl_cost_models(&network))
            .map_err(WError::from_err(
                "CoreCSL - add_script_hash - calc_script_data_hash",
            ))?;
        Ok(())
    }

    pub fn build_tx(&mut self, safe_build: bool) -> Result<String, WError> {
        let tx = if safe_build {
            self.tx_builder.build_tx()
        } else {
            self.tx_builder.build_tx_unsafe()
        }
        .map_err(WError::from_err("CoreCSL - build_tx - build_tx"))?;

        self.tx_hex = tx.to_hex();
        Ok(self.tx_hex.to_string())
    }
}

// impl Default for CoreCSL {
//     pub fn default() -> Self {
//         Self::new(None).unwrap()
//     }
// }
