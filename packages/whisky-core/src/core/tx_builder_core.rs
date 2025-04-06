use super::core_csl::WhiskyCSL;
use crate::model::*;

#[derive(Clone, Debug)]
pub struct TxBuilderCore {
    pub serializer: TxBuildable,
    pub tx_builder_body: TxBuilderBody,
    pub tx_evaluation_multiplier_percentage: u64,
}

impl TxBuilderCore {
    /// ## Transaction building method
    ///
    /// Create a new TxBuilder instance
    ///
    /// ### Returns
    ///
    /// * `Self` - A new TxBuilder instance
    ///
    pub fn new_core(params: Option<Protocol>) -> Self {
        Self {
            serializer: WhiskyCSL::new(params),
            tx_builder_body: TxBuilderBody {
                inputs: vec![],
                outputs: vec![],
                collaterals: vec![],
                required_signatures: vec![],
                reference_inputs: vec![],
                withdrawals: vec![],
                mints: vec![],
                change_address: "".to_string(),
                change_datum: None,
                certificates: vec![],
                votes: vec![],
                metadata: vec![],
                validity_range: ValidityRange {
                    invalid_before: None,
                    invalid_hereafter: None,
                },
                signing_key: vec![],
                fee: None,
                network: None,
            },
            tx_evaluation_multiplier_percentage: 110,
        }
    }

    /// ## Transaction building method
    ///
    /// Complete the signing process
    ///
    /// ### Returns
    ///
    /// * `String` - The signed transaction in hex
    pub fn complete_signing(&mut self) -> Result<String, WError> {
        let signing_keys = self.tx_builder_body.signing_key.clone();
        self.add_all_signing_keys(
            &signing_keys
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<&str>>(),
        )?;
        Ok(self.whisky_csl.tx_hex.to_string())
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
            self.whisky_csl.add_signing_keys(signing_keys)?;
        }
        Ok(())
    }

    /// ## Internal method
    ///
    /// Add multiple inputs to the TxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `whisky_csl` - The WhiskyCSL instance
    /// * `inputs` - A vector of inputs
    pub fn add_all_inputs(whisky_csl: &mut WhiskyCSL, inputs: Vec<TxIn>) -> Result<(), WError> {
        for input in inputs {
            match input {
                TxIn::PubKeyTxIn(pub_key_tx_in) => whisky_csl.add_tx_in(pub_key_tx_in)?,
                TxIn::SimpleScriptTxIn(simple_script_tx_in) => {
                    whisky_csl.add_simple_script_tx_in(simple_script_tx_in)?
                }
                TxIn::ScriptTxIn(script_tx_in) => whisky_csl.add_script_tx_in(script_tx_in)?,
            };
        }
        whisky_csl
            .tx_builder
            .set_inputs(&whisky_csl.tx_inputs_builder);
        Ok(())
    }

    /// ## Internal method
    ///
    /// Add multiple outputs to the TxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `whisky_csl` - The WhiskyCSL instance
    /// * `outputs` - A vector of outputs
    pub fn add_all_outputs(whisky_csl: &mut WhiskyCSL, outputs: Vec<Output>) -> Result<(), WError> {
        for output in outputs {
            whisky_csl.add_output(output)?;
        }
        Ok(())
    }

    /// ## Internal method
    ///
    /// Add multiple collaterals to the TxBuilder instance
    ///
    /// ## Arguments
    ///
    /// * `whisky_csl` - The WhiskyCSL instance
    /// * `collaterals` - A vector of collaterals
    pub fn add_all_collaterals(
        whisky_csl: &mut WhiskyCSL,
        collaterals: Vec<PubKeyTxIn>,
    ) -> Result<(), WError> {
        let mut collateral_builder = csl::TxInputsBuilder::new();
        for collateral in collaterals {
            whisky_csl.add_collateral(&mut collateral_builder, collateral)?
        }
        whisky_csl.tx_builder.set_collateral(&collateral_builder);
        Ok(())
    }

    /// ## Internal method
    ///
    /// Add multiple reference inputs to the TxBuilder instance
    ///
    /// ## Arguments
    ///
    /// * `whisky_csl` - The WhiskyCSL instance
    /// * `ref_inputs` - A vector of reference inputs
    pub fn add_all_reference_inputs(
        whisky_csl: &mut WhiskyCSL,
        ref_inputs: Vec<RefTxIn>,
    ) -> Result<(), WError> {
        for ref_input in ref_inputs {
            whisky_csl.add_reference_input(ref_input)?;
        }
        Ok(())
    }

    /// ## Internal method
    ///
    /// Add multiple withdrawals to the TxBuilder instance
    ///
    /// ## Arguments
    ///
    /// * `whisky_csl` - The WhiskyCSL instance
    /// * `withdrawals` - A vector of withdrawals
    pub fn add_all_withdrawals(
        whisky_csl: &mut WhiskyCSL,
        withdrawals: Vec<Withdrawal>,
    ) -> Result<(), WError> {
        for withdrawal in withdrawals {
            match withdrawal {
                Withdrawal::PubKeyWithdrawal(pub_key_withdrawal) => {
                    whisky_csl.add_pub_key_withdrawal(pub_key_withdrawal)?
                }
                Withdrawal::PlutusScriptWithdrawal(plutus_script_withdrawal) => {
                    whisky_csl.add_plutus_withdrawal(plutus_script_withdrawal)?
                }
                Withdrawal::SimpleScriptWithdrawal(simple_script_withdrawal) => {
                    whisky_csl.add_simple_script_withdrawal(simple_script_withdrawal)?
                }
            }
        }
        whisky_csl
            .tx_builder
            .set_withdrawals_builder(&whisky_csl.tx_withdrawals_builder);
        Ok(())
    }

    /// ## Internal method
    ///
    /// Add multiple mints to the TxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `whisky_csl` - The WhiskyCSL instance
    /// * `mints` - A vector of mints
    pub fn add_all_mints(whisky_csl: &mut WhiskyCSL, mints: Vec<MintItem>) -> Result<(), WError> {
        for (index, mint) in mints.into_iter().enumerate() {
            match mint {
                MintItem::ScriptMint(script_mint) => {
                    whisky_csl.add_plutus_mint(script_mint, index as u64)?
                }
                MintItem::SimpleScriptMint(simple_script_mint) => {
                    whisky_csl.add_native_mint(simple_script_mint)?
                }
            };
        }
        whisky_csl.tx_builder.set_mint_builder(&mint_builder);
        Ok(())
    }

    /// ## Internal method
    ///
    /// Add multiple certificates to the TxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `whisky_csl` - The WhiskyCSL instance
    /// * `certificates` - A vector of certificates
    pub fn add_all_certificates(
        whisky_csl: &mut WhiskyCSL,
        certificates: Vec<Certificate>,
    ) -> Result<(), WError> {
        let mut certificates_builder = csl::CertificatesBuilder::new();
        for (index, cert) in certificates.into_iter().enumerate() {
            whisky_csl.add_cert(&mut certificates_builder, cert, index as u64)?
        }
        whisky_csl
            .tx_builder
            .set_certs_builder(&certificates_builder);
        Ok(())
    }

    /// ## Internal method
    ///
    /// Add multiple votes to the TxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `whisky_csl` - The WhiskyCSL instance
    /// * `votes` - A vector of votes
    pub fn add_all_votes(whisky_csl: &mut WhiskyCSL, votes: Vec<Vote>) -> Result<(), WError> {
        let mut vote_builder = csl::VotingBuilder::new();
        for (index, vote) in votes.into_iter().enumerate() {
            whisky_csl.add_vote(&mut vote_builder, vote, index as u64)?
        }
        whisky_csl.tx_builder.set_voting_builder(&vote_builder);
        Ok(())
    }

    /// ## Internal method
    ///
    /// Add a validity range to the TxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `whisky_csl` - The WhiskyCSL instance
    /// * `validity_range` - The validity range
    pub fn add_validity_range(whisky_csl: &mut WhiskyCSL, validity_range: ValidityRange) {
        if validity_range.invalid_before.is_some() {
            whisky_csl.add_invalid_before(validity_range.invalid_before.unwrap())
        }
        if validity_range.invalid_hereafter.is_some() {
            whisky_csl.add_invalid_hereafter(validity_range.invalid_hereafter.unwrap())
        }
    }

    /// ## Internal method
    ///
    /// Add multiple required signatures to the TxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `whisky_csl` - The WhiskyCSL instance
    /// * `required_signatures` - A vector of required signatures
    pub fn add_all_required_signature(
        whisky_csl: &mut WhiskyCSL,
        required_signatures: &[&str],
    ) -> Result<(), WError> {
        for pub_key_hash in required_signatures {
            whisky_csl.add_required_signature(pub_key_hash)?;
        }
        Ok(())
    }

    /// ## Internal method
    ///
    /// Add multiple metadata to the TxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `whisky_csl` - The WhiskyCSL instance
    /// * `all_metadata` - A vector of metadata
    pub fn add_all_metadata(
        whisky_csl: &mut WhiskyCSL,
        all_metadata: Vec<Metadata>,
    ) -> Result<(), WError> {
        for metadata in all_metadata {
            whisky_csl.add_metadata(metadata)?;
        }
        Ok(())
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

    pub fn set_fee(whisky_csl: &mut WhiskyCSL, fee: String) {
        whisky_csl.set_fee(fee);
    }
}

impl Default for TxBuilderCore {
    fn default() -> Self {
        Self::new_core(None)
    }
}
