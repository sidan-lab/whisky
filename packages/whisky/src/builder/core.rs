use async_trait::async_trait;
use sidan_csl_rs::{
    builder::{serialize_tx_body, IMeshTxBuilderCore, MeshTxBuilderCore},
    core::{algo::select_utxos, builder::IMeshCSL, utils::build_tx_builder},
    csl,
    model::{
        Asset, Datum, DatumSource, InlineDatumSource, InlineScriptSource, LanguageVersion,
        MeshTxBuilderBody, Metadata, MintItem, Output, ProvidedDatumSource, ProvidedScriptSource,
        PubKeyTxIn, Redeemer, RefTxIn, ScriptSource, ScriptTxIn, ScriptTxInParameter, TxIn,
        TxInParameter, UTxO, Value,
    },
};

use super::{IMeshTxBuilder, MeshTxBuilder, MeshTxEvaluator};
use crate::service::ITxEvaluation;

#[async_trait]
impl IMeshTxBuilder for MeshTxBuilder {
    fn new(param: super::MeshTxBuilderParam) -> Self {
        MeshTxBuilder {
            core: MeshTxBuilderCore::new_core(None),
            protocol_params: param.params,
            tx_in_item: None,
            extra_inputs: vec![],
            selection_threshold: 5_000_000,
            mint_item: None,
            collateral_item: None,
            tx_output: None,
            adding_script_input: false,
            adding_plutus_mint: false,
            fetcher: param.fetcher,
            evaluator: match param.evaluator {
                Some(evaluator) => Some(evaluator),
                None => Some(Box::new(MeshTxEvaluator::new())),
            },
            submitter: param.submitter,
            chained_txs: vec![],
            inputs_for_evaluation: vec![],
        }
    }

    fn new_core() -> Self {
        Self::new(super::MeshTxBuilderParam {
            evaluator: None,
            fetcher: None,
            submitter: None,
            params: None,
        })
    }

    async fn complete(&mut self, customized_tx: Option<MeshTxBuilderBody>) -> &mut Self {
        self.complete_sync(customized_tx);
        match &self.evaluator {
            Some(evaluator) => {
                let tx_evaluation_result = evaluator
                    .evaluate_tx(
                        &self.core.mesh_csl.tx_hex,
                        &self.inputs_for_evaluation.clone(),
                        &self.chained_txs.clone(),
                    )
                    .await;
                match tx_evaluation_result {
                    Ok(actions) => self.update_redeemer(actions),
                    Err(_) => panic!("Error evaluating transaction"),
                }
            }
            None => self,
        };
        self.complete_sync(None)
    }

    fn complete_sync(&mut self, customized_tx: Option<MeshTxBuilderBody>) -> &mut Self {
        if customized_tx.is_some() {
            self.core.mesh_tx_builder_body = customized_tx.unwrap();
        } else {
            self.queue_all_last_item();
            if !self.extra_inputs.is_empty() {
                self.add_utxos_from(self.extra_inputs.clone(), self.selection_threshold);
            }
        }

        self.core
            .mesh_tx_builder_body
            .mints
            .sort_by(|a, b| a.policy_id.cmp(&b.policy_id));

        self.core.mesh_tx_builder_body.inputs.sort_by(|a, b| {
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

        let tx_hex = serialize_tx_body(
            self.core.mesh_tx_builder_body.clone(),
            self.protocol_params.clone(),
        );
        self.core.mesh_csl.tx_hex = tx_hex;
        self.core.mesh_csl.tx_builder = build_tx_builder(None);
        self.core.mesh_csl.tx_inputs_builder = csl::TxInputsBuilder::new();
        self
    }

    fn complete_signing(&mut self) -> String {
        self.core.complete_signing()
    }

    fn tx_hex(&mut self) -> String {
        self.core.mesh_csl.tx_hex.to_string()
    }

    fn tx_in(
        &mut self,
        tx_hash: &str,
        tx_index: u32,
        amount: Vec<Asset>,
        address: &str,
    ) -> &mut Self {
        if self.tx_in_item.is_some() {
            self.queue_input();
        }
        if !self.adding_script_input {
            let item = TxIn::PubKeyTxIn(PubKeyTxIn {
                type_: "PubKey".to_string(),
                tx_in: TxInParameter {
                    tx_hash: tx_hash.to_string(),
                    tx_index,
                    amount: Some(amount),
                    address: Some(address.to_string()),
                },
            });
            self.tx_in_item = Some(item);
        } else {
            let item = TxIn::ScriptTxIn(ScriptTxIn {
                type_: "Script".to_string(),
                tx_in: TxInParameter {
                    tx_hash: tx_hash.to_string(),
                    tx_index,
                    amount: Some(amount),
                    address: Some(address.to_string()),
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

    fn tx_in_script(&mut self, script_cbor: &str, version: LanguageVersion) -> &mut Self {
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
                        script_cbor: script_cbor.to_string(),
                        language_version: version,
                    }));
                self.tx_in_item = Some(TxIn::ScriptTxIn(input));
            }
        }
        self
    }

    fn tx_in_datum_value(&mut self, data: &str) -> &mut Self {
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
                        data: data.to_string(),
                    }));
                self.tx_in_item = Some(TxIn::ScriptTxIn(input));
            }
        }
        self
    }

    fn tx_in_inline_datum_present(&mut self) -> &mut Self {
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

    fn tx_in_redeemer_value(&mut self, redeemer: Redeemer) -> &mut Self {
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

    fn tx_out(&mut self, address: &str, amount: Vec<Asset>) -> &mut Self {
        if self.tx_output.is_some() {
            let tx_output = self.tx_output.take();
            self.core
                .mesh_tx_builder_body
                .outputs
                .push(tx_output.unwrap());
        }
        self.tx_output = Some(Output {
            address: address.to_string(),
            amount,
            datum: None,
            reference_script: None,
        });
        self
    }

    fn tx_out_datum_hash_value(&mut self, data: &str) -> &mut Self {
        let tx_output = self.tx_output.take();
        if tx_output.is_none() {
            panic!("Undefined output")
        }
        let mut tx_output = tx_output.unwrap();
        tx_output.datum = Some(Datum {
            type_: "Hash".to_string(),
            data: data.to_string(),
        });
        self.tx_output = Some(tx_output);
        self
    }

    fn tx_out_inline_datum_value(&mut self, data: &str) -> &mut Self {
        let tx_output = self.tx_output.take();
        if tx_output.is_none() {
            panic!("Undefined output")
        }
        let mut tx_output = tx_output.unwrap();
        tx_output.datum = Some(Datum {
            type_: "Inline".to_string(),
            data: data.to_string(),
        });
        self.tx_output = Some(tx_output);
        self
    }

    fn tx_out_reference_script(
        &mut self,
        script_cbor: &str,
        version: LanguageVersion,
    ) -> &mut Self {
        let tx_output = self.tx_output.take();
        if tx_output.is_none() {
            panic!("Undefined output")
        }
        let mut tx_output = tx_output.unwrap();
        tx_output.reference_script = Some(ProvidedScriptSource {
            script_cbor: script_cbor.to_string(),
            language_version: version,
        });
        self.tx_output = Some(tx_output);
        self
    }

    fn spending_plutus_script_v2(&mut self) -> &mut Self {
        self.adding_script_input = true;
        self
    }

    fn spending_tx_in_reference(
        &mut self,
        tx_hash: &str,
        tx_index: u32,
        spending_script_hash: &str,
        version: LanguageVersion,
        script_size: usize,
    ) -> &mut Self {
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
                        tx_hash: tx_hash.to_string(),
                        tx_index,
                        spending_script_hash: spending_script_hash.to_string(),
                        language_version: version,
                        script_size,
                    }));
                self.tx_in_item = Some(TxIn::ScriptTxIn(input));
            }
        }
        self
    }

    fn spending_reference_tx_in_inline_datum_present(&mut self) -> &mut Self {
        self.tx_in_inline_datum_present()
    }

    fn spending_reference_tx_in_redeemer_value(&mut self, redeemer: Redeemer) -> &mut Self {
        self.tx_in_redeemer_value(redeemer)
    }

    fn read_only_tx_in_reference(&mut self, tx_hash: &str, tx_index: u32) -> &mut Self {
        self.core
            .mesh_tx_builder_body
            .reference_inputs
            .push(RefTxIn {
                tx_hash: tx_hash.to_string(),
                tx_index,
            });
        self
    }

    fn mint_plutus_script_v2(&mut self) -> &mut Self {
        self.adding_plutus_mint = true;
        self
    }

    fn mint(&mut self, quantity: u64, policy: &str, name: &str) -> &mut Self {
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
            policy_id: policy.to_string(),
            asset_name: name.to_string(),
            amount: quantity,
            redeemer: None,
            script_source: None,
        });
        self.adding_plutus_mint = false;
        self
    }

    fn minting_script(&mut self, script_cbor: &str, version: LanguageVersion) -> &mut Self {
        let mint_item = self.mint_item.take();
        if mint_item.is_none() {
            panic!("Undefined mint");
        }
        let mut mint_item = mint_item.unwrap();
        mint_item.script_source = Some(ScriptSource::ProvidedScriptSource(ProvidedScriptSource {
            script_cbor: script_cbor.to_string(),
            language_version: version,
        }));
        self.mint_item = Some(mint_item);
        self
    }

    fn mint_tx_in_reference(
        &mut self,
        tx_hash: &str,
        tx_index: u32,
        spending_script_hash: &str,
        version: LanguageVersion,
        script_size: usize,
    ) -> &mut Self {
        let mint_item = self.mint_item.take();
        if mint_item.is_none() {
            panic!("Undefined mint");
        }
        let mut mint_item = mint_item.unwrap();
        mint_item.script_source = Some(ScriptSource::InlineScriptSource(InlineScriptSource {
            tx_hash: tx_hash.to_string(),
            tx_index,
            spending_script_hash: spending_script_hash.to_string(),
            language_version: version,
            script_size,
        }));
        self.mint_item = Some(mint_item);
        self
    }

    fn mint_redeemer_value(&mut self, redeemer: Redeemer) -> &mut Self {
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

    fn mint_reference_tx_in_redeemer_value(&mut self, redeemer: Redeemer) -> &mut Self {
        self.mint_redeemer_value(redeemer)
    }

    fn required_signer_hash(&mut self, pub_key_hash: &str) -> &mut Self {
        self.core
            .mesh_tx_builder_body
            .required_signatures
            .add(pub_key_hash.to_string());
        self
    }

    fn tx_in_collateral(
        &mut self,
        tx_hash: &str,
        tx_index: u32,
        amount: Vec<Asset>,
        address: &str,
    ) -> &mut Self {
        let collateral_item = self.collateral_item.take();
        if let Some(collateral_item) = collateral_item {
            self.core
                .mesh_tx_builder_body
                .collaterals
                .push(collateral_item);
        }
        self.collateral_item = Some(PubKeyTxIn {
            type_: "PubKey".to_string(),
            tx_in: TxInParameter {
                tx_hash: tx_hash.to_string(),
                tx_index,
                amount: Some(amount),
                address: Some(address.to_string()),
            },
        });
        self
    }

    fn change_address(&mut self, address: &str) -> &mut Self {
        self.core.mesh_tx_builder_body.change_address = address.to_string();
        self
    }

    fn change_output_datum(&mut self, data: &str) -> &mut Self {
        self.core.mesh_tx_builder_body.change_datum = Some(Datum {
            type_: "Inline".to_string(),
            data: data.to_string(),
        });
        self
    }

    fn invalid_before(&mut self, slot: u64) -> &mut Self {
        self.core.mesh_tx_builder_body.validity_range.invalid_before = Some(slot);
        self
    }

    fn invalid_hereafter(&mut self, slot: u64) -> &mut Self {
        self.core
            .mesh_tx_builder_body
            .validity_range
            .invalid_hereafter = Some(slot);
        self
    }

    fn metadata_value(&mut self, tag: &str, metadata: &str) -> &mut Self {
        self.core.mesh_tx_builder_body.metadata.push(Metadata {
            tag: tag.to_string(),
            metadata: metadata.to_string(),
        });
        self
    }

    fn signing_key(&mut self, skey_hex: &str) -> &mut Self {
        self.core
            .mesh_tx_builder_body
            .signing_key
            .add(skey_hex.to_string());
        self
    }

    fn chain_tx(&mut self, tx_hex: &str) -> &mut Self {
        self.chained_txs.push(tx_hex.to_string());
        self
    }

    fn input_for_evaluation(&mut self, input: UTxO) -> &mut Self {
        self.inputs_for_evaluation.push(input);
        self
    }

    fn select_utxos_from(&mut self, extra_inputs: Vec<UTxO>, threshold: u64) -> &mut Self {
        self.selection_threshold = threshold;
        self.extra_inputs = extra_inputs;
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
        self.core
            .mesh_tx_builder_body
            .inputs
            .push(self.tx_in_item.clone().unwrap());
        self.tx_in_item = None
    }

    fn queue_mint(&mut self) {
        let mint_item = self.mint_item.clone().unwrap();
        if mint_item.script_source.is_none() {
            panic!("Missing mint script information");
        }
        self.core.mesh_tx_builder_body.mints.push(mint_item);
        self.mint_item = None;
    }

    fn queue_all_last_item(&mut self) {
        if self.tx_output.is_some() {
            self.core
                .mesh_tx_builder_body
                .outputs
                .push(self.tx_output.clone().unwrap());
            self.tx_output = None;
        }
        if self.tx_in_item.is_some() {
            self.queue_input();
        }
        if self.collateral_item.is_some() {
            self.core
                .mesh_tx_builder_body
                .collaterals
                .push(self.collateral_item.clone().unwrap());
            self.collateral_item = None;
        }
        if self.mint_item.is_some() {
            self.queue_mint();
        }
    }

    fn add_utxos_from(&mut self, extra_inputs: Vec<UTxO>, threshold: u64) {
        let mut required_assets = Value::new();

        for output in &self.core.mesh_tx_builder_body.outputs {
            let output_value = Value::from_asset_vec(output.amount.clone());
            required_assets.merge(output_value);
        }

        for input in &self.core.mesh_tx_builder_body.inputs {
            match input {
                TxIn::PubKeyTxIn(pub_key_tx_in) => {
                    let input_value =
                        Value::from_asset_vec(pub_key_tx_in.tx_in.amount.clone().unwrap());
                    required_assets.negate_assets(input_value);
                }
                TxIn::ScriptTxIn(script_tx_in) => {
                    let input_value =
                        Value::from_asset_vec(script_tx_in.tx_in.amount.clone().unwrap());
                    required_assets.negate_assets(input_value);
                }
            }
        }

        for mint in &self.core.mesh_tx_builder_body.mints {
            let mint_amount = Asset::new(
                mint.policy_id.clone() + &mint.asset_name,
                mint.amount.to_string(),
            );
            required_assets.negate_asset(mint_amount);
        }

        let selected_inputs =
            select_utxos(extra_inputs, required_assets, threshold.to_string()).unwrap();

        for input in selected_inputs {
            self.core.mesh_csl.add_tx_in(PubKeyTxIn {
                type_: "PubKey".to_string(),
                tx_in: TxInParameter {
                    tx_hash: input.input.tx_hash.clone(),
                    tx_index: input.input.output_index,
                    amount: Some(input.output.amount.clone()),
                    address: Some(input.output.address.clone()),
                },
            });
            self.core
                .mesh_tx_builder_body
                .inputs
                .push(TxIn::PubKeyTxIn(PubKeyTxIn {
                    type_: "PubKey".to_string(),
                    tx_in: TxInParameter {
                        tx_hash: input.input.tx_hash,
                        tx_index: input.input.output_index,
                        amount: Some(input.output.amount),
                        address: Some(input.output.address),
                    },
                }));
        }
    }
}
