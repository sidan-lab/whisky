use crate::{csl, model::*};

use super::utils::{build_tx_builder, sign_transaction, to_bignum, to_value};

pub trait IMeshCSL {
    fn new() -> Self;
    fn add_tx_in(&mut self, input: PubKeyTxIn);
    fn add_script_tx_in(&mut self, input: ScriptTxIn);
    fn add_output(&mut self, output: Output);
    fn add_collateral(
        &mut self,
        collateral_builder: &mut csl::tx_builder::tx_inputs_builder::TxInputsBuilder,
        collateral: PubKeyTxIn,
    );
    fn add_reference_input(&mut self, ref_input: RefTxIn);
    fn add_plutus_mint(
        &mut self,
        mint_builder: &mut csl::tx_builder::mint_builder::MintBuilder,
        mint: MintItem,
        index: u64,
    );
    fn add_native_mint(
        &mut self,
        mint_builder: &mut csl::tx_builder::mint_builder::MintBuilder,
        mint: MintItem,
    );
    fn add_invalid_before(&mut self, invalid_before: u64);
    fn add_invalid_hereafter(&mut self, invalid_hereafter: u64);
    fn add_change(&mut self, change_address: String, change_datum: Option<Datum>);
    fn add_signing_keys(&mut self, signing_keys: JsVecString);
    fn add_required_signature(&mut self, pub_key_hash: String);
    fn add_metadata(&mut self, metadata: Metadata);
    fn add_script_hash(&mut self);
    fn build_tx(&mut self);
}

pub struct MeshCSL {
    pub tx_hex: String,
    pub tx_builder: csl::tx_builder::TransactionBuilder,
    pub tx_inputs_builder: csl::tx_builder::tx_inputs_builder::TxInputsBuilder,
}

impl IMeshCSL for MeshCSL {
    fn new() -> MeshCSL {
        MeshCSL {
            tx_hex: String::new(),
            tx_builder: build_tx_builder(),
            tx_inputs_builder: csl::tx_builder::tx_inputs_builder::TxInputsBuilder::new(),
        }
    }

    fn add_tx_in(&mut self, input: PubKeyTxIn) {
        self.tx_inputs_builder.add_input(
            &csl::address::Address::from_bech32(&input.tx_in.address.unwrap()).unwrap(),
            &csl::TransactionInput::new(
                &csl::crypto::TransactionHash::from_hex(&input.tx_in.tx_hash).unwrap(),
                input.tx_in.tx_index,
            ),
            &to_value(&input.tx_in.amount.unwrap()),
        );
    }

    fn add_script_tx_in(&mut self, input: ScriptTxIn) {
        let datum_source = input.script_tx_in.datum_source.unwrap();
        let script_source = input.script_tx_in.script_source.unwrap();
        let redeemer = input.script_tx_in.redeemer.unwrap();
        let csl_datum: csl::tx_builder::tx_inputs_builder::DatumSource = match datum_source {
            DatumSource::ProvidedDatumSource(datum) => {
                csl::tx_builder::tx_inputs_builder::DatumSource::new(
                    &csl::plutus::PlutusData::from_json(
                        &datum.data,
                        csl::plutus::PlutusDatumSchema::DetailedSchema,
                    )
                    .unwrap(),
                )
            }
            DatumSource::InlineDatumSource(datum) => {
                let ref_input = csl::TransactionInput::new(
                    &csl::crypto::TransactionHash::from_hex(&datum.tx_hash).unwrap(),
                    datum.tx_index,
                );
                csl::tx_builder::tx_inputs_builder::DatumSource::new_ref_input(&ref_input)
            }
        };

        let csl_script: csl::tx_builder::tx_inputs_builder::PlutusScriptSource = match script_source
        {
            ScriptSource::ProvidedScriptSource(script) => {
                let language_version: csl::plutus::Language = match script.language_version {
                    LanguageVersion::V1 => csl::plutus::Language::new_plutus_v1(),
                    LanguageVersion::V2 => csl::plutus::Language::new_plutus_v2(),
                };
                csl::tx_builder::tx_inputs_builder::PlutusScriptSource::new(
                    &csl::plutus::PlutusScript::from_hex_with_version(
                        &script.script_cbor,
                        &language_version,
                    )
                    .unwrap(),
                )
            }
            ScriptSource::InlineScriptSource(script) => {
                let language_version: csl::plutus::Language = match script.language_version {
                    LanguageVersion::V1 => csl::plutus::Language::new_plutus_v1(),
                    LanguageVersion::V2 => csl::plutus::Language::new_plutus_v2(),
                };
                csl::tx_builder::tx_inputs_builder::PlutusScriptSource::new_ref_input_with_lang_ver(
                    &csl::crypto::ScriptHash::from_hex(&script.spending_script_hash).unwrap(),
                    &csl::TransactionInput::new(
                        &csl::crypto::TransactionHash::from_hex(&script.tx_hash).unwrap(),
                        script.tx_index,
                    ),
                    &language_version,
                )
            }
        };

        let csl_redeemer: csl::plutus::Redeemer = csl::plutus::Redeemer::new(
            &csl::plutus::RedeemerTag::new_spend(),
            &to_bignum(0),
            &csl::plutus::PlutusData::from_json(
                &redeemer.data,
                csl::plutus::PlutusDatumSchema::DetailedSchema,
            )
            .unwrap(),
            &csl::plutus::ExUnits::new(
                &to_bignum(redeemer.ex_units.mem),
                &to_bignum(redeemer.ex_units.steps),
            ),
        );
        self.tx_inputs_builder.add_plutus_script_input(
            &csl::tx_builder::tx_inputs_builder::PlutusWitness::new_with_ref(
                &csl_script,
                &csl_datum,
                &csl_redeemer,
            ),
            &csl::TransactionInput::new(
                &csl::crypto::TransactionHash::from_hex(&input.tx_in.tx_hash).unwrap(),
                input.tx_in.tx_index,
            ),
            &to_value(&input.tx_in.amount.unwrap()),
        )
    }

    fn add_output(&mut self, output: Output) {
        let mut output_builder = csl::output_builder::TransactionOutputBuilder::new()
            .with_address(&csl::address::Address::from_bech32(&output.address).unwrap());
        if output.datum.is_some() {
            let datum = output.datum.unwrap();

            match datum.type_.as_str() {
                "Hash" => {
                    output_builder = output_builder.with_data_hash(&csl::utils::hash_plutus_data(
                        &csl::plutus::PlutusData::from_json(
                            &datum.data,
                            csl::plutus::PlutusDatumSchema::DetailedSchema,
                        )
                        .unwrap(),
                    ));
                }
                "Inline" => {
                    output_builder = output_builder.with_plutus_data(
                        &csl::plutus::PlutusData::from_json(
                            &datum.data,
                            csl::plutus::PlutusDatumSchema::DetailedSchema,
                        )
                        .unwrap(),
                    );
                }
                _ => {}
            };
        }

        if output.reference_script.is_some() {
            let output_script = output.reference_script.unwrap();
            let language_version: csl::plutus::Language = match output_script.language_version {
                LanguageVersion::V1 => csl::plutus::Language::new_plutus_v1(),
                LanguageVersion::V2 => csl::plutus::Language::new_plutus_v2(),
            };
            output_builder = output_builder.with_script_ref(&csl::ScriptRef::new_plutus_script(
                &csl::plutus::PlutusScript::from_hex_with_version(
                    &output_script.script_cbor,
                    &language_version,
                )
                .unwrap(),
            ))
        }

        let tx_value = to_value(&output.amount);
        let amount_builder = output_builder.next().unwrap();
        let built_output: csl::TransactionOutput = if tx_value.multiasset().is_some() {
            if tx_value.coin().is_zero() {
                amount_builder
                    .with_asset_and_min_required_coin_by_utxo_cost(
                        &tx_value.multiasset().unwrap(),
                        &csl::DataCost::new_coins_per_byte(&to_bignum(4310)),
                    )
                    .unwrap()
                    .build()
                    .unwrap()
            } else {
                amount_builder
                    .with_coin_and_asset(&tx_value.coin(), &tx_value.multiasset().unwrap())
                    .build()
                    .unwrap()
            }
        } else {
            amount_builder.with_coin(&tx_value.coin()).build().unwrap()
        };
        self.tx_builder.add_output(&built_output).unwrap();
    }

    fn add_collateral(
        &mut self,
        collateral_builder: &mut csl::tx_builder::tx_inputs_builder::TxInputsBuilder,
        collateral: PubKeyTxIn,
    ) {
        collateral_builder.add_input(
            &csl::address::Address::from_bech32(&collateral.tx_in.address.unwrap()).unwrap(),
            &csl::TransactionInput::new(
                &csl::crypto::TransactionHash::from_hex(&collateral.tx_in.tx_hash).unwrap(),
                collateral.tx_in.tx_index,
            ),
            &to_value(&collateral.tx_in.amount.unwrap()),
        );
    }
    fn add_reference_input(&mut self, ref_input: RefTxIn) {
        let csl_ref_input = csl::TransactionInput::new(
            &csl::crypto::TransactionHash::from_hex(&ref_input.tx_hash).unwrap(),
            ref_input.tx_index,
        );
        self.tx_builder.add_reference_input(&csl_ref_input);
    }

    fn add_plutus_mint(
        &mut self,
        mint_builder: &mut csl::tx_builder::mint_builder::MintBuilder,
        mint: MintItem,
        index: u64,
    ) {
        let redeemer_info = mint.redeemer.unwrap();
        let mint_redeemer = csl::plutus::Redeemer::new(
            &csl::plutus::RedeemerTag::new_mint(),
            &to_bignum(index),
            &csl::plutus::PlutusData::from_json(
                &redeemer_info.data,
                csl::plutus::PlutusDatumSchema::DetailedSchema,
            )
            .unwrap(),
            &csl::plutus::ExUnits::new(
                &to_bignum(redeemer_info.ex_units.mem),
                &to_bignum(redeemer_info.ex_units.steps),
            ),
        );
        let script_source_info = mint.script_source.unwrap();
        let mint_script = match script_source_info {
            ScriptSource::InlineScriptSource(script) => {
                let language_version: csl::plutus::Language = match script.language_version {
                    LanguageVersion::V1 => csl::plutus::Language::new_plutus_v1(),
                    LanguageVersion::V2 => csl::plutus::Language::new_plutus_v2(),
                };
                csl::tx_builder::tx_inputs_builder::PlutusScriptSource::new_ref_input_with_lang_ver(
                    &csl::crypto::ScriptHash::from_hex(mint.policy_id.as_str()).unwrap(),
                    &csl::TransactionInput::new(
                        &csl::crypto::TransactionHash::from_hex(&script.tx_hash).unwrap(),
                        script.tx_index,
                    ),
                    &language_version,
                )
            }
            ScriptSource::ProvidedScriptSource(script) => {
                let language_version: csl::plutus::Language = match script.language_version {
                    LanguageVersion::V1 => csl::plutus::Language::new_plutus_v1(),
                    LanguageVersion::V2 => csl::plutus::Language::new_plutus_v2(),
                };
                csl::tx_builder::tx_inputs_builder::PlutusScriptSource::new(
                    &csl::plutus::PlutusScript::from_hex_with_version(
                        script.script_cbor.as_str(),
                        &language_version,
                    )
                    .unwrap(),
                )
            }
        };

        mint_builder.add_asset(
            &csl::tx_builder::mint_builder::MintWitness::new_plutus_script(
                &mint_script,
                &mint_redeemer,
            ),
            &csl::AssetName::new(hex::decode(mint.asset_name).unwrap()).unwrap(),
            &csl::utils::Int::new_i32(mint.amount.try_into().unwrap()),
        );
    }

    fn add_native_mint(
        &mut self,
        mint_builder: &mut csl::tx_builder::mint_builder::MintBuilder,
        mint: MintItem,
    ) {
        let script_info = mint.script_source.unwrap();
        match script_info {
            ScriptSource::ProvidedScriptSource(script) => mint_builder.add_asset(
                &csl::tx_builder::mint_builder::MintWitness::new_native_script(
                    &csl::NativeScript::from_hex(&script.script_cbor).unwrap(),
                ),
                &csl::AssetName::new(hex::decode(mint.asset_name).unwrap()).unwrap(),
                &csl::utils::Int::new_i32(mint.amount.try_into().unwrap()),
            ),
            ScriptSource::InlineScriptSource(_) => {
                panic!("Native scripts cannot be referenced")
            }
        }
    }

    fn add_invalid_before(&mut self, invalid_before: u64) {
        self.tx_builder
            .set_validity_start_interval_bignum(to_bignum(invalid_before));
    }

    fn add_invalid_hereafter(&mut self, invalid_hereafter: u64) {
        self.tx_builder
            .set_ttl_bignum(&to_bignum(invalid_hereafter));
    }
    fn add_change(&mut self, change_address: String, change_datum: Option<Datum>) {
        if let Some(change_datum) = change_datum {
            self.tx_builder
                .add_change_if_needed_with_datum(
                    &csl::address::Address::from_bech32(&change_address).unwrap(),
                    &csl::OutputDatum::new_data(
                        &csl::plutus::PlutusData::from_json(
                            &change_datum.data,
                            csl::plutus::PlutusDatumSchema::DetailedSchema,
                        )
                        .unwrap(),
                    ),
                )
                .unwrap();
        } else {
            self.tx_builder
                .add_change_if_needed(&csl::address::Address::from_bech32(&change_address).unwrap())
                .unwrap();
        }
    }

    fn add_signing_keys(&mut self, signing_keys: JsVecString) {
        self.tx_hex = sign_transaction(self.tx_hex.to_string(), signing_keys);
    }

    fn add_required_signature(&mut self, pub_key_hash: String) {
        self.tx_builder
            .add_required_signer(&csl::crypto::Ed25519KeyHash::from_hex(&pub_key_hash).unwrap())
    }

    fn add_metadata(&mut self, metadata: Metadata) {
        self.tx_builder
            .add_json_metadatum(
                &csl::utils::BigNum::from_str(&metadata.tag).unwrap(),
                metadata.metadata,
            )
            .unwrap()
    }

    fn add_script_hash(&mut self) {
        self.tx_builder
            .calc_script_data_hash(
                &csl::tx_builder_constants::TxBuilderConstants::plutus_vasil_cost_models(),
            )
            .unwrap();
    }

    fn build_tx(&mut self) {
        let tx = self.tx_builder.build_tx().unwrap();
        self.tx_hex = tx.to_hex();
    }
}

impl Default for MeshCSL {
    fn default() -> Self {
        Self::new()
    }
}
