use cardano_serialization_lib as csl;

use crate::{
    builder::models::*,
    utils::csl::{build_tx_builder, to_bignum, to_value},
};

pub struct MeshTxBuilderCore {
    pub tx_hex: String,
    pub tx_builder: csl::tx_builder::TransactionBuilder,
    pub tx_inputs_builder: csl::tx_builder::tx_inputs_builder::TxInputsBuilder,
    pub mesh_tx_builder_body: MeshTxBuilderBody,

    tx_in_item: Option<TxIn>,
    mint_item: Option<MintItem>,
    collateral_item: Option<PubKeyTxIn>,
    tx_output: Option<Output>,
    adding_script_input: bool,
    adding_plutus_mint: bool,
}

impl MeshTxBuilderCore {
    pub fn new() -> Self {
        Self {
            tx_hex: "".to_string(),
            tx_builder: build_tx_builder(),
            tx_inputs_builder: csl::tx_builder::tx_inputs_builder::TxInputsBuilder::new(),
            mesh_tx_builder_body: MeshTxBuilderBody {
                inputs: vec![],
                outputs: vec![],
                collaterals: vec![],
                required_signatures: vec![],
                reference_inputs: vec![],
                mints: vec![],
                change_address: "".to_string(),
                metadata: vec![],
                validity_range: ValidityRange {
                    invalid_before: None,
                    invalid_hereafter: None,
                },
                signing_key: vec![],
            },
            tx_in_item: None,
            mint_item: None,
            collateral_item: None,
            tx_output: None,
            adding_script_input: false,
            adding_plutus_mint: false,
        }
    }

    pub fn complete_sync(
        &mut self,
        customized_tx: Option<MeshTxBuilderBody>,
    ) -> &mut MeshTxBuilderCore {
        if customized_tx.is_some() {
            self.mesh_tx_builder_body = customized_tx.unwrap();
        }
        self.serialize_tx_body()
    }

    pub fn complete_signing(&mut self) -> String {
        let signing_keys = self.mesh_tx_builder_body.signing_key.clone();
        self.add_all_signing_keys(signing_keys);
        self.tx_hex.to_string()
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
        self.add_all_mints(self.mesh_tx_builder_body.mints.clone());
        self.add_validity_range(self.mesh_tx_builder_body.validity_range.clone());
        self.add_all_required_signature(self.mesh_tx_builder_body.required_signatures.clone());
        self.add_all_metadata(self.mesh_tx_builder_body.metadata.clone());

        self.add_script_hash();
        if self.mesh_tx_builder_body.change_address != "" {
            let collateral_inputs = self.mesh_tx_builder_body.collaterals.clone();
            let collateral_vec: Vec<u64> = collateral_inputs
                .into_iter()
                .map(|pub_key_tx_in| {
                    let assets = pub_key_tx_in.tx_in.amount.unwrap();
                    let lovelace = assets
                        .into_iter()
                        .find(|asset| asset.unit == "lovelace")
                        .unwrap();
                    lovelace.quantity.parse::<u64>().unwrap()
                })
                .collect();
            let total_collateral: u64 = collateral_vec.into_iter().sum();

            let collateral_estimate: u64 = (150
                * self
                    .tx_builder
                    .min_fee()
                    .unwrap()
                    .checked_add(&to_bignum(10000))
                    .unwrap()
                    .to_string()
                    .parse::<u64>()
                    .unwrap())
                / 100;

            let mut collateral_return_needed = false;

            if total_collateral - collateral_estimate > 0 {
                let collateral_estimate_output = csl::TransactionOutput::new(
                    &csl::address::Address::from_bech32(&self.mesh_tx_builder_body.change_address)
                        .unwrap(),
                    &csl::utils::Value::new(&to_bignum(collateral_estimate)),
                );

                let min_ada = csl::utils::min_ada_for_output(
                    &collateral_estimate_output,
                    &csl::DataCost::new_coins_per_byte(&to_bignum(4310)),
                )
                .unwrap()
                .to_string()
                .parse::<u64>()
                .unwrap();

                if total_collateral - collateral_estimate > min_ada {
                    self.tx_builder
                        .set_collateral_return(&csl::TransactionOutput::new(
                            &csl::address::Address::from_bech32(
                                &self.mesh_tx_builder_body.change_address,
                            )
                            .unwrap(),
                            &csl::utils::Value::new(&to_bignum(total_collateral)),
                        ));

                    self.tx_builder
                        .set_total_collateral(&to_bignum(total_collateral));

                    collateral_return_needed = true;
                }
            }
            self.add_change(self.mesh_tx_builder_body.change_address.clone());
            if collateral_return_needed {
                self.add_collateral_return(self.mesh_tx_builder_body.change_address.clone());
            }
        }
        self.build_tx();
        self
    }

    pub fn tx_in(
        &mut self,
        tx_hash: String,
        tx_index: u32,
        amount: Vec<Asset>,
        address: String,
    ) -> &mut MeshTxBuilderCore {
        if self.tx_in_item.is_some() {
            self.queue_input();
        }
        if !self.adding_script_input {
            let item = TxIn::PubKeyTxIn(PubKeyTxIn {
                type_: "PubKey".to_string(),
                tx_in: TxInParameter {
                    tx_hash,
                    tx_index,
                    amount: Some(amount),
                    address: Some(address),
                },
            });
            self.tx_in_item = Some(item);
        } else {
            let item = TxIn::ScriptTxIn(ScriptTxIn {
                type_: "Script".to_string(),
                tx_in: TxInParameter {
                    tx_hash,
                    tx_index,
                    amount: Some(amount),
                    address: Some(address),
                },
                script_tx_in: ScriptTxInParameter {
                    script_source: None,
                    datum_source: None,
                    redeemer: None,
                },
            });
            self.tx_in_item = Some(item);
        }
        self
    }

    fn queue_input(&mut self) {
        let tx_in_item = self.tx_in_item.clone().unwrap();
        match tx_in_item {
            TxIn::ScriptTxIn(tx_in) => {
                match (
                    tx_in.script_tx_in.datum_source,
                    tx_in.script_tx_in.redeemer,
                    tx_in.script_tx_in.script_source,
                ) {
                    (None, _, _) => panic!("Datum in a script input cannot be None"),
                    (_, None, _) => panic!("Redeemer in script input cannot be None"),
                    (_, _, None) => panic!("Script source in script input cannot be None"),
                    _ => {}
                }
            }
            TxIn::PubKeyTxIn(_) => {}
        }
        self.mesh_tx_builder_body
            .inputs
            .push(self.tx_in_item.clone().unwrap());
        self.tx_in_item = None
    }

    fn queue_mint(&mut self) {
        let mint_item = self.mint_item.clone().unwrap();
        if mint_item.script_source.is_none() {
            panic!("Missing mint script information");
        }
        self.mesh_tx_builder_body.mints.push(mint_item);
        self.mint_item = None;
    }

    pub fn tx_in_script(
        &mut self,
        script_cbor: String,
        version: LanguageVersion,
    ) -> &mut MeshTxBuilderCore {
        let tx_in_item = self.tx_in_item.take();
        if tx_in_item.is_none() {
            panic!("Undefined input")
        }
        let tx_in_item = tx_in_item.unwrap();
        match tx_in_item {
            TxIn::PubKeyTxIn(_) => panic!("Script cannot be defined for a pubkey tx in"),
            TxIn::ScriptTxIn(mut input) => {
                input.script_tx_in.script_source =
                    Some(ScriptSource::ProvidedScriptSource(ProvidedScriptSource {
                        script_cbor,
                        language_version: version,
                    }));
                self.tx_in_item = Some(TxIn::ScriptTxIn(input));
            }
        }
        self
    }

    pub fn tx_in_datum_value(&mut self, data: String) -> &mut MeshTxBuilderCore {
        let tx_in_item = self.tx_in_item.take();
        if tx_in_item.is_none() {
            panic!("Undefined input")
        }
        let tx_in_item = tx_in_item.unwrap();
        match tx_in_item {
            TxIn::PubKeyTxIn(_) => panic!("Datum cannot be defined for a pubkey tx in"),
            TxIn::ScriptTxIn(mut input) => {
                input.script_tx_in.datum_source =
                    Some(DatumSource::ProvidedDatumSource(ProvidedDatumSource {
                        data,
                    }));
                self.tx_in_item = Some(TxIn::ScriptTxIn(input));
            }
        }
        self
    }

    pub fn tx_in_inline_datum_present(&mut self) -> &mut MeshTxBuilderCore {
        let tx_in_item = self.tx_in_item.take();
        if tx_in_item.is_none() {
            panic!("Undefined input")
        }
        let tx_in_item = tx_in_item.unwrap();
        match tx_in_item {
            TxIn::PubKeyTxIn(_) => panic!("Datum cannot be defined for a pubkey tx in"),
            TxIn::ScriptTxIn(mut input) => {
                input.script_tx_in.datum_source =
                    Some(DatumSource::InlineDatumSource(InlineDatumSource {
                        tx_hash: input.tx_in.tx_hash.clone(),
                        tx_index: input.tx_in.tx_index.clone(),
                    }));
                self.tx_in_item = Some(TxIn::ScriptTxIn(input));
            }
        }
        self
    }

    pub fn tx_in_redeemer_value(&mut self, redeemer: Redeemer) -> &mut MeshTxBuilderCore {
        let tx_in_item = self.tx_in_item.take();
        if tx_in_item.is_none() {
            panic!("Undefined input")
        }
        let tx_in_item = tx_in_item.unwrap();
        match tx_in_item {
            TxIn::PubKeyTxIn(_) => panic!("Redeemer cannot be defined for a pubkey tx in"),
            TxIn::ScriptTxIn(mut input) => {
                input.script_tx_in.redeemer = Some(redeemer);
                self.tx_in_item = Some(TxIn::ScriptTxIn(input));
            }
        }
        self
    }

    pub fn tx_out(&mut self, address: String, amount: Vec<Asset>) -> &mut MeshTxBuilderCore {
        if self.tx_output.is_some() {
            let tx_output = self.tx_output.take();
            self.mesh_tx_builder_body.outputs.push(tx_output.unwrap());
        }
        self.tx_output = Some(Output {
            address,
            amount,
            datum: None,
            reference_script: None,
        });
        self
    }

    pub fn tx_out_datum_hash_value(&mut self, data: String) -> &mut MeshTxBuilderCore {
        let tx_output = self.tx_output.take();
        if tx_output.is_none() {
            panic!("Undefined output")
        }
        let mut tx_output = tx_output.unwrap();
        tx_output.datum = Some(Datum {
            type_: "Hash".to_string(),
            data
        });
        self
    }

    pub fn tx_out_reference_script(
        &mut self,
        script_cbor: String,
        version: LanguageVersion,
    ) -> &mut MeshTxBuilderCore {
        let tx_output = self.tx_output.take();
        if tx_output.is_none() {
            panic!("Undefined output")
        }
        let mut tx_output = tx_output.unwrap();
        tx_output.reference_script = Some(ProvidedScriptSource {
            script_cbor,
            language_version: version,
        });
        self.tx_output = Some(tx_output);
        self
    }

    pub fn spending_plutus_script_v2(&mut self) -> &mut MeshTxBuilderCore {
        self.adding_script_input = true;
        self
    }

    pub fn spending_tx_in_reference(
        &mut self,
        tx_hash: String,
        tx_index: u32,
        spending_script_hash: String,
        version: LanguageVersion,
    ) -> &mut MeshTxBuilderCore {
        let tx_in_item = self.tx_in_item.take();
        if tx_in_item.is_none() {
            panic!("Undefined output")
        }
        let tx_in_item = tx_in_item.unwrap();
        match tx_in_item {
            TxIn::PubKeyTxIn(_) => panic!("Script reference cannot be defined for a pubkey tx in"),
            TxIn::ScriptTxIn(mut input) => {
                input.script_tx_in.script_source =
                    Some(ScriptSource::InlineScriptSource(InlineScriptSource {
                        tx_hash,
                        tx_index,
                        spending_script_hash,
                        language_version: version,
                    }));
                self.tx_in_item = Some(TxIn::ScriptTxIn(input));
            }
        }
        self
    }

    pub fn spending_reference_tx_in_inline_datum_present(&mut self) -> &mut MeshTxBuilderCore {
        self.tx_in_inline_datum_present()
    }

    pub fn spending_reference_tx_in_redeemer_value(
        &mut self,
        redeemer: Redeemer,
    ) -> &mut MeshTxBuilderCore {
        self.tx_in_redeemer_value(redeemer)
    }

    pub fn read_only_tx_in_reference(
        &mut self,
        tx_hash: String,
        tx_index: u32,
    ) -> &mut MeshTxBuilderCore {
        self.mesh_tx_builder_body
            .reference_inputs
            .push(RefTxIn { tx_hash, tx_index });
        self
    }

    pub fn mint_plutus_script_v2(&mut self) -> &mut MeshTxBuilderCore {
        self.adding_plutus_mint = true;
        self
    }

    pub fn mint(&mut self, quantity: u64, policy: String, name: String) -> &mut MeshTxBuilderCore {
        if self.mint_item.is_some() {
            self.queue_mint();
        }
        let mint_type = if self.adding_plutus_mint {
            "Plutus"
        } else {
            "Native"
        };
        self.mint_item = Some(MintItem {
            type_: mint_type.to_string(),
            policy_id: policy,
            asset_name: name,
            amount: quantity,
            redeemer: None,
            script_source: None,
        });
        self.adding_plutus_mint = false;
        self
    }

    pub fn minting_script(
        &mut self,
        script_cbor: String,
        version: LanguageVersion,
    ) -> &mut MeshTxBuilderCore {
        let mint_item = self.mint_item.take();
        if mint_item.is_none() {
            panic!("Undefined mint");
        }
        let mut mint_item = mint_item.unwrap();
        mint_item.script_source = Some(ScriptSource::ProvidedScriptSource(ProvidedScriptSource {
            script_cbor,
            language_version: version,
        }));
        self.mint_item = Some(mint_item);
        self
    }

    pub fn mint_tx_in_reference(
        &mut self,
        tx_hash: String,
        tx_index: u32,
        spending_script_hash: String,
        version: LanguageVersion,
    ) -> &mut MeshTxBuilderCore {
        let mint_item = self.mint_item.take();
        if mint_item.is_none() {
            panic!("Undefined mint");
        }
        let mut mint_item = mint_item.unwrap();
        mint_item.script_source = Some(ScriptSource::InlineScriptSource(InlineScriptSource {
            tx_hash,
            tx_index,
            spending_script_hash,
            language_version: version,
        }));
        self.mint_item = Some(mint_item);
        self
    }

    pub fn mint_redeemer_value(&mut self, redeemer: Redeemer) -> &mut MeshTxBuilderCore {
        let mint_item = self.mint_item.take();
        if mint_item.is_none() {
            panic!("Undefined mint");
        }
        let mut mint_item = mint_item.unwrap();
        if mint_item.type_ == "Native" {
            panic!("Redeemer cannot be defined for Native script mints");
        }
        mint_item.redeemer = Some(redeemer);
        self.mint_item = Some(mint_item);
        self
    }

    pub fn mint_reference_tx_in_redeemer_value(
        &mut self,
        redeemer: Redeemer,
    ) -> &mut MeshTxBuilderCore {
        self.mint_redeemer_value(redeemer)
    }

    pub fn required_signer_hash(&mut self, pub_key_hash: String) -> &mut MeshTxBuilderCore {
        self.mesh_tx_builder_body
            .required_signatures
            .push(pub_key_hash);
        self
    }

    pub fn tx_in_collateral(
        &mut self,
        tx_hash: String,
        tx_index: u32,
        amount: Vec<Asset>,
        address: String,
    ) -> &mut MeshTxBuilderCore {
        let collateral_item = self.collateral_item.take();
        if self.collateral_item.is_some() {
            self.mesh_tx_builder_body
                .collaterals
                .push(collateral_item.unwrap());
        }
        self.collateral_item = Some(PubKeyTxIn {
            type_: "PubKey".to_string(),
            tx_in: TxInParameter {
                tx_hash,
                tx_index,
                amount: Some(amount),
                address: Some(address),
            },
        });
        self
    }

    pub fn change_address(&mut self, address: String) -> &mut MeshTxBuilderCore {
        self.mesh_tx_builder_body.change_address = address;
        self
    }

    pub fn invalid_before(&mut self, slot: u64) -> &mut MeshTxBuilderCore {
        self.mesh_tx_builder_body.validity_range.invalid_before = Some(slot);
        self
    }

    pub fn invalid_hereafter(&mut self, slot: u64) -> &mut MeshTxBuilderCore {
        self.mesh_tx_builder_body.validity_range.invalid_hereafter = Some(slot);
        self
    }

    pub fn metadata_value(&mut self, tag: String, metadata: String) -> &mut MeshTxBuilderCore {
        self.mesh_tx_builder_body
            .metadata
            .push(Metadata { tag, metadata });
        self
    }

    pub fn signing_key(&mut self, skey_hex: String) -> &mut MeshTxBuilderCore {
        self.mesh_tx_builder_body.signing_key.push(skey_hex);
        self
    }

    fn add_all_signing_keys(&mut self, signing_keys: Vec<String>) {
        if signing_keys.len() > 0 {
            let mut vkey_witnesses = csl::crypto::Vkeywitnesses::new();
            let unsigned_transaction: csl::Transaction =
                csl::Transaction::from_hex(&self.tx_hex).unwrap();
            let tx_body = unsigned_transaction.body();
            for key in signing_keys {
                let clean_hex = if &key[0..4] == "5820" {
                    key[4..].to_string()
                } else {
                    key
                };
                let skey = csl::crypto::PrivateKey::from_hex(&clean_hex).unwrap();
                let vkey_witness =
                    csl::utils::make_vkey_witness(&csl::utils::hash_transaction(&tx_body), &skey);
                vkey_witnesses.add(&vkey_witness);
            }
            let mut witness_set = unsigned_transaction.witness_set();
            witness_set.set_vkeys(&vkey_witnesses);
            let signed_transaction = csl::Transaction::new(
                &tx_body,
                &witness_set,
                unsigned_transaction.auxiliary_data(),
            );
            self.tx_hex = signed_transaction.to_hex();
        }
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
        );
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

    fn add_all_mints(&mut self, mints: Vec<MintItem>) {
        let mut mint_builder = csl::tx_builder::mint_builder::MintBuilder::new();
        for (index, mint) in mints.into_iter().enumerate() {
            match mint.type_.as_str() {
                "Plutus" => self.add_plutus_mint(&mut mint_builder, mint, index as u64),
                "Native" => self.add_native_mint(&mut mint_builder, mint),
                _ => {}
            };
        }
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
                    &csl::crypto::ScriptHash::from_hex(&mint.policy_id.as_str()).unwrap(),
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
                        &script.script_cbor.as_str(),
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

    fn add_validity_range(&mut self, validity_range: ValidityRange) {
        if validity_range.invalid_before.is_some() {
            self.tx_builder
                .set_validity_start_interval_bignum(to_bignum(
                    validity_range.invalid_before.unwrap(),
                ));
        }
        if validity_range.invalid_hereafter.is_some() {
            self.tx_builder
                .set_ttl_bignum(&to_bignum(validity_range.invalid_hereafter.unwrap()));
        }
    }

    fn add_all_required_signature(&mut self, required_signatures: Vec<String>) {
        for pub_key_hash in required_signatures {
            self.tx_builder
                .add_required_signer(&csl::crypto::Ed25519KeyHash::from_hex(&pub_key_hash).unwrap())
        }
    }

    fn add_all_metadata(&mut self, all_metadata: Vec<Metadata>) {
        for metadata in all_metadata {
            self.tx_builder
                .add_json_metadatum(
                    &csl::utils::BigNum::from_str(&metadata.tag).unwrap(),
                    metadata.metadata,
                )
                .unwrap()
        }
    }

    fn add_script_hash(&mut self) {
        let _ = self.tx_builder.calc_script_data_hash(
            &csl::tx_builder_constants::TxBuilderConstants::plutus_vasil_cost_models(),
        );
    }

    fn add_change(&mut self, change_address: String) {
        let _ = self
            .tx_builder
            .add_change_if_needed(&csl::address::Address::from_bech32(&change_address).unwrap());
    }

    fn add_collateral_return(&mut self, change_address: String) {
        let current_fee = self
            .tx_builder
            .get_fee_if_set()
            .unwrap()
            .to_string()
            .parse::<u64>()
            .unwrap();

        let collateral_amount = 150 * ((current_fee / 100) + 1);
        let _ = self.tx_builder.set_total_collateral_and_return(
            &to_bignum(collateral_amount),
            &csl::address::Address::from_bech32(&change_address).unwrap(),
        );
    }

    fn build_tx(&mut self) {
        let tx = self.tx_builder.build_tx().unwrap();
        self.tx_hex = tx.to_hex();
    }
}
