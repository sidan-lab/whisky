use cardano_serialization_lib::JsError;
use sidan_csl_rs::{
    core::utils::get_script_hash,
    model::{LanguageVersion, UTxO},
};

use crate::builder::{WData, WRedeemer};

use super::WhiskyTx;

pub struct TxInput {
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
        language_version: &LanguageVersion,
        input: &[TxInput],
        redeemer: &WRedeemer,
        script_cbor: &str,
        ref_script_input: Option<UTxO>,
    ) -> Result<&mut Self, JsError> {
        for input in input.iter() {
            let utxo = &input.utxo;
            self.tx_builder
                .spending_plutus_script(language_version)
                .tx_in(
                    &utxo.input.tx_hash,
                    utxo.input.output_index,
                    &utxo.output.amount,
                    &utxo.output.address,
                )
                .tx_in_redeemer_value(redeemer);
            match &input.datum {
                InputDatum::Hash(datum) => {
                    self.tx_builder.tx_in_datum_value(datum);
                }
                InputDatum::Inline => {
                    self.tx_builder.tx_in_inline_datum_present();
                }
            }
            match ref_script_input.clone() {
                Some(ref_script_input) => {
                    self.tx_builder
                        .spending_tx_in_reference(
                            &ref_script_input.input.tx_hash,
                            ref_script_input.input.output_index,
                            &get_script_hash(script_cbor, language_version.clone())?,
                            script_cbor.len() / 2,
                        )
                        .input_for_evaluation(&ref_script_input);
                }
                None => {
                    self.tx_builder.tx_in_script(script_cbor);
                }
            }
            self.tx_builder.input_for_evaluation(utxo);
        }
        Ok(self)
    }

    pub fn add_collateral(&mut self, collateral: &UTxO) -> Result<&mut Self, JsError> {
        self.tx_builder.tx_in_collateral(
            &collateral.input.tx_hash,
            collateral.input.output_index,
            &collateral.output.amount,
            &collateral.output.address,
        );
        Ok(self)
    }
}
