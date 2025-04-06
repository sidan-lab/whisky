use crate::*;

use super::{TxBuilder, WRedeemer};

impl TxBuilder {
    /// ## Transaction building method
    ///
    /// Indicate that the transaction is withdrawing using a plutus staking script in the TxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `language_version` - The language version of the script
    ///
    /// ### Returns
    ///
    /// * `Self` - The TxBuilder instance
    pub fn mint_plutus_script(&mut self, language_version: &LanguageVersion) -> &mut Self {
        match language_version {
            LanguageVersion::V1 => self.mint_plutus_script_v1(),
            LanguageVersion::V2 => self.mint_plutus_script_v2(),
            LanguageVersion::V3 => self.mint_plutus_script_v3(),
        }
    }

    /// ## Transaction building method
    ///
    /// Indicate that the transaction is minting a Plutus script v1 in the TxBuilder instance
    ///
    /// ### Returns
    ///
    /// * `Self` - The TxBuilder instance
    pub fn mint_plutus_script_v1(&mut self) -> &mut Self {
        self.adding_plutus_mint = Some(LanguageVersion::V1);
        self
    }

    /// ## Transaction building method
    ///
    /// Indicate that the transaction is minting a Plutus script v2 in the TxBuilder instance
    ///
    /// ### Returns
    ///
    /// * `Self` - The TxBuilder instance
    pub fn mint_plutus_script_v2(&mut self) -> &mut Self {
        self.adding_plutus_mint = Some(LanguageVersion::V2);
        self
    }

    /// ## Transaction building method
    ///
    /// Indicate that the transaction is minting a Plutus script v2 in the TxBuilder instance
    ///
    /// ### Returns
    ///
    /// * `Self` - The TxBuilder instance
    pub fn mint_plutus_script_v3(&mut self) -> &mut Self {
        self.adding_plutus_mint = Some(LanguageVersion::V3);
        self
    }

    /// ## Transaction building method
    ///
    /// Mint assets in the TxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `quantity` - The quantity of assets to mint
    /// * `policy` - The policy
    /// * `name` - The name of the asset
    ///
    /// ### Returns
    ///
    /// * `Self` - The TxBuilder instance
    pub fn mint(&mut self, quantity: i128, policy: &str, name: &str) -> &mut Self {
        if self.mint_item.is_some() {
            self.queue_mint();
        }
        match &self.adding_plutus_mint {
            Some(_) => {
                self.mint_item = Some(MintItem::ScriptMint(ScriptMint {
                    mint: MintParameter {
                        policy_id: policy.to_string(),
                        asset_name: name.to_string(),
                        amount: quantity,
                    },
                    redeemer: None,
                    script_source: None,
                }))
            }
            None => {
                self.mint_item = Some(MintItem::SimpleScriptMint(SimpleScriptMint {
                    mint: MintParameter {
                        policy_id: policy.to_string(),
                        asset_name: name.to_string(),
                        amount: quantity,
                    },
                    script_source: None,
                }))
            }
        }
        self
    }

    /// ## Transaction building method
    ///
    /// Add a minting script to the TxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `script_cbor` - The script in CBOR format
    /// * `version` - The language version, if the language version is None, the script is assumed to be a Native Script
    ///
    /// ### Returns
    ///
    /// * `Self` - The TxBuilder instance
    pub fn minting_script(&mut self, script_cbor: &str) -> &mut Self {
        let mint_item = self.mint_item.take();
        if mint_item.is_none() {
            panic!("Undefined mint");
        }
        let mint_item = mint_item.unwrap();
        match mint_item {
            MintItem::ScriptMint(mut script_mint) => {
                script_mint.script_source =
                    Some(ScriptSource::ProvidedScriptSource(ProvidedScriptSource {
                        script_cbor: script_cbor.to_string(),
                        language_version: self
                            .adding_plutus_mint
                            .clone()
                            .expect("Plutus mints must have version specified"),
                    }));
                self.adding_plutus_mint = None;
                self.mint_item = Some(MintItem::ScriptMint(script_mint));
            }
            MintItem::SimpleScriptMint(mut simple_script_mint) => {
                simple_script_mint.script_source = Some(
                    SimpleScriptSource::ProvidedSimpleScriptSource(ProvidedSimpleScriptSource {
                        script_cbor: script_cbor.to_string(),
                    }),
                );
                self.mint_item = Some(MintItem::SimpleScriptMint(simple_script_mint));
            }
        }
        self
    }

    /// ## Transaction building method
    ///
    /// Add a minting transaction input reference to the TxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `tx_hash` - The transaction hash
    /// * `tx_index` - The transaction index
    /// * `script_hash` - The script hash
    /// * `version` - The language version, if the language version is None, the script is assumed to be a Native Script
    /// * `script_size` - Size of the script
    ///
    /// ### Returns
    ///
    /// * `Self` - The TxBuilder instance
    pub fn mint_tx_in_reference(
        &mut self,
        tx_hash: &str,
        tx_index: u32,
        script_hash: &str,
        script_size: usize,
    ) -> &mut Self {
        let mint_item = self.mint_item.take();
        if mint_item.is_none() {
            panic!("Undefined mint");
        }
        let mint_item = mint_item.unwrap();
        match mint_item {
            MintItem::ScriptMint(mut script_mint) => {
                script_mint.script_source =
                    Some(ScriptSource::InlineScriptSource(InlineScriptSource {
                        ref_tx_in: RefTxIn {
                            tx_hash: tx_hash.to_string(),
                            tx_index,
                            // Script size is already accounted for in script source
                            script_size: None,
                        },
                        script_hash: script_hash.to_string(),
                        language_version: self
                            .adding_plutus_mint
                            .clone()
                            .expect("Plutus mints must have version specified"),
                        script_size,
                    }));
                self.mint_item = Some(MintItem::ScriptMint(script_mint));
            }
            MintItem::SimpleScriptMint(mut simple_script_mint) => {
                simple_script_mint.script_source = Some(
                    SimpleScriptSource::InlineSimpleScriptSource(InlineSimpleScriptSource {
                        ref_tx_in: RefTxIn {
                            tx_hash: tx_hash.to_string(),
                            tx_index,
                            // Script size is already accounted for in script source
                            script_size: None,
                        },
                        simple_script_hash: script_hash.to_string(),
                        script_size,
                    }),
                );
                self.mint_item = Some(MintItem::SimpleScriptMint(simple_script_mint));
            }
        }
        self
    }

    /// ## Transaction building method
    ///
    /// Set the minting redeemer value in the TxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `redeemer` - The redeemer value
    ///
    /// ### Returns
    ///
    /// * `Self` - The TxBuilder instance
    pub fn mint_redeemer_value(&mut self, redeemer: &WRedeemer) -> &mut Self {
        let mint_item = self.mint_item.take();
        if mint_item.is_none() {
            panic!("Undefined mint");
        }
        let mint_item = mint_item.unwrap();
        match mint_item {
            MintItem::ScriptMint(mut script_mint) => match redeemer.data.to_cbor() {
                Ok(raw_redeemer) => {
                    script_mint.redeemer = Some(Redeemer {
                        data: raw_redeemer,
                        ex_units: redeemer.clone().ex_units,
                    });
                    self.mint_item = Some(MintItem::ScriptMint(script_mint));
                }
                Err(_) => {
                    panic!("Error converting redeemer to CBOR");
                }
            },
            MintItem::SimpleScriptMint(_) => {
                panic!("Redeemer values cannot be defined for native script mints")
            }
        }

        self
    }

    /// ## Transaction building method
    ///
    /// Set the minting reference transaction input redeemer value in the TxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `redeemer` - The redeemer value
    ///
    /// ### Returns
    ///
    /// * `Self` - The TxBuilder instance
    pub fn mint_reference_tx_in_redeemer_value(&mut self, redeemer: &WRedeemer) -> &mut Self {
        self.mint_redeemer_value(redeemer)
    }
}
