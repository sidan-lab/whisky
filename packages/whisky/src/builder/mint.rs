use sidan_csl_rs::model::*;

use super::{MeshTxBuilder, WRedeemer};

impl MeshTxBuilder {
    /// ## Transaction building method
    ///
    /// Indicate that the transaction is minting a Plutus script v2 in the MeshTxBuilder instance
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    pub fn mint_plutus_script_v2(&mut self) -> &mut Self {
        self.adding_plutus_mint = true;
        self
    }

    /// ## Transaction building method
    ///
    /// Mint assets in the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `quantity` - The quantity of assets to mint
    /// * `policy` - The policy
    /// * `name` - The name of the asset
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    pub fn mint(&mut self, quantity: i128, policy: &str, name: &str) -> &mut Self {
        if self.mint_item.is_some() {
            self.queue_mint();
        }
        if self.adding_plutus_mint {
            self.mint_item = Some(MintItem::ScriptMint(ScriptMint {
                mint: MintParameter {
                    policy_id: policy.to_string(),
                    asset_name: name.to_string(),
                    amount: quantity,
                },
                redeemer: None,
                script_source: None,
            }))
        } else {
            self.mint_item = Some(MintItem::SimpleScriptMint(SimpleScriptMint {
                mint: MintParameter {
                    policy_id: policy.to_string(),
                    asset_name: name.to_string(),
                    amount: quantity,
                },
                script_source: None,
            }))
        };
        self.adding_plutus_mint = false;
        self
    }

    /// ## Transaction building method
    ///
    /// Add a minting script to the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `script_cbor` - The script in CBOR format
    /// * `version` - The language version, if the language version is None, the script is assumed to be a Native Script
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    pub fn minting_script(
        &mut self,
        script_cbor: &str,
        version: Option<LanguageVersion>,
    ) -> &mut Self {
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
                        language_version: version
                            .expect("Plutus mints must have version specified"),
                    }));
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
    /// Add a minting transaction input reference to the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `tx_hash` - The transaction hash
    /// * `tx_index` - The transaction index
    /// * `spending_script_hash` - The spending script hash
    /// * `version` - The language version, if the language version is None, the script is assumed to be a Native Script
    /// * `script_size` - Size of the script
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    pub fn mint_tx_in_reference(
        &mut self,
        tx_hash: &str,
        tx_index: u32,
        spending_script_hash: &str,
        version: Option<LanguageVersion>,
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
                        },
                        spending_script_hash: spending_script_hash.to_string(),
                        language_version: version
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
                        },
                        simple_script_hash: spending_script_hash.to_string(),
                    }),
                );
                self.mint_item = Some(MintItem::SimpleScriptMint(simple_script_mint));
            }
        }
        self
    }

    /// ## Transaction building method
    ///
    /// Set the minting redeemer value in the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `redeemer` - The redeemer value
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    pub fn mint_redeemer_value(&mut self, redeemer: WRedeemer) -> &mut Self {
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
                        ex_units: redeemer.ex_units,
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
    /// Set the minting reference transaction input redeemer value in the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `redeemer` - The redeemer value
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    pub fn mint_reference_tx_in_redeemer_value(&mut self, redeemer: WRedeemer) -> &mut Self {
        self.mint_redeemer_value(redeemer)
    }
}
