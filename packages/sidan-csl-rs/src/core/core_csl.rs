use crate::{csl, model::*};
use cardano_serialization_lib::{JsError, MintWitness};

use super::{
    constants::build_csl_cost_models,
    utils::{
        build_tx_builder, sign_transaction, to_bignum, to_csl_anchor, to_csl_cert, to_csl_redeemer,
        to_csl_script_source, to_csl_simple_script_source, to_csl_vote_kind, to_csl_voter,
        to_value,
    },
};

#[derive(Clone, Debug)]
pub struct MeshCSL {
    pub tx_hex: String,
    pub tx_builder: csl::TransactionBuilder,
    pub tx_inputs_builder: csl::TxInputsBuilder,
    pub tx_withdrawals_builder: csl::WithdrawalsBuilder,
    pub protocol_params: Protocol,
}

impl MeshCSL {
    pub fn new(params: Option<Protocol>) -> MeshCSL {
        MeshCSL {
            tx_hex: String::new(),
            tx_builder: build_tx_builder(params.clone()),
            tx_inputs_builder: csl::TxInputsBuilder::new(),
            tx_withdrawals_builder: csl::WithdrawalsBuilder::new(),
            protocol_params: params.unwrap_or_default(),
        }
    }

    pub fn add_tx_in(&mut self, input: PubKeyTxIn) -> Result<(), JsError> {
        self.tx_inputs_builder.add_regular_input(
            &csl::Address::from_bech32(&input.tx_in.address.unwrap())?,
            &csl::TransactionInput::new(
                &csl::TransactionHash::from_hex(&input.tx_in.tx_hash)?,
                input.tx_in.tx_index,
            ),
            &to_value(&input.tx_in.amount.unwrap()),
        )?;
        Ok(())
    }

    pub fn add_simple_script_tx_in(&mut self, input: SimpleScriptTxIn) -> Result<(), JsError> {
        match input.simple_script_tx_in {
            SimpleScriptTxInParameter::ProvidedSimpleScriptSource(script) => {
                self.tx_inputs_builder.add_native_script_input(
                    &csl::NativeScriptSource::new(&csl::NativeScript::from_hex(
                        &script.script_cbor,
                    )?),
                    &csl::TransactionInput::new(
                        &csl::TransactionHash::from_hex(&input.tx_in.tx_hash)?,
                        input.tx_in.tx_index,
                    ),
                    &to_value(&input.tx_in.amount.unwrap()),
                );
                Ok(())
            }
            SimpleScriptTxInParameter::InlineSimpleScriptSource(script) => {
                self.tx_inputs_builder.add_native_script_input(
                    &csl::NativeScriptSource::new_ref_input(
                        &csl::ScriptHash::from_hex(&script.simple_script_hash)?,
                        &csl::TransactionInput::new(
                            &csl::TransactionHash::from_hex(&script.ref_tx_in.tx_hash)?,
                            script.ref_tx_in.tx_index,
                        ),
                        script.script_size,
                    ),
                    &csl::TransactionInput::new(
                        &csl::TransactionHash::from_hex(&input.tx_in.tx_hash)?,
                        input.tx_in.tx_index,
                    ),
                    &to_value(&input.tx_in.amount.unwrap()),
                );
                Ok(())
            }
        }
    }

    pub fn add_script_tx_in(&mut self, input: ScriptTxIn) -> Result<(), JsError> {
        let datum_source = input.script_tx_in.datum_source.unwrap();
        let script_source = input.script_tx_in.script_source.unwrap();
        let redeemer = input.script_tx_in.redeemer.unwrap();
        let csl_datum: Option<csl::DatumSource> = match datum_source {
            DatumSource::ProvidedDatumSource(datum) => Some(csl::DatumSource::new(
                &csl::PlutusData::from_hex(&datum.data)?,
            )),
            DatumSource::InlineDatumSource(datum) => {
                let ref_input = csl::TransactionInput::new(
                    &csl::TransactionHash::from_hex(&datum.tx_hash)?,
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
            &to_bignum(0),
            &csl::PlutusData::from_hex(&redeemer.data)?,
            &csl::ExUnits::new(
                &to_bignum(redeemer.ex_units.mem),
                &to_bignum(redeemer.ex_units.steps),
            ),
        );

        let csl_plutus_witness = match csl_datum {
            Some(datum) => csl::PlutusWitness::new_with_ref(&csl_script, &datum, &csl_redeemer),
            None => csl::PlutusWitness::new_with_ref_without_datum(&csl_script, &csl_redeemer),
        };

        self.tx_inputs_builder.add_plutus_script_input(
            &csl_plutus_witness,
            &csl::TransactionInput::new(
                &csl::TransactionHash::from_hex(&input.tx_in.tx_hash)?,
                input.tx_in.tx_index,
            ),
            &to_value(&input.tx_in.amount.unwrap()),
        );
        Ok(())
    }

    pub fn add_output(&mut self, output: Output) -> Result<(), JsError> {
        let mut output_address = csl::Address::from_bech32(&output.address);
        // If the address is not in bech32 format, it might be a Byron address
        match output_address {
            Ok(_) => {}
            Err(_) => {
                output_address = csl::ByronAddress::from_base58(&output.address)
                    .map(|byron_addr| byron_addr.to_address());
            }
        };
        let mut output_builder = csl::TransactionOutputBuilder::new()
            .with_address(&output_address?);
        if output.datum.is_some() {
            let datum = output.datum.unwrap();

            match datum {
                Datum::Hash(data) => {
                    output_builder = output_builder
                        .with_data_hash(&csl::hash_plutus_data(&csl::PlutusData::from_hex(&data)?));
                }
                Datum::Inline(data) => {
                    output_builder =
                        output_builder.with_plutus_data(&csl::PlutusData::from_hex(&data)?);
                }
                Datum::Embedded(data) => {
                    let datum = &csl::PlutusData::from_hex(&data)?;
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
                            )?,
                        ))
                }
                OutputScriptSource::ProvidedSimpleScriptSource(script) => {
                    output_builder =
                        output_builder.with_script_ref(&csl::ScriptRef::new_native_script(
                            &csl::NativeScript::from_hex(&script.script_cbor)?,
                        ))
                }
            }
        }

        let tx_value = to_value(&output.amount);
        let amount_builder = output_builder.next()?;
        let mut built_output: csl::TransactionOutput = if tx_value.multiasset().is_some() {
            if tx_value.coin().is_zero() {
                amount_builder
                    .with_asset_and_min_required_coin_by_utxo_cost(
                        &tx_value.multiasset().unwrap(),
                        &csl::DataCost::new_coins_per_byte(&to_bignum(
                            self.protocol_params.coins_per_utxo_size,
                        )),
                    )?
                    .build()?
            } else {
                amount_builder
                    .with_coin_and_asset(&tx_value.coin(), &tx_value.multiasset().unwrap())
                    .build()?
            }
        } else {
            amount_builder.with_coin(&tx_value.coin()).build()?
        };
        match built_output.amount().multiasset() {
            Some(multiasset) => {
                if multiasset.len() == 0 {
                    built_output = csl::TransactionOutput::new(
                        &built_output.address(),
                        &csl::Value::new(&built_output.amount().coin()),
                    )
                }
            }
            None => {}
        }
        self.tx_builder.add_output(&built_output)?;
        Ok(())
    }

    pub fn add_collateral(
        &mut self,
        collateral_builder: &mut csl::TxInputsBuilder,
        collateral: PubKeyTxIn,
    ) -> Result<(), JsError> {
        collateral_builder.add_regular_input(
            &csl::Address::from_bech32(&collateral.tx_in.address.unwrap())?,
            &csl::TransactionInput::new(
                &csl::TransactionHash::from_hex(&collateral.tx_in.tx_hash)?,
                collateral.tx_in.tx_index,
            ),
            &to_value(&collateral.tx_in.amount.unwrap()),
        )?;
        Ok(())
    }

    pub fn add_reference_input(&mut self, ref_input: RefTxIn) -> Result<(), JsError> {
        let csl_ref_input = csl::TransactionInput::new(
            &csl::TransactionHash::from_hex(&ref_input.tx_hash)?,
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

    pub fn add_pub_key_withdrawal(&mut self, withdrawal: PubKeyWithdrawal) -> Result<(), JsError> {
        self.tx_withdrawals_builder.add(
            &csl::RewardAddress::from_address(&csl::Address::from_bech32(&withdrawal.address)?)
                .unwrap(),
            &csl::BigNum::from_str(&withdrawal.coin.to_string())?,
        )?;
        Ok(())
    }

    pub fn add_plutus_withdrawal(
        &mut self,
        withdrawal: PlutusScriptWithdrawal,
    ) -> Result<(), JsError> {
        let script_source = withdrawal.script_source.unwrap();
        let redeemer = withdrawal.redeemer.unwrap();

        let csl_script: csl::PlutusScriptSource = to_csl_script_source(script_source)?;

        let csl_redeemer: csl::Redeemer = csl::Redeemer::new(
            &csl::RedeemerTag::new_spend(),
            &to_bignum(0),
            &csl::PlutusData::from_hex(&redeemer.data)?,
            &csl::ExUnits::new(
                &to_bignum(redeemer.ex_units.mem),
                &to_bignum(redeemer.ex_units.steps),
            ),
        );

        self.tx_withdrawals_builder.add_with_plutus_witness(
            &csl::RewardAddress::from_address(&csl::Address::from_bech32(&withdrawal.address)?)
                .unwrap(),
            &csl::BigNum::from_str(&withdrawal.coin.to_string())?,
            &csl::PlutusWitness::new_with_ref_without_datum(&csl_script, &csl_redeemer),
        )?;
        Ok(())
    }

    pub fn add_simple_script_withdrawal(
        &mut self,
        withdrawal: SimpleScriptWithdrawal,
    ) -> Result<(), JsError> {
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
                return Err(JsError::from_str(
                    "Missing script source for native script withdrawal",
                ))
            }
        };

        self.tx_withdrawals_builder.add_with_native_script(
            &csl::RewardAddress::from_address(&csl::Address::from_bech32(&withdrawal.address)?)
                .unwrap(),
            &csl::BigNum::from_str(&withdrawal.coin.to_string())?,
            &csl_native_script_source,
        )
    }

    pub fn add_plutus_mint(
        &mut self,
        mint_builder: &mut csl::MintBuilder,
        script_mint: ScriptMint,
        index: u64,
    ) -> Result<(), JsError> {
        let redeemer_info = script_mint.redeemer.unwrap();
        let mint_redeemer = csl::Redeemer::new(
            &csl::RedeemerTag::new_mint(),
            &to_bignum(index),
            &csl::PlutusData::from_hex(&redeemer_info.data)?,
            &csl::ExUnits::new(
                &to_bignum(redeemer_info.ex_units.mem),
                &to_bignum(redeemer_info.ex_units.steps),
            ),
        );
        let script_source_info = script_mint.script_source.unwrap();
        let mint_script = to_csl_script_source(script_source_info)?;
        mint_builder.add_asset(
            &csl::MintWitness::new_plutus_script(&mint_script, &mint_redeemer),
            &csl::AssetName::new(hex::decode(script_mint.mint.asset_name).unwrap())?,
            &csl::Int::from_str(&script_mint.mint.amount.to_string()).unwrap(),
        )?;
        Ok(())
    }

    pub fn add_native_mint(
        &mut self,
        mint_builder: &mut csl::MintBuilder,
        native_mint: SimpleScriptMint,
    ) -> Result<(), JsError> {
        let script_info = native_mint.script_source.unwrap();
        match script_info {
            SimpleScriptSource::ProvidedSimpleScriptSource(script) => mint_builder.add_asset(
                &csl::MintWitness::new_native_script(&csl::NativeScriptSource::new(
                    &csl::NativeScript::from_hex(&script.script_cbor)?,
                )),
                &csl::AssetName::new(hex::decode(native_mint.mint.asset_name).unwrap())?,
                &csl::Int::from_str(&native_mint.mint.amount.to_string()).unwrap(),
            )?,
            SimpleScriptSource::InlineSimpleScriptSource(script) => mint_builder.add_asset(
                &MintWitness::new_native_script(&csl::NativeScriptSource::new_ref_input(
                    &csl::ScriptHash::from_hex(&script.simple_script_hash)?,
                    &csl::TransactionInput::new(
                        &csl::TransactionHash::from_hex(&script.ref_tx_in.tx_hash)?,
                        script.ref_tx_in.tx_index,
                    ),
                    script.script_size,
                )),
                &csl::AssetName::new(hex::decode(native_mint.mint.asset_name).unwrap())?,
                &csl::Int::from_str(&native_mint.mint.amount.to_string()).unwrap(),
            )?,
        };
        Ok(())
    }

    pub fn add_cert(
        &mut self,
        certificates_builder: &mut csl::CertificatesBuilder,
        cert: Certificate,
        index: u64,
    ) -> Result<(), JsError> {
        match cert {
            Certificate::BasicCertificate(basic_cert) => {
                certificates_builder.add(&to_csl_cert(basic_cert)?)?
            }
            Certificate::ScriptCertificate(script_cert) => {
                let cert_script_source: csl::PlutusScriptSource = match script_cert.script_source {
                    Some(script_source) => to_csl_script_source(script_source)?,
                    None => {
                        return Err(JsError::from_str(
                            "Missing Plutus Script Source in Plutus Cert",
                        ))
                    }
                };
                let cert_redeemer = match script_cert.redeemer {
                    Some(redeemer) => to_csl_redeemer(RedeemerTag::Cert, redeemer, index)?,
                    None => return Err(JsError::from_str("Missing Redeemer in Plutus Cert")),
                };
                let csl_plutus_witness: csl::PlutusWitness =
                    csl::PlutusWitness::new_with_ref_without_datum(
                        &cert_script_source,
                        &cert_redeemer,
                    );
                certificates_builder
                    .add_with_plutus_witness(&to_csl_cert(script_cert.cert)?, &csl_plutus_witness)?
            }
            Certificate::SimpleScriptCertificate(simple_script_cert) => {
                let script_info = simple_script_cert.simple_script_source;
                let script_source: csl::NativeScriptSource = match script_info {
                    Some(simple_script_source) => {
                        to_csl_simple_script_source(simple_script_source)?
                    }
                    None => {
                        return Err(JsError::from_str(
                            "Missing Native Script Source in Native Cert",
                        ))
                    }
                };
                certificates_builder.add_with_native_script(
                    &to_csl_cert(simple_script_cert.cert)?,
                    &script_source,
                )?
            }
        };
        Ok(())
    }

    pub fn add_vote(
        &mut self,
        vote_builder: &mut csl::VotingBuilder,
        vote: Vote,
        index: u64,
    ) -> Result<(), JsError> {
        match vote {
            Vote::BasicVote(vote_type) => {
                let voter = to_csl_voter(vote_type.voter)?;
                let vote_kind = to_csl_vote_kind(vote_type.voting_procedure.vote_kind);
                let voting_procedure = match vote_type.voting_procedure.anchor {
                    Some(anchor) => {
                        csl::VotingProcedure::new_with_anchor(vote_kind, &to_csl_anchor(&anchor)?)
                    }
                    None => csl::VotingProcedure::new(vote_kind),
                };
                vote_builder.add(
                    &voter,
                    &csl::GovernanceActionId::new(
                        &csl::TransactionHash::from_hex(&vote_type.gov_action_id.tx_hash)?,
                        vote_type.gov_action_id.tx_index,
                    ),
                    &voting_procedure,
                )?
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
                        return Err(JsError::from_str(
                            "Missing Plutus Script Source in Plutus Vote",
                        ))
                    }
                };
                let vote_redeemer = match script_vote.redeemer {
                    Some(redeemer) => to_csl_redeemer(RedeemerTag::Vote, redeemer, index)?,
                    None => return Err(JsError::from_str("Missing Redeemer in Plutus Vote")),
                };
                let csl_plutus_witness: csl::PlutusWitness =
                    csl::PlutusWitness::new_with_ref_without_datum(
                        &vote_script_source,
                        &vote_redeemer,
                    );
                vote_builder.add_with_plutus_witness(
                    &voter,
                    &csl::GovernanceActionId::new(
                        &csl::TransactionHash::from_hex(&script_vote.vote.gov_action_id.tx_hash)?,
                        script_vote.vote.gov_action_id.tx_index,
                    ),
                    &voting_procedure,
                    &csl_plutus_witness,
                )?
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
                    None => return Err(JsError::from_str("Missing ")),
                };
                vote_builder.add_with_native_script(
                    &voter,
                    &csl::GovernanceActionId::new(
                        &csl::TransactionHash::from_hex(
                            &simple_script_vote.vote.gov_action_id.tx_hash,
                        )?,
                        simple_script_vote.vote.gov_action_id.tx_index,
                    ),
                    &voting_procedure,
                    &csl_simple_script_source,
                )?
            }
        };
        Ok(())
    }

    pub fn add_invalid_before(&mut self, invalid_before: u64) {
        self.tx_builder
            .set_validity_start_interval_bignum(to_bignum(invalid_before));
    }

    pub fn add_invalid_hereafter(&mut self, invalid_hereafter: u64) {
        self.tx_builder
            .set_ttl_bignum(&to_bignum(invalid_hereafter));
    }

    pub fn set_fee(&mut self, fee: String) {
        self.tx_builder
            .set_fee(&csl::BigNum::from_str(&fee).expect("Error parsing fee amount"));
    }

    pub fn add_change(
        &mut self,
        change_address: String,
        change_datum: Option<Datum>,
    ) -> Result<(), JsError> {
        if let Some(change_datum) = change_datum {
            self.tx_builder.add_change_if_needed_with_datum(
                &csl::Address::from_bech32(&change_address)?,
                &csl::OutputDatum::new_data(&csl::PlutusData::from_hex(change_datum.get_inner())?),
            )?;
        } else {
            self.tx_builder
                .add_change_if_needed(&csl::Address::from_bech32(&change_address)?)?;
        }
        Ok(())
    }

    pub fn add_signing_keys(&mut self, signing_keys: &[&str]) -> Result<(), JsError> {
        self.tx_hex = sign_transaction(&self.tx_hex, signing_keys)?;
        Ok(())
    }

    pub fn add_required_signature(&mut self, pub_key_hash: &str) -> Result<(), JsError> {
        self.tx_builder
            .add_required_signer(&csl::Ed25519KeyHash::from_hex(pub_key_hash)?);
        Ok(())
    }

    pub fn add_metadata(&mut self, metadata: Metadata) -> Result<(), JsError> {
        self.tx_builder
            .add_json_metadatum(&csl::BigNum::from_str(&metadata.tag)?, metadata.metadata)?;
        Ok(())
    }

    pub fn add_script_hash(&mut self, network: Network) -> Result<(), JsError> {
        self.tx_builder
            .calc_script_data_hash(&build_csl_cost_models(&network))?;
        Ok(())
    }

    pub fn build_tx(&mut self) -> Result<String, JsError> {
        let tx = self.tx_builder.build_tx()?;
        self.tx_hex = tx.to_hex();
        Ok(self.tx_hex.to_string())
    }
}

impl Default for MeshCSL {
    fn default() -> Self {
        Self::new(None)
    }
}
