use sidan_csl_rs::model::*;

use super::{MeshTxBuilder, WRedeemer};

impl MeshTxBuilder {
    /// ## Transaction building method
    ///
    /// Indicate that the transaction is withdrawing using a plutus staking script in the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `language_version` - The language version of the script
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    pub fn withdrawal_plutus_script(&mut self, language_version: &LanguageVersion) -> &mut Self {
        match language_version {
            LanguageVersion::V1 => self.withdrawal_plutus_script_v1(),
            LanguageVersion::V2 => self.withdrawal_plutus_script_v2(),
            LanguageVersion::V3 => self.withdrawal_plutus_script_v3(),
        }
    }

    /// ## Transaction building method
    ///
    /// Indicate that the transaction is withdrawing using a plutus V1 staking script in the MeshTxBuilder instance
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    pub fn withdrawal_plutus_script_v1(&mut self) -> &mut Self {
        self.adding_plutus_withdrawal = Some(LanguageVersion::V1);
        self
    }

    /// ## Transaction building method
    ///
    /// Indicate that the transaction is withdrawing using a plutus V2 staking script in the MeshTxBuilder instance
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    pub fn withdrawal_plutus_script_v2(&mut self) -> &mut Self {
        self.adding_plutus_withdrawal = Some(LanguageVersion::V2);
        self
    }

    /// ## Transaction building method
    ///
    /// Indicate that the transaction is withdrawing using a plutus V3 staking script in the MeshTxBuilder instance
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    pub fn withdrawal_plutus_script_v3(&mut self) -> &mut Self {
        self.adding_plutus_withdrawal = Some(LanguageVersion::V3);
        self
    }

    /// ## Transaction building method
    ///
    /// Add a withdrawal reference to the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `tx_hash` - The transaction hash
    /// * `tx_index` - The transaction index
    /// * `withdrawal_script_hash` - The withdrawal script hash
    /// * `version` - The language version, if the language version is None, the script is assumed to be a Native Script
    /// * `script_size` - Size of the script
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    pub fn withdrawal_tx_in_reference(
        &mut self,
        tx_hash: &str,
        tx_index: u32,
        withdrawal_script_hash: &str,
        script_size: usize,
    ) -> &mut Self {
        let withdrawal_item = self.withdrawal_item.take();
        if withdrawal_item.is_none() {
            panic!("Undefined output")
        }
        let withdrawal_item = withdrawal_item.unwrap();
        match withdrawal_item {
            Withdrawal::PubKeyWithdrawal(_) => {
                panic!("Script reference cannot be defined for a pubkey withdrawal")
            }
            Withdrawal::SimpleScriptWithdrawal(mut withdrawal) => {
                withdrawal.script_source = Some(SimpleScriptSource::InlineSimpleScriptSource(
                    InlineSimpleScriptSource {
                        ref_tx_in: RefTxIn {
                            tx_hash: tx_hash.to_string(),
                            tx_index,
                        },
                        simple_script_hash: withdrawal_script_hash.to_string(),
                        script_size,
                    },
                ))
            }
            Withdrawal::PlutusScriptWithdrawal(mut withdrawal) => {
                withdrawal.script_source =
                    Some(ScriptSource::InlineScriptSource(InlineScriptSource {
                        ref_tx_in: RefTxIn {
                            tx_hash: tx_hash.to_string(),
                            tx_index,
                        },
                        script_hash: withdrawal_script_hash.to_string(),
                        language_version: self
                            .adding_plutus_withdrawal
                            .clone()
                            .expect("Plutus withdrawals require a language version"),
                        script_size,
                    }));
                self.withdrawal_item = Some(Withdrawal::PlutusScriptWithdrawal(withdrawal));
            }
        }
        self
    }

    /// ## Transaction building method
    ///
    /// Withdraw stake rewards in the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `stake_address` - The address corresponding to the stake key
    /// * `coin` - The amount of lovelaces in the withdrawal
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    pub fn withdrawal(&mut self, stake_address: &str, coin: u64) -> &mut Self {
        if self.withdrawal_item.is_some() {
            self.queue_withdrawal();
        }

        match self.adding_plutus_withdrawal {
            Some(_) => {
                let withdrawal_item = Withdrawal::PlutusScriptWithdrawal(PlutusScriptWithdrawal {
                    address: stake_address.to_string(),
                    coin,
                    script_source: None,
                    redeemer: None,
                });
                self.withdrawal_item = Some(withdrawal_item);
            }
            None => {
                let withdrawal_item = Withdrawal::PubKeyWithdrawal(PubKeyWithdrawal {
                    address: stake_address.to_string(),
                    coin,
                });
                self.withdrawal_item = Some(withdrawal_item);
            }
        }
        self
    }

    /// ## Transaction building method
    ///
    /// Add a withdrawal script to the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `script_cbor` - The script in CBOR format
    /// * `version` - The language version, if the language version is None, the script is assumed to be a Native Script
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    pub fn withdrawal_script(&mut self, script_cbor: &str) -> &mut Self {
        let withdrawal_item = self.withdrawal_item.take();
        if withdrawal_item.is_none() {
            panic!("Undefined withdrawal")
        }
        let withdrawal_item = withdrawal_item.unwrap();
        match withdrawal_item {
            Withdrawal::PubKeyWithdrawal(_) => {
                panic!("Script cannot be defined for a pubkey withdrawal")
            }
            Withdrawal::SimpleScriptWithdrawal(mut withdraw) => {
                withdraw.script_source = Some(SimpleScriptSource::ProvidedSimpleScriptSource(
                    ProvidedSimpleScriptSource {
                        script_cbor: script_cbor.to_string(),
                    },
                ))
            }
            Withdrawal::PlutusScriptWithdrawal(mut withdraw) => {
                withdraw.script_source =
                    Some(ScriptSource::ProvidedScriptSource(ProvidedScriptSource {
                        script_cbor: script_cbor.to_string(),
                        language_version: self
                            .adding_plutus_withdrawal
                            .clone()
                            .expect("Plutus withdrawals require a language version"),
                    }));
                self.withdrawal_item = Some(Withdrawal::PlutusScriptWithdrawal(withdraw));
                self.adding_plutus_withdrawal = None;
            }
        }
        self
    }

    /// ## Transaction building method
    ///
    /// Set the transaction withdrawal redeemer value in the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `redeemer` - The redeemer value
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    pub fn withdrawal_redeemer_value(&mut self, redeemer: &WRedeemer) -> &mut Self {
        let withdrawal_item = self.withdrawal_item.take();
        if withdrawal_item.is_none() {
            panic!("Undefined input")
        }
        let withdrawal_item = withdrawal_item.unwrap();
        match withdrawal_item {
            Withdrawal::PubKeyWithdrawal(_) => {
                panic!("Redeemer cannot be defined for a pubkey withdrawal")
            }
            Withdrawal::SimpleScriptWithdrawal(_) => {
                panic!("Redeemer cannot be defined for a native script withdrawal")
            }
            Withdrawal::PlutusScriptWithdrawal(mut withdraw) => match redeemer.data.to_cbor() {
                Ok(raw_redeemer) => {
                    withdraw.redeemer = Some(Redeemer {
                        data: raw_redeemer,
                        ex_units: redeemer.clone().ex_units,
                    });
                    self.withdrawal_item = Some(Withdrawal::PlutusScriptWithdrawal(withdraw));
                }
                Err(_) => panic!("Error converting redeemer to CBOR"),
            },
        }
        self
    }

    /// ## Transaction building method
    ///
    /// Set the withdrawal reference redeemer value in the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `redeemer` - The redeemer value
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    pub fn withdrawal_reference_tx_in_redeemer_value(&mut self, redeemer: &WRedeemer) -> &mut Self {
        self.withdrawal_redeemer_value(redeemer)
    }
}
