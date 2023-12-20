use cardano_serialization_lib as csl;

use crate::{
    builder::models::*,
    utils::csl::{build_tx_builder, to_bignum, to_value},
};

pub struct MeshTxBuilderCore {
    tx_builder: csl::tx_builder::TransactionBuilder,
    tx_inputs_builder: csl::tx_builder::tx_inputs_builder::TxInputsBuilder,
    mesh_tx_builder_body: MeshTxBuilderBody,
}

impl MeshTxBuilderCore {
    pub fn new() -> Self {
        Self {
            tx_builder: build_tx_builder(),
            tx_inputs_builder: csl::tx_builder::tx_inputs_builder::TxInputsBuilder::new(),
            mesh_tx_builder_body: MeshTxBuilderBody {
                inputs: vec![],
                outputs: vec![],
                collaterals: vec![],
                required_signatures: vec![],
                reference_inputs: vec![],
                mints: vec![],
                change_address: String::from(""),
                metadata: vec![],
                validity_range: ValidityRange {
                    invalid_before: None,
                    invalid_hereafter: None,
                },
                signing_key: vec![],
            },
        }
    }

    pub fn complete_sync(
        &mut self,
        customized_tx: Option<MeshTxBuilderBody>,
    ) -> &mut MeshTxBuilderCore {
        if customized_tx.is_some() {
            self.mesh_tx_builder_body = customized_tx.unwrap();
        }
        return self.serialize_tx_body();
    }

    pub fn serialize_tx_body(&mut self) -> &mut MeshTxBuilderCore {
        self.mesh_tx_builder_body
            .mints
            .sort_by(|a, b| a.policy_id.cmp(&b.policy_id));

        self.mesh_tx_builder_body.inputs.sort_by(|a, b| {
            let tx_in_data_a: &TxInParameter = match a {
                TxIn::PubKeyTxIn(pub_key_tx_in) => &pub_key_tx_in.tx_in,
                TxIn::ScriptTxIn(script_tx_in) => &script_tx_in.tx_in,
            };

            let tx_in_data_b: &TxInParameter = match b {
                TxIn::PubKeyTxIn(pub_key_tx_in) => &pub_key_tx_in.tx_in,
                TxIn::ScriptTxIn(script_tx_in) => &script_tx_in.tx_in,
            };

            tx_in_data_a
                .tx_hash
                .cmp(&tx_in_data_b.tx_hash)
                .then_with(|| tx_in_data_a.tx_index.cmp(&tx_in_data_b.tx_index))
        });
        self.add_all_inputs(self.mesh_tx_builder_body.inputs.clone());
        self.add_all_outputs(self.mesh_tx_builder_body.outputs.clone());
        self.add_all_collaterals(self.mesh_tx_builder_body.collaterals.clone());
        self.add_all_reference_inputs(self.mesh_tx_builder_body.reference_inputs.clone());
        self
    }

    fn add_all_inputs(&mut self, inputs: Vec<TxIn>) {
        for input in inputs {
            match input {
                TxIn::PubKeyTxIn(pub_key_tx_in) => self.add_tx_in(pub_key_tx_in),
                TxIn::ScriptTxIn(script_tx_in) => self.add_script_tx_in(script_tx_in),
            };
        }
        self.tx_builder.set_inputs(&self.tx_inputs_builder);
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
        let csl_datum: csl::tx_builder::tx_inputs_builder::DatumSource;

        match datum_source {
            DatumSource::ProvidedDatumSource(datum) => {
                csl_datum = csl::tx_builder::tx_inputs_builder::DatumSource::new(
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
                csl_datum =
                    csl::tx_builder::tx_inputs_builder::DatumSource::new_ref_input(&ref_input)
            }
        };

        let csl_script: csl::tx_builder::tx_inputs_builder::PlutusScriptSource;
        match script_source {
            ScriptSource::ProvidedScriptSource(script) => {
                let language_version: csl::plutus::Language = match script.language_version {
                    LanguageVersion::V1 => csl::plutus::Language::new_plutus_v1(),
                    LanguageVersion::V2 => csl::plutus::Language::new_plutus_v2(),
                };
                csl_script = csl::tx_builder::tx_inputs_builder::PlutusScriptSource::new(
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
                csl_script =
                csl::tx_builder::tx_inputs_builder::PlutusScriptSource::new_ref_input_with_lang_ver(
                    &csl::crypto::ScriptHash::from_hex(&script.spending_script_hash).unwrap(),
                    &csl::TransactionInput::new(
                        &csl::crypto::TransactionHash::from_hex(&script.tx_hash).unwrap(),
                        script.tx_index,
                    ),
                    &language_version
                );
            }
        }

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

    fn add_all_outputs(&mut self, outputs: Vec<Output>) {
        for output in outputs {
            self.add_output(output);
        }
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
                    ))
                }
                "Inline" => {
                    output_builder = output_builder.with_plutus_data(
                        &csl::plutus::PlutusData::from_json(
                            &datum.data,
                            csl::plutus::PlutusDatumSchema::DetailedSchema,
                        )
                        .unwrap(),
                    )
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
        let built_output: csl::TransactionOutput;

        if tx_value.multiasset().is_some() {
            built_output = if tx_value.coin().is_zero() {
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
            };
        } else {
            built_output = amount_builder.with_coin(&tx_value.coin()).build().unwrap();
        }
        let _ = self.tx_builder.add_output(&built_output);
    }

    fn add_all_collaterals(&mut self, collaterals: Vec<PubKeyTxIn>) {
        let mut collateral_builder = csl::tx_builder::tx_inputs_builder::TxInputsBuilder::new();
        for collateral in collaterals {
            self.add_collateral(&mut collateral_builder, collateral)
        }
        self.tx_builder.set_collateral(&collateral_builder)
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
        )
    }

    fn add_all_reference_inputs(&mut self, ref_inputs: Vec<RefTxIn>) {
        for ref_input in ref_inputs {
            self.add_reference_input(ref_input);
        }
    }

    fn add_reference_input(&mut self, ref_input: RefTxIn) {
        let csl_ref_input = csl::TransactionInput::new(
            &csl::crypto::TransactionHash::from_hex(&ref_input.tx_hash).unwrap(),
            ref_input.tx_index,
        );
        self.tx_builder.add_reference_input(&csl_ref_input);
    }
}
