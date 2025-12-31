use std::collections::HashMap;

use pallas::ledger::primitives::Fragment;
use whisky_common::{
    DatumSource::InlineDatumSource,
    DatumSource::ProvidedDatumSource,
    LanguageVersion,
    ScriptSource::{InlineScriptSource, ProvidedScriptSource},
    SimpleScriptTxInParameter::{InlineSimpleScriptSource, ProvidedSimpleScriptSource},
    TxBuilderBody, TxIn, WError,
};

use crate::{
    converter::{convert_inputs, convert_value},
    wrapper::{
        transaction_body::{Transaction, TransactionBody, TransactionInput, Value},
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
    pub tx_builder_body: TxBuilderBody,
    pub tx_evaluation_multiplier_percentage: u64,
    pub tx_hex: String,

    // Required info for balancing transaction
    pub inputs_map: HashMap<TransactionInput, Value>,

    // Required info for generating witness set
    pub native_scripts_vec: Vec<NativeScript>,
    pub plutus_v1_scripts_vec: Vec<PlutusScript<1>>,
    pub plutus_v2_scripts_vec: Vec<PlutusScript<2>>,
    pub plutus_v3_scripts_vec: Vec<PlutusScript<3>>,
    pub redeemers_map: HashMap<TransactionInput, Redeemer>,
    pub plutus_data_vec: Vec<PlutusData>,

    // Potential reference inputs (shouldn't overlap with actual inputs)
    pub ref_inputs_vec: Vec<TransactionInput>,
}

impl CorePallas {
    pub fn new(tx_builder_body: TxBuilderBody, tx_evaluation_multiplier_percentage: u64) -> Self {
        Self {
            tx_builder_body,
            tx_evaluation_multiplier_percentage,
            tx_hex: String::new(),
            inputs_map: HashMap::new(),
            native_scripts_vec: vec![],
            plutus_v1_scripts_vec: vec![],
            plutus_v2_scripts_vec: vec![],
            plutus_v3_scripts_vec: vec![],
            redeemers_map: HashMap::new(),
            plutus_data_vec: vec![],
            ref_inputs_vec: vec![],
        }
    }

    fn add_inputs(&mut self) -> Result<Vec<TransactionInput>, WError> {
        let mut inputs: Vec<TransactionInput> = vec![];
        for tx_in in &self.tx_builder_body.inputs {
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

                    match script_source {
                        ProvidedScriptSource(provided_script_source) => {
                            match provided_script_source.language_version {
                                LanguageVersion::V1 => {
                                    self.plutus_v1_scripts_vec.push(PlutusScript::<1>::new(
                                        provided_script_source.script_cbor,
                                    )?);
                                }
                                LanguageVersion::V2 => {
                                    self.plutus_v2_scripts_vec.push(PlutusScript::<2>::new(
                                        provided_script_source.script_cbor,
                                    )?);
                                }
                                LanguageVersion::V3 => {
                                    self.plutus_v3_scripts_vec.push(PlutusScript::<3>::new(
                                        provided_script_source.script_cbor,
                                    )?);
                                }
                            }
                        }
                        InlineScriptSource(inline_script_source) => {
                            self.ref_inputs_vec.push(TransactionInput::new(
                                &inline_script_source.ref_tx_in.tx_hash,
                                inline_script_source.ref_tx_in.tx_index.into(),
                            )?)
                        }
                    }

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
                    }

                    self.redeemers_map.insert(
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
                    );
                }
            }
        }
        Ok(inputs)
    }

    pub fn build_tx(&mut self) -> Result<String, WError> {
        let tx_body = TransactionBody::new(
            convert_inputs(&self.tx_builder_body.inputs)?,
            vec![],
            0,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )?;
        let witness_set = WitnessSet::new(None, None, None, None, None, None, None, None)?;
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
