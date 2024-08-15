use sidan_csl_rs::model::*;

use super::{MeshTxBuilder, WData};

impl MeshTxBuilder {
    /// ## Transaction building method
    ///
    /// Add a transaction output to the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `address` - The address
    /// * `amount` - The amount of assets
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    pub fn tx_out(&mut self, address: &str, amount: &[Asset]) -> &mut Self {
        if self.tx_output.is_some() {
            let tx_output = self.tx_output.take();
            self.core
                .mesh_tx_builder_body
                .outputs
                .push(tx_output.unwrap());
        }
        self.tx_output = Some(Output {
            address: address.to_string(),
            amount: amount.to_vec(),
            datum: None,
            reference_script: None,
        });
        self
    }

    /// ## Transaction building method
    ///
    /// Set the transaction output datum hash value in the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `data` - The datum hash value
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    pub fn tx_out_datum_hash_value(&mut self, data: &WData) -> &mut Self {
        let tx_output = self.tx_output.take();
        if tx_output.is_none() {
            panic!("Undefined output")
        }
        let mut tx_output = tx_output.unwrap();
        match data.to_cbor() {
            Ok(raw_data) => {
                tx_output.datum = Some(Datum::Hash(raw_data));
                self.tx_output = Some(tx_output);
            }
            Err(_) => {
                panic!("Error converting datum to CBOR");
            }
        }
        self
    }

    /// ## Transaction building method
    ///
    /// Set the transaction output inline datum value in the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `data` - The inline datum value
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    pub fn tx_out_inline_datum_value(&mut self, data: &WData) -> &mut Self {
        let tx_output = self.tx_output.take();
        if tx_output.is_none() {
            panic!("Undefined output")
        }
        let mut tx_output = tx_output.unwrap();
        match data.to_cbor() {
            Ok(raw_data) => {
                tx_output.datum = Some(Datum::Inline(raw_data));
                self.tx_output = Some(tx_output);
            }
            Err(_) => {
                panic!("Error converting datum to CBOR");
            }
        }
        self
    }

    /// ## Transaction building method
    ///
    /// Add a transaction output reference script to the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `script_cbor` - The script in CBOR format
    /// * `version` - The language version, if the language version is None, the script is assumed to be a Native Script
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    pub fn tx_out_reference_script(
        &mut self,
        script_cbor: &str,
        version: Option<LanguageVersion>,
    ) -> &mut Self {
        let tx_output = self.tx_output.take();
        if tx_output.is_none() {
            panic!("Undefined output")
        }
        let mut tx_output = tx_output.unwrap();
        match version {
            Some(language_version) => {
                tx_output.reference_script = Some(OutputScriptSource::ProvidedScriptSource(
                    ProvidedScriptSource {
                        script_cbor: script_cbor.to_string(),
                        language_version,
                    },
                ));
            }
            None => {
                tx_output.reference_script = Some(OutputScriptSource::ProvidedSimpleScriptSource(
                    ProvidedSimpleScriptSource {
                        script_cbor: script_cbor.to_string(),
                    },
                ))
            }
        }

        self.tx_output = Some(tx_output);
        self
    }
}
