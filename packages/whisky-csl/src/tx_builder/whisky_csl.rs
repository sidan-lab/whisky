use super::CoreCSL;
use whisky_common::*;

#[derive(Clone, Debug)]
pub struct WhiskyCSL {
    pub core: CoreCSL,
    pub tx_builder_body: TxBuilderBody,
    pub tx_evaluation_multiplier_percentage: u64,
    pub tx_hex: String,
}

impl WhiskyCSL {
    /// ## Transaction building method
    ///
    /// Create a new TxBuilder instance
    ///
    /// ### Returns
    ///
    /// * `Self` - A new TxBuilder instance
    ///
    pub fn new(params: Option<Protocol>) -> Result<Self, WError> {
        let whisky = WhiskyCSL {
            core: CoreCSL::new(params)?,
            tx_builder_body: TxBuilderBody::new(),
            tx_evaluation_multiplier_percentage: 110,
            tx_hex: String::new(),
        };

        Ok(whisky)
    }

    /// ## Internal method
    ///
    /// Add multiple signing keys to the TxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `signing_keys` - A vector of signing keys in hexadecimal
    pub fn add_all_signing_keys(&mut self, signing_keys: &[&str]) -> Result<(), WError> {
        if !signing_keys.is_empty() {
            self.core.add_signing_keys(signing_keys)?;
        }
        Ok(())
    }

    /// ## Internal method
    ///
    /// Add multiple inputs to the TxBuilder instance
    ///
    pub fn add_all_inputs(&mut self) -> Result<&mut Self, WError> {
        let inputs = self.tx_builder_body.inputs.clone();
        for input in inputs {
            match input {
                TxIn::PubKeyTxIn(pub_key_tx_in) => self.core.add_tx_in(pub_key_tx_in)?,
                TxIn::SimpleScriptTxIn(simple_script_tx_in) => {
                    self.core.add_simple_script_tx_in(simple_script_tx_in)?
                }
                TxIn::ScriptTxIn(script_tx_in) => self.core.add_script_tx_in(script_tx_in)?,
            };
        }
        self.core
            .tx_builder
            .set_inputs(&self.core.tx_inputs_builder);
        Ok(self)
    }

    /// ## Internal method
    ///
    /// Add multiple outputs to the TxBuilder instance
    pub fn add_all_outputs(&mut self) -> Result<&mut Self, WError> {
        let outputs = self.tx_builder_body.outputs.clone();
        for output in outputs {
            self.core.add_output(output)?;
        }
        Ok(self)
    }

    /// ## Internal method
    ///
    /// Add multiple collaterals to the TxBuilder instance
    pub fn add_all_collaterals(&mut self) -> Result<&mut Self, WError> {
        let collaterals = self.tx_builder_body.collaterals.clone();
        for collateral in collaterals {
            self.core.add_collateral(collateral)?
        }
        self.core
            .tx_builder
            .set_collateral(&self.core.collateral_builder);
        Ok(self)
    }

    /// ## Internal method
    ///
    /// Add multiple reference inputs to the TxBuilder instance
    pub fn add_all_reference_inputs(&mut self) -> Result<&mut Self, WError> {
        let ref_inputs = self.tx_builder_body.reference_inputs.clone();
        for ref_input in ref_inputs {
            self.core.add_reference_input(ref_input)?;
        }
        Ok(self)
    }

    /// ## Internal method
    ///
    /// Add multiple withdrawals to the TxBuilder instance
    pub fn add_all_withdrawals(&mut self) -> Result<&mut Self, WError> {
        let withdrawals = self.tx_builder_body.withdrawals.clone();
        for withdrawal in withdrawals {
            match withdrawal {
                Withdrawal::PubKeyWithdrawal(pub_key_withdrawal) => {
                    self.core.add_pub_key_withdrawal(pub_key_withdrawal)?
                }
                Withdrawal::PlutusScriptWithdrawal(plutus_script_withdrawal) => {
                    self.core.add_plutus_withdrawal(plutus_script_withdrawal)?
                }
                Withdrawal::SimpleScriptWithdrawal(simple_script_withdrawal) => self
                    .core
                    .add_simple_script_withdrawal(simple_script_withdrawal)?,
            }
        }
        self.core
            .tx_builder
            .set_withdrawals_builder(&self.core.tx_withdrawals_builder);
        Ok(self)
    }

    /// ## Internal method
    ///
    /// Add multiple mints to the TxBuilder instance
    pub fn add_all_mints(&mut self) -> Result<&mut Self, WError> {
        let mints = self.tx_builder_body.mints.clone();
        for (index, mint) in mints.into_iter().enumerate() {
            match mint {
                MintItem::ScriptMint(script_mint) => {
                    self.core.add_plutus_mint(script_mint, index as u64)?
                }
                MintItem::SimpleScriptMint(simple_script_mint) => {
                    self.core.add_native_mint(simple_script_mint)?
                }
            };
        }
        self.core
            .tx_builder
            .set_mint_builder(&self.core.mint_builder);
        Ok(self)
    }

    /// ## Internal method
    ///
    /// Add multiple certificates to the TxBuilder instance
    pub fn add_all_certificates(&mut self) -> Result<&mut Self, WError> {
        let certificates = self.tx_builder_body.certificates.clone();
        for (index, cert) in certificates.into_iter().enumerate() {
            self.core.add_cert(cert, index as u64)?
        }
        self.core
            .tx_builder
            .set_certs_builder(&self.core.certificates_builder);
        Ok(self)
    }

    /// ## Internal method
    ///
    /// Add multiple votes to the TxBuilder instance
    pub fn add_all_votes(&mut self) -> Result<&mut Self, WError> {
        let votes = self.tx_builder_body.votes.clone();
        for (index, vote) in votes.into_iter().enumerate() {
            self.core.add_vote(vote, index as u64)?
        }
        self.core
            .tx_builder
            .set_voting_builder(&self.core.vote_builder);
        Ok(self)
    }

    /// ## Internal method
    ///
    /// Add a validity range to the TxBuilder instance
    pub fn add_validity_range(&mut self) -> Result<&mut Self, WError> {
        let validity_range = self.tx_builder_body.validity_range.clone();
        if validity_range.invalid_before.is_some() {
            self.core
                .add_invalid_before(validity_range.invalid_before.unwrap())?;
        };
        if validity_range.invalid_hereafter.is_some() {
            self.core
                .add_invalid_hereafter(validity_range.invalid_hereafter.unwrap())?;
        };
        Ok(self)
    }

    /// ## Internal method
    ///
    /// Add multiple required signatures to the TxBuilder instance
    pub fn add_all_required_signature(&mut self) -> Result<&mut Self, WError> {
        let required_signatures = self
            .tx_builder_body
            .required_signatures
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<&str>>();
        for pub_key_hash in required_signatures {
            self.core
                .add_required_signature(pub_key_hash)
                .map_err(WError::from_err("add_all_required_signature"))?;
        }
        Ok(self)
    }

    /// ## Internal method
    ///
    /// Add multiple metadata to the TxBuilder instance
    pub fn add_all_metadata(&mut self) -> Result<&mut Self, WError> {
        let all_metadata = self.tx_builder_body.metadata.clone();
        for metadata in all_metadata {
            self.core
                .add_metadata(metadata)
                .map_err(WError::from_err("add_all_metadata"))?;
        }
        Ok(self)
    }

    /// ## Internal method
    ///
    /// Add a script hash to the TxBuilder instance
    pub fn add_script_hash(&mut self) -> Result<&mut Self, WError> {
        match self.tx_builder_body.network.clone() {
            Some(current_network) => self.core.add_script_hash(current_network)?,
            None => self.core.add_script_hash(Network::Mainnet)?,
        };
        Ok(self)
    }

    /// ## Internal method
    ///
    /// Set the fee if needed
    pub fn set_fee_if_needed(&mut self) -> Result<&mut Self, WError> {
        if self.tx_builder_body.fee.is_some() {
            self.set_fee(self.tx_builder_body.fee.clone().unwrap());
        }
        Ok(self)
    }

    pub fn add_change_utxo(&mut self) -> Result<&mut Self, WError> {
        self.core.add_change(
            self.tx_builder_body.change_address.clone(),
            self.tx_builder_body.change_datum.clone(),
        )?;
        Ok(self)
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

    pub fn set_fee(&mut self, fee: String) {
        self.core.set_fee(fee);
    }
}

// if self.tx_builder_body.change_address != "" {
//     let collateral_inputs = self.tx_builder_body.collaterals.clone();
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
//     &csl::address::Address::from_bech32(&self.tx_builder_body.change_address)
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
//                 &self.tx_builder_body.change_address,
//             )
//             .unwrap(),
//             &csl::utils::Value::new(&to_bignum(total_collateral)),
//         ));

//     self.tx_builder
//         .set_total_collateral(&to_bignum(total_collateral));

//     collateral_return_needed = true;
// }
// }
// self.add_change(self.tx_builder_body.change_address.clone());
// if collateral_return_needed {
//     self.add_collateral_return(self.tx_builder_body.change_address.clone());
// }
// }
