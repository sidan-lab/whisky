use cardano_serialization_lib::JsError;
use sidan_csl_rs::model::{LanguageVersion, UTxO, UtxoInput, UtxoOutput};

use crate::builder::{WData, WRedeemer};

use super::{RefScriptInput, WhiskyTx};

pub struct Input {
    pub utxo: UTxO,
    pub datum: InputDatum,
}

pub enum InputDatum {
    Hash(WData),
    Inline,
}

impl WhiskyTx {
    pub fn unlock_with_full_eval(
        &mut self,
        language_version: LanguageVersion,
        input: &[Input],
        redeemer: WRedeemer,
        script_cbor: &str,
        ref_script_input: Option<RefScriptInput>,
    ) -> Result<&mut Self, JsError> {
        for input in input.iter() {
            let utxo = &input.utxo;
            self.tx_builder
                .spending_plutus_script(language_version.clone())
                .tx_in(
                    &utxo.input.tx_hash,
                    utxo.input.output_index,
                    utxo.output.amount.clone(),
                    &utxo.output.address,
                )
                .tx_in_redeemer_value(redeemer.clone());
            match &input.datum {
                InputDatum::Hash(datum) => {
                    self.tx_builder.tx_in_datum_value(datum.clone());
                }
                InputDatum::Inline => {
                    self.tx_builder.tx_in_inline_datum_present();
                }
            }
            match ref_script_input.clone() {
                Some(ref_script_input) => {
                    self.tx_builder.spending_tx_in_reference(
                        &ref_script_input.tx_hash,
                        ref_script_input.tx_index,
                        &ref_script_input.script_hash,
                        ref_script_input.script_size,
                    );
                    self.tx_builder.input_for_evaluation(UTxO {
                        input: UtxoInput {
                            tx_hash: ref_script_input.tx_hash,
                            output_index: ref_script_input.tx_index,
                        },
                        output: UtxoOutput {
                            address: "".to_string(),
                            amount: vec![],
                            data_hash: None,
                            plutus_data: None,
                            script_ref: Some(script_cbor.to_string()),
                            script_hash: None,
                        },
                    });
                }
                None => {
                    self.tx_builder.tx_in_script(script_cbor);
                }
            }
            self.tx_builder.input_for_evaluation(utxo.clone());
        }
        Ok(self)
    }
}
