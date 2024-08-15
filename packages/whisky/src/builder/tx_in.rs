use super::{MeshTxBuilder, WData, WRedeemer};

use sidan_csl_rs::model::*;

impl MeshTxBuilder {
    /// ## Transaction building method
    ///
    /// Indicate that the transaction is spending a Plutus script in the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `language_version` - The language version of the script
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    pub fn spending_plutus_script(&mut self, language_version: &LanguageVersion) -> &mut Self {
        match language_version {
            LanguageVersion::V1 => self.spending_plutus_script_v1(),
            LanguageVersion::V2 => self.spending_plutus_script_v2(),
            LanguageVersion::V3 => self.spending_plutus_script_v3(),
        }
    }

    /// ## Transaction building method
    ///
    /// Indicate that the transaction is spending a Plutus script v1 in the MeshTxBuilder instance
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    pub fn spending_plutus_script_v1(&mut self) -> &mut Self {
        self.adding_script_input = Some(LanguageVersion::V1);
        self
    }
    /// ## Transaction building method
    ///
    /// Indicate that the transaction is spending a Plutus script v2 in the MeshTxBuilder instance
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    pub fn spending_plutus_script_v2(&mut self) -> &mut Self {
        self.adding_script_input = Some(LanguageVersion::V2);
        self
    }
    /// ## Transaction building method
    ///
    /// Indicate that the transaction is spending a Plutus script v2 in the MeshTxBuilder instance
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    pub fn spending_plutus_script_v3(&mut self) -> &mut Self {
        self.adding_script_input = Some(LanguageVersion::V3);
        self
    }

    /// ## Transaction building method
    ///
    /// Add a transaction input to the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `tx_hash` - The transaction hash
    /// * `tx_index` - The transaction index
    /// * `amount` - The amount of assets
    /// * `address` - The address
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    pub fn tx_in(
        &mut self,
        tx_hash: &str,
        tx_index: u32,
        amount: &[Asset],
        address: &str,
    ) -> &mut Self {
        if self.tx_in_item.is_some() {
            self.queue_input();
        }
        match self.adding_script_input {
            Some(_) => {
                let item = TxIn::ScriptTxIn(ScriptTxIn {
                    tx_in: TxInParameter {
                        tx_hash: tx_hash.to_string(),
                        tx_index,
                        amount: Some(amount.to_vec()),
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
            None => {
                let item = TxIn::PubKeyTxIn(PubKeyTxIn {
                    tx_in: TxInParameter {
                        tx_hash: tx_hash.to_string(),
                        tx_index,
                        amount: Some(amount.to_vec()),
                        address: Some(address.to_string()),
                    },
                });
                self.tx_in_item = Some(item);
            }
        }
        self
    }

    /// ## Transaction building method
    ///
    /// Add a transaction input script to the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `script_cbor` - The script in CBOR format
    /// * `version` - The language version, leave as None for Native scripts
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    pub fn tx_in_script(&mut self, script_cbor: &str) -> &mut Self {
        let tx_in_item = self.tx_in_item.take();
        if tx_in_item.is_none() {
            panic!("Undefined input")
        }
        let tx_in_item = tx_in_item.unwrap();
        match tx_in_item {
            TxIn::PubKeyTxIn(input) => {
                self.tx_in_item = Some(TxIn::SimpleScriptTxIn(SimpleScriptTxIn {
                    tx_in: input.tx_in,
                    simple_script_tx_in: SimpleScriptTxInParameter::ProvidedSimpleScriptSource(
                        ProvidedSimpleScriptSource {
                            script_cbor: script_cbor.to_string(),
                        },
                    ),
                }))
            }
            TxIn::ScriptTxIn(mut input) => {
                input.script_tx_in.script_source =
                    Some(ScriptSource::ProvidedScriptSource(ProvidedScriptSource {
                        script_cbor: script_cbor.to_string(),
                        language_version: self
                            .adding_script_input
                            .clone()
                            .expect("Plutus script must have version specified"),
                    }));
                self.adding_script_input = None;
                self.tx_in_item = Some(TxIn::ScriptTxIn(input));
            }

            // Technically this should be unreachable, but it's here for completeness
            TxIn::SimpleScriptTxIn(mut input) => {
                input.simple_script_tx_in = SimpleScriptTxInParameter::ProvidedSimpleScriptSource(
                    ProvidedSimpleScriptSource {
                        script_cbor: script_cbor.to_string(),
                    },
                );
                self.tx_in_item = Some(TxIn::SimpleScriptTxIn(input));
            }
        }
        self
    }

    /// ## Transaction building method
    ///
    /// Set the transaction input datum value in the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `data` - The datum value
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    pub fn tx_in_datum_value(&mut self, data: &WData) -> &mut Self {
        let tx_in_item = self.tx_in_item.take();
        if tx_in_item.is_none() {
            panic!("Undefined input")
        }
        let tx_in_item = tx_in_item.unwrap();
        match tx_in_item {
            TxIn::PubKeyTxIn(_) => panic!("Datum cannot be defined for a pubkey tx in"),
            TxIn::SimpleScriptTxIn(_) => {
                panic!("Datum cannot be defined for a simple script tx in")
            }
            TxIn::ScriptTxIn(mut input) => match data.to_cbor() {
                Ok(raw_data) => {
                    input.script_tx_in.datum_source =
                        Some(DatumSource::ProvidedDatumSource(ProvidedDatumSource {
                            data: raw_data.to_string(),
                        }));
                    self.tx_in_item = Some(TxIn::ScriptTxIn(input));
                }
                Err(_) => {
                    panic!("Error converting datum to CBOR");
                }
            },
        }
        self
    }

    /// ## Transaction building method
    ///
    /// Indicate that the transaction input has an inline datum in the MeshTxBuilder instance
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    pub fn tx_in_inline_datum_present(&mut self) -> &mut Self {
        let tx_in_item = self.tx_in_item.take();
        if tx_in_item.is_none() {
            panic!("Undefined input")
        }
        let tx_in_item = tx_in_item.unwrap();
        match tx_in_item {
            TxIn::PubKeyTxIn(_) => panic!("Datum cannot be defined for a pubkey tx in"),
            TxIn::SimpleScriptTxIn(_) => {
                panic!("Datum cannot be defined for a simple script tx in")
            }
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

    /// ## Transaction building method
    ///
    /// Set the transaction input redeemer value in the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `redeemer` - The redeemer value
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    pub fn tx_in_redeemer_value(&mut self, redeemer: &WRedeemer) -> &mut Self {
        let tx_in_item = self.tx_in_item.take();
        if tx_in_item.is_none() {
            panic!("Undefined input")
        }
        let tx_in_item = tx_in_item.unwrap();
        match tx_in_item {
            TxIn::PubKeyTxIn(_) => panic!("Redeemer cannot be defined for a pubkey tx in"),
            TxIn::SimpleScriptTxIn(_) => {
                panic!("Redeemer cannot be defined for a simple script tx in")
            }
            TxIn::ScriptTxIn(mut input) => match redeemer.data.to_cbor() {
                Ok(raw_redeemer) => {
                    input.script_tx_in.redeemer = Some(Redeemer {
                        data: raw_redeemer,
                        ex_units: redeemer.clone().ex_units,
                    });
                    self.tx_in_item = Some(TxIn::ScriptTxIn(input));
                }
                Err(_) => {
                    panic!("Error converting redeemer to CBOR");
                }
            },
        }
        self
    }

    /// ## Transaction building method
    ///
    /// Add a spending transaction input reference to the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `tx_hash` - The transaction hash
    /// * `tx_index` - The transaction index
    /// * `script_hash` - The spending script hash
    /// * `scrip_size` - Size of the script
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    pub fn spending_tx_in_reference(
        &mut self,
        tx_hash: &str,
        tx_index: u32,
        script_hash: &str,
        script_size: usize,
    ) -> &mut Self {
        let tx_in_item = self.tx_in_item.take();
        if tx_in_item.is_none() {
            panic!("Undefined output")
        }
        let tx_in_item = tx_in_item.unwrap();
        match tx_in_item {
            TxIn::PubKeyTxIn(_) => panic!("Script reference cannot be defined for a pubkey tx in"),
            TxIn::SimpleScriptTxIn(_) => {
                panic!("Script reference cannot be defined for a simple script tx in")
            }
            TxIn::ScriptTxIn(mut input) => {
                input.script_tx_in.script_source =
                    Some(ScriptSource::InlineScriptSource(InlineScriptSource {
                        ref_tx_in: RefTxIn {
                            tx_hash: tx_hash.to_string(),
                            tx_index,
                        },
                        script_hash: script_hash.to_string(),
                        language_version: self
                            .adding_script_input
                            .clone()
                            .expect("Plutus script must have version specified"),
                        script_size,
                    }));
                self.tx_in_item = Some(TxIn::ScriptTxIn(input));
            }
        }
        self
    }

    /// ## Transaction building method
    ///
    /// Indicate that the spending reference transaction input has an inline datum in the MeshTxBuilder instance
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    pub fn spending_reference_tx_in_inline_datum_present(&mut self) -> &mut Self {
        self.tx_in_inline_datum_present()
    }

    /// ## Transaction building method
    ///
    /// Set the spending reference transaction input redeemer value in the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `redeemer` - The redeemer value
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    pub fn spending_reference_tx_in_redeemer_value(&mut self, redeemer: &WRedeemer) -> &mut Self {
        self.tx_in_redeemer_value(redeemer)
    }

    /// ## Transaction building method
    ///
    /// Add a read-only transaction input reference to the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `tx_hash` - The transaction hash
    /// * `tx_index` - The transaction index
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    pub fn read_only_tx_in_reference(&mut self, tx_hash: &str, tx_index: u32) -> &mut Self {
        self.core
            .mesh_tx_builder_body
            .reference_inputs
            .push(RefTxIn {
                tx_hash: tx_hash.to_string(),
                tx_index,
            });
        self
    }

    /// ## Transaction building method
    ///
    /// Add a transaction input collateral to the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `tx_hash` - The transaction hash
    /// * `tx_index` - The transaction index
    /// * `amount` - The amount of assets
    /// * `address` - The address
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    pub fn tx_in_collateral(
        &mut self,
        tx_hash: &str,
        tx_index: u32,
        amount: &[Asset],
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
            tx_in: TxInParameter {
                tx_hash: tx_hash.to_string(),
                tx_index,
                amount: Some(amount.to_vec()),
                address: Some(address.to_string()),
            },
        });
        self
    }
}
