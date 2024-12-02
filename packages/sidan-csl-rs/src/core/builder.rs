use super::core_csl::MeshCSL;
use crate::{csl, model::*};
use cardano_serialization_lib::JsError;

#[derive(Clone, Debug)]
pub struct TxBuilderCore {
    pub mesh_csl: MeshCSL,
    pub tx_builder_body: TxBuilderBody,
    pub tx_evaluation_multiplier_percentage: u64,
}

/// ## Transaction building method
///
/// Serialize the transaction body
///
/// ### Arguments
///
/// * `tx_builder_body` - The transaction builder body information
/// * `params` - Optional protocol parameters, default as Cardano mainnet configuration
///
/// ### Returns
///
/// * `String` - the built transaction hex
pub fn serialize_tx_body(
    tx_builder_body: TxBuilderBody,
    params: Option<Protocol>,
) -> Result<String, JsError> {
    if tx_builder_body.change_address.is_empty() {
        return Err(JsError::from_str("change address cannot be empty"));
    }
    let mut mesh_csl = MeshCSL::new(params);

    TxBuilderCore::add_all_inputs(&mut mesh_csl, tx_builder_body.inputs.clone())?;
    TxBuilderCore::add_all_outputs(&mut mesh_csl, tx_builder_body.outputs.clone())?;
    TxBuilderCore::add_all_collaterals(&mut mesh_csl, tx_builder_body.collaterals.clone())?;
    TxBuilderCore::add_all_reference_inputs(
        &mut mesh_csl,
        tx_builder_body.reference_inputs.clone(),
    )?;
    TxBuilderCore::add_all_withdrawals(&mut mesh_csl, tx_builder_body.withdrawals.clone())?;
    TxBuilderCore::add_all_mints(&mut mesh_csl, tx_builder_body.mints.clone())?;
    TxBuilderCore::add_all_certificates(&mut mesh_csl, tx_builder_body.certificates.clone())?;
    TxBuilderCore::add_all_votes(&mut mesh_csl, tx_builder_body.votes.clone())?;
    TxBuilderCore::add_validity_range(&mut mesh_csl, tx_builder_body.validity_range.clone());
    TxBuilderCore::add_all_required_signature(
        &mut mesh_csl,
        &tx_builder_body
            .required_signatures
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<&str>>(),
    )?;
    TxBuilderCore::add_all_metadata(&mut mesh_csl, tx_builder_body.metadata.clone())?;

    match tx_builder_body.network {
        Some(current_network) => mesh_csl.add_script_hash(current_network)?,
        None => mesh_csl.add_script_hash(Network::Mainnet)?,
    };
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
    if tx_builder_body.fee.is_some() {
        mesh_csl.set_fee(tx_builder_body.fee.unwrap());
    }
    mesh_csl.add_change(
        tx_builder_body.change_address.clone(),
        tx_builder_body.change_datum.clone(),
    )?;
    mesh_csl.build_tx()
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
            mesh_csl: MeshCSL::new(params),
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
    pub fn complete_signing(&mut self) -> Result<String, JsError> {
        let signing_keys = self.tx_builder_body.signing_key.clone();
        self.add_all_signing_keys(
            &signing_keys
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<&str>>(),
        )?;
        Ok(self.mesh_csl.tx_hex.to_string())
    }

    /// ## Internal method
    ///
    /// Add multiple signing keys to the TxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `signing_keys` - A vector of signing keys in hexadecimal
    fn add_all_signing_keys(&mut self, signing_keys: &[&str]) -> Result<(), JsError> {
        if !signing_keys.is_empty() {
            self.mesh_csl.add_signing_keys(signing_keys)?;
        }
        Ok(())
    }

    /// ## Internal method
    ///
    /// Add multiple inputs to the TxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `mesh_csl` - The MeshCSL instance
    /// * `inputs` - A vector of inputs
    fn add_all_inputs(mesh_csl: &mut MeshCSL, inputs: Vec<TxIn>) -> Result<(), JsError> {
        for input in inputs {
            match input {
                TxIn::PubKeyTxIn(pub_key_tx_in) => mesh_csl.add_tx_in(pub_key_tx_in)?,
                TxIn::SimpleScriptTxIn(simple_script_tx_in) => {
                    mesh_csl.add_simple_script_tx_in(simple_script_tx_in)?
                }
                TxIn::ScriptTxIn(script_tx_in) => mesh_csl.add_script_tx_in(script_tx_in)?,
            };
        }
        mesh_csl.tx_builder.set_inputs(&mesh_csl.tx_inputs_builder);
        Ok(())
    }

    /// ## Internal method
    ///
    /// Add multiple outputs to the TxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `mesh_csl` - The MeshCSL instance
    /// * `outputs` - A vector of outputs
    fn add_all_outputs(mesh_csl: &mut MeshCSL, outputs: Vec<Output>) -> Result<(), JsError> {
        for output in outputs {
            mesh_csl.add_output(output)?;
        }
        Ok(())
    }

    /// ## Internal method
    ///
    /// Add multiple collaterals to the TxBuilder instance
    ///
    /// ## Arguments
    ///
    /// * `mesh_csl` - The MeshCSL instance
    /// * `collaterals` - A vector of collaterals
    fn add_all_collaterals(
        mesh_csl: &mut MeshCSL,
        collaterals: Vec<PubKeyTxIn>,
    ) -> Result<(), JsError> {
        let mut collateral_builder = csl::TxInputsBuilder::new();
        for collateral in collaterals {
            mesh_csl.add_collateral(&mut collateral_builder, collateral)?
        }
        mesh_csl.tx_builder.set_collateral(&collateral_builder);
        Ok(())
    }

    /// ## Internal method
    ///
    /// Add multiple reference inputs to the TxBuilder instance
    ///
    /// ## Arguments
    ///
    /// * `mesh_csl` - The MeshCSL instance
    /// * `ref_inputs` - A vector of reference inputs
    fn add_all_reference_inputs(
        mesh_csl: &mut MeshCSL,
        ref_inputs: Vec<RefTxIn>,
    ) -> Result<(), JsError> {
        for ref_input in ref_inputs {
            mesh_csl.add_reference_input(ref_input)?;
        }
        Ok(())
    }

    /// ## Internal method
    ///
    /// Add multiple withdrawals to the TxBuilder instance
    ///
    /// ## Arguments
    ///
    /// * `mesh_csl` - The MeshCSL instance
    /// * `withdrawals` - A vector of withdrawals
    fn add_all_withdrawals(
        mesh_csl: &mut MeshCSL,
        withdrawals: Vec<Withdrawal>,
    ) -> Result<(), JsError> {
        for withdrawal in withdrawals {
            match withdrawal {
                Withdrawal::PubKeyWithdrawal(pub_key_withdrawal) => {
                    mesh_csl.add_pub_key_withdrawal(pub_key_withdrawal)?
                }
                Withdrawal::PlutusScriptWithdrawal(plutus_script_withdrawal) => {
                    mesh_csl.add_plutus_withdrawal(plutus_script_withdrawal)?
                }
                Withdrawal::SimpleScriptWithdrawal(simple_script_withdrawal) => {
                    mesh_csl.add_simple_script_withdrawal(simple_script_withdrawal)?
                }
            }
        }
        mesh_csl
            .tx_builder
            .set_withdrawals_builder(&mesh_csl.tx_withdrawals_builder);
        Ok(())
    }

    /// ## Internal method
    ///
    /// Add multiple mints to the TxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `mesh_csl` - The MeshCSL instance
    /// * `mints` - A vector of mints
    fn add_all_mints(mesh_csl: &mut MeshCSL, mints: Vec<MintItem>) -> Result<(), JsError> {
        let mut mint_builder = csl::MintBuilder::new();
        for (index, mint) in mints.into_iter().enumerate() {
            match mint {
                MintItem::ScriptMint(script_mint) => {
                    mesh_csl.add_plutus_mint(&mut mint_builder, script_mint, index as u64)?
                }
                MintItem::SimpleScriptMint(simple_script_mint) => {
                    mesh_csl.add_native_mint(&mut mint_builder, simple_script_mint)?
                }
            };
        }
        mesh_csl.tx_builder.set_mint_builder(&mint_builder);
        Ok(())
    }

    /// ## Internal method
    ///
    /// Add multiple certificates to the TxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `mesh_csl` - The MeshCSL instance
    /// * `certificates` - A vector of certificates
    fn add_all_certificates(
        mesh_csl: &mut MeshCSL,
        certificates: Vec<Certificate>,
    ) -> Result<(), JsError> {
        let mut certificates_builder = csl::CertificatesBuilder::new();
        for (index, cert) in certificates.into_iter().enumerate() {
            mesh_csl.add_cert(&mut certificates_builder, cert, index as u64)?
        }
        mesh_csl.tx_builder.set_certs_builder(&certificates_builder);
        Ok(())
    }

    /// ## Internal method
    ///
    /// Add multiple votes to the TxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `mesh_csl` - The MeshCSL instance
    /// * `votes` - A vector of votes
    fn add_all_votes(mesh_csl: &mut MeshCSL, votes: Vec<Vote>) -> Result<(), JsError> {
        let mut vote_builder = csl::VotingBuilder::new();
        for (index, vote) in votes.into_iter().enumerate() {
            mesh_csl.add_vote(&mut vote_builder, vote, index as u64)?
        }
        mesh_csl.tx_builder.set_voting_builder(&vote_builder);
        Ok(())
    }

    /// ## Internal method
    ///
    /// Add a validity range to the TxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `mesh_csl` - The MeshCSL instance
    /// * `validity_range` - The validity range
    fn add_validity_range(mesh_csl: &mut MeshCSL, validity_range: ValidityRange) {
        if validity_range.invalid_before.is_some() {
            mesh_csl.add_invalid_before(validity_range.invalid_before.unwrap())
        }
        if validity_range.invalid_hereafter.is_some() {
            mesh_csl.add_invalid_hereafter(validity_range.invalid_hereafter.unwrap())
        }
    }

    /// ## Internal method
    ///
    /// Add multiple required signatures to the TxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `mesh_csl` - The MeshCSL instance
    /// * `required_signatures` - A vector of required signatures
    fn add_all_required_signature(
        mesh_csl: &mut MeshCSL,
        required_signatures: &[&str],
    ) -> Result<(), JsError> {
        for pub_key_hash in required_signatures {
            mesh_csl.add_required_signature(pub_key_hash)?;
        }
        Ok(())
    }

    /// ## Internal method
    ///
    /// Add multiple metadata to the TxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `mesh_csl` - The MeshCSL instance
    /// * `all_metadata` - A vector of metadata
    fn add_all_metadata(
        mesh_csl: &mut MeshCSL,
        all_metadata: Vec<Metadata>,
    ) -> Result<(), JsError> {
        for metadata in all_metadata {
            mesh_csl.add_metadata(metadata)?;
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

    fn set_fee(mesh_csl: &mut MeshCSL, fee: String) {
        mesh_csl.set_fee(fee);
    }
}

impl Default for TxBuilderCore {
    fn default() -> Self {
        Self::new_core(None)
    }
}
