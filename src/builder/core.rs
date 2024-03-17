use cardano_serialization_lib as csl;

use crate::{
    csl::builder::{IMeshCSL, MeshCSL},
    model::builder::*,
};

use super::interface::{IMeshTxBuilderCore, MeshTxBuilderCore};

impl IMeshTxBuilderCore for MeshTxBuilderCore {
    fn new() -> Self {
        Self {
            mesh_csl: MeshCSL::new(),
            mesh_tx_builder_body: MeshTxBuilderBody {
                inputs: vec![],
                outputs: vec![],
                collaterals: vec![],
                required_signatures: vec![],
                reference_inputs: vec![],
                mints: vec![],
                change_address: "".to_string(),
                change_datum: None,
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

    fn complete_sync(
        &mut self,
        customized_tx: Option<MeshTxBuilderBody>,
    ) -> &mut MeshTxBuilderCore {
        if customized_tx.is_some() {
            self.mesh_tx_builder_body = customized_tx.unwrap();
        } else {
            self.queue_all_last_item();
        }
        self.serialize_tx_body()
    }

    fn complete_signing(&mut self) -> String {
        let signing_keys = self.mesh_tx_builder_body.signing_key.clone();
        self.add_all_signing_keys(signing_keys);
        self.mesh_csl.tx_hex.to_string()
    }

    fn serialize_tx_body(&mut self) -> &mut MeshTxBuilderCore {
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

        self.mesh_csl.add_script_hash();
        // if self.mesh_tx_builder_body.change_address != "" {
        //     let collateral_inputs = self.mesh_tx_builder_body.collaterals.clone();
        //     let collateral_vec: Vec<u64> = collateral_inputs
        //         .into_iter()
        //         .map(|pub_key_tx_in| {
        //             let assets = pub_key_tx_in.tx_in.amount.unwrap();
        //             let lovelace = assets
        //                 .into_iter()
        //                 .find(|asset| asset.unit == "lovelace")
        //                 .unwrap();
        //             lovelace.quantity.parse::<u64>().unwrap()
        //         })
        //         .collect();
        //     let total_collateral: u64 = collateral_vec.into_iter().sum();

        //     let collateral_estimate: u64 = (150
        //         * self
        //             .tx_builder
        //             .min_fee()
        //             .unwrap()
        //             .checked_add(&to_bignum(10000))
        //             .unwrap()
        //             .to_string()
        //             .parse::<u64>()
        //             .unwrap())
        //         / 100;

        //     let mut collateral_return_needed = false;
        // if (total_collateral - collateral_estimate) > 0 {
        // let collateral_estimate_output = csl::TransactionOutput::new(
        //     &csl::address::Address::from_bech32(&self.mesh_tx_builder_body.change_address)
        //         .unwrap(),
        //     &csl::utils::Value::new(&to_bignum(collateral_estimate)),
        // );

        // let min_ada = csl::utils::min_ada_for_output(
        //     &collateral_estimate_output,
        //     &csl::DataCost::new_coins_per_byte(&to_bignum(4310)),
        // )
        // .unwrap()
        // .to_string()
        // .parse::<u64>()
        // .unwrap();

        // if total_collateral - collateral_estimate > min_ada {
        //     self.tx_builder
        //         .set_collateral_return(&csl::TransactionOutput::new(
        //             &csl::address::Address::from_bech32(
        //                 &self.mesh_tx_builder_body.change_address,
        //             )
        //             .unwrap(),
        //             &csl::utils::Value::new(&to_bignum(total_collateral)),
        //         ));

        //     self.tx_builder
        //         .set_total_collateral(&to_bignum(total_collateral));

        //     collateral_return_needed = true;
        // }
        // }
        // self.add_change(self.mesh_tx_builder_body.change_address.clone());
        // if collateral_return_needed {
        //     self.add_collateral_return(self.mesh_tx_builder_body.change_address.clone());
        // }
        // }
        self.mesh_csl.add_change(
            self.mesh_tx_builder_body.change_address.clone(),
            self.mesh_tx_builder_body.change_datum.clone(),
        );
        self.mesh_csl.build_tx();
        self
    }

    fn tx_in(
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

    fn tx_in_script(
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

    fn tx_in_datum_value(&mut self, data: String) -> &mut MeshTxBuilderCore {
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

    fn tx_in_inline_datum_present(&mut self) -> &mut MeshTxBuilderCore {
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
                        tx_index: input.tx_in.tx_index,
                    }));
                self.tx_in_item = Some(TxIn::ScriptTxIn(input));
            }
        }
        self
    }

    fn tx_in_redeemer_value(&mut self, redeemer: Redeemer) -> &mut MeshTxBuilderCore {
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

    fn tx_out(&mut self, address: String, amount: Vec<Asset>) -> &mut MeshTxBuilderCore {
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

    fn tx_out_datum_hash_value(&mut self, data: String) -> &mut MeshTxBuilderCore {
        let tx_output = self.tx_output.take();
        if tx_output.is_none() {
            panic!("Undefined output")
        }
        let mut tx_output = tx_output.unwrap();
        tx_output.datum = Some(Datum {
            type_: "Hash".to_string(),
            data,
        });
        self.tx_output = Some(tx_output);
        self
    }

    fn tx_out_inline_datum_value(&mut self, data: String) -> &mut MeshTxBuilderCore {
        let tx_output = self.tx_output.take();
        if tx_output.is_none() {
            panic!("Undefined output")
        }
        let mut tx_output = tx_output.unwrap();
        tx_output.datum = Some(Datum {
            type_: "Inline".to_string(),
            data,
        });
        self.tx_output = Some(tx_output);
        self
    }

    fn tx_out_reference_script(
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

    fn spending_plutus_script_v2(&mut self) -> &mut MeshTxBuilderCore {
        self.adding_script_input = true;
        self
    }

    fn spending_tx_in_reference(
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

    fn spending_reference_tx_in_inline_datum_present(&mut self) -> &mut MeshTxBuilderCore {
        self.tx_in_inline_datum_present()
    }

    fn spending_reference_tx_in_redeemer_value(
        &mut self,
        redeemer: Redeemer,
    ) -> &mut MeshTxBuilderCore {
        self.tx_in_redeemer_value(redeemer)
    }

    fn read_only_tx_in_reference(
        &mut self,
        tx_hash: String,
        tx_index: u32,
    ) -> &mut MeshTxBuilderCore {
        self.mesh_tx_builder_body
            .reference_inputs
            .push(RefTxIn { tx_hash, tx_index });
        self
    }

    fn mint_plutus_script_v2(&mut self) -> &mut MeshTxBuilderCore {
        self.adding_plutus_mint = true;
        self
    }

    fn mint(&mut self, quantity: u64, policy: String, name: String) -> &mut MeshTxBuilderCore {
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

    fn minting_script(
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

    fn mint_tx_in_reference(
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

    fn mint_redeemer_value(&mut self, redeemer: Redeemer) -> &mut MeshTxBuilderCore {
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

    fn mint_reference_tx_in_redeemer_value(
        &mut self,
        redeemer: Redeemer,
    ) -> &mut MeshTxBuilderCore {
        self.mint_redeemer_value(redeemer)
    }

    fn required_signer_hash(&mut self, pub_key_hash: String) -> &mut MeshTxBuilderCore {
        self.mesh_tx_builder_body
            .required_signatures
            .push(pub_key_hash);
        self
    }

    fn tx_in_collateral(
        &mut self,
        tx_hash: String,
        tx_index: u32,
        amount: Vec<Asset>,
        address: String,
    ) -> &mut MeshTxBuilderCore {
        let collateral_item = self.collateral_item.take();
        if let Some(collateral_item) = collateral_item {
            self.mesh_tx_builder_body.collaterals.push(collateral_item);
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

    fn change_address(&mut self, address: String) -> &mut MeshTxBuilderCore {
        self.mesh_tx_builder_body.change_address = address;
        self
    }

    fn change_output_datum(&mut self, data: String) -> &mut MeshTxBuilderCore {
        self.mesh_tx_builder_body.change_datum = Some(Datum {
            type_: "Inline".to_string(),
            data,
        });
        self
    }

    fn invalid_before(&mut self, slot: u64) -> &mut MeshTxBuilderCore {
        self.mesh_tx_builder_body.validity_range.invalid_before = Some(slot);
        self
    }

    fn invalid_hereafter(&mut self, slot: u64) -> &mut MeshTxBuilderCore {
        self.mesh_tx_builder_body.validity_range.invalid_hereafter = Some(slot);
        self
    }

    fn metadata_value(&mut self, tag: String, metadata: String) -> &mut MeshTxBuilderCore {
        self.mesh_tx_builder_body
            .metadata
            .push(Metadata { tag, metadata });
        self
    }

    fn signing_key(&mut self, skey_hex: String) -> &mut MeshTxBuilderCore {
        self.mesh_tx_builder_body.signing_key.push(skey_hex);
        self
    }
    fn add_all_signing_keys(&mut self, signing_keys: Vec<String>) {
        if !signing_keys.is_empty() {
            self.mesh_csl.add_signing_keys(signing_keys);
        }
    }

    fn add_all_inputs(&mut self, inputs: Vec<TxIn>) {
        for input in inputs {
            match input {
                TxIn::PubKeyTxIn(pub_key_tx_in) => self.mesh_csl.add_tx_in(pub_key_tx_in),
                TxIn::ScriptTxIn(script_tx_in) => self.mesh_csl.add_script_tx_in(script_tx_in),
            };
        }
        self.mesh_csl
            .tx_builder
            .set_inputs(&self.mesh_csl.tx_inputs_builder);
    }

    fn add_all_outputs(&mut self, outputs: Vec<Output>) {
        for output in outputs {
            self.mesh_csl.add_output(output);
        }
    }

    fn add_all_collaterals(&mut self, collaterals: Vec<PubKeyTxIn>) {
        let mut collateral_builder = csl::tx_builder::tx_inputs_builder::TxInputsBuilder::new();
        for collateral in collaterals {
            self.mesh_csl
                .add_collateral(&mut collateral_builder, collateral)
        }
        self.mesh_csl.tx_builder.set_collateral(&collateral_builder)
    }

    fn add_all_reference_inputs(&mut self, ref_inputs: Vec<RefTxIn>) {
        for ref_input in ref_inputs {
            self.mesh_csl.add_reference_input(ref_input);
        }
    }

    fn add_all_mints(&mut self, mints: Vec<MintItem>) {
        let mut mint_builder = csl::tx_builder::mint_builder::MintBuilder::new();
        for (index, mint) in mints.into_iter().enumerate() {
            match mint.type_.as_str() {
                "Plutus" => self
                    .mesh_csl
                    .add_plutus_mint(&mut mint_builder, mint, index as u64),
                "Native" => self.mesh_csl.add_native_mint(&mut mint_builder, mint),
                _ => {}
            };
        }
        self.mesh_csl.tx_builder.set_mint_builder(&mint_builder)
    }

    fn add_validity_range(&mut self, validity_range: ValidityRange) {
        if validity_range.invalid_before.is_some() {
            self.mesh_csl
                .add_invalid_before(validity_range.invalid_before.unwrap())
        }
        if validity_range.invalid_hereafter.is_some() {
            self.mesh_csl
                .add_invalid_hereafter(validity_range.invalid_hereafter.unwrap())
        }
    }

    fn add_all_required_signature(&mut self, required_signatures: Vec<String>) {
        for pub_key_hash in required_signatures {
            self.mesh_csl.add_required_signature(pub_key_hash);
        }
    }

    fn add_all_metadata(&mut self, all_metadata: Vec<Metadata>) {
        for metadata in all_metadata {
            self.mesh_csl.add_metadata(metadata);
        }
    }

    // fn add_collateral_return(&mut self, change_address: String) {
    //     let current_fee = self
    //         .tx_builder
    //         .get_fee_if_set()
    //         .unwrap()
    //         .to_string()
    //         .parse::<u64>()
    //         .unwrap();

    //     let collateral_amount = 150 * ((current_fee / 100) + 1);
    //     let _ = self
    //         .tx_builder
    //         .set_total_collateral_and_return(
    //             &to_bignum(collateral_amount),
    //             &csl::address::Address::from_bech32(&change_address).unwrap(),
    //         )
    //         .unwrap();
    // }

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

    fn queue_all_last_item(&mut self) {
        if self.tx_output.is_some() {
            self.mesh_tx_builder_body
                .outputs
                .push(self.tx_output.clone().unwrap());
            self.tx_output = None;
        }
        if self.tx_in_item.is_some() {
            self.queue_input();
        }
        if self.collateral_item.is_some() {
            self.mesh_tx_builder_body
                .collaterals
                .push(self.collateral_item.clone().unwrap());
            self.collateral_item = None;
        }
        if self.mint_item.is_some() {
            self.queue_mint();
        }
    }
}

impl Default for MeshTxBuilderCore {
    fn default() -> Self {
        Self::new()
    }
}
