use cardano_serialization_lib::JsError;

use crate::{
    core::builder::{IMeshCSL, MeshCSL},
    csl,
    model::*,
    *,
};

use super::{interface::MeshTxBuilderCore, IMeshTxBuilderCore};

#[wasm_bindgen]
pub fn js_serialize_tx_body(mesh_tx_builder_body_json: &str, params_json: &str) -> String {
    let mesh_tx_builder_body: MeshTxBuilderBody = serde_json::from_str(mesh_tx_builder_body_json)
        .expect("Error deserializing transaction body");

    let params: Option<Protocol> = match serde_json::from_str(params_json) {
        Ok(params) => Some(params),
        Err(_) => None,
    };

    serialize_tx_body(mesh_tx_builder_body, params).unwrap()
}

// #[test]
// fn test_js_serialize_tx_body() {
//     let mesh_tx_builder_body_json = r#"{"inputs":[{"pubKeyTxIn":{"type":"PubKey","txIn":{"txHash":"1662c4b349907e4d92e0995fd9dcdc9a4489f7dff4f5cce6b4b3901de479308c","txIndex":14,"amount":[{"unit":"lovelace","quantity":"774643176"}],"address":"addr_test1qq0yavv5uve45rwvfaw96qynrqt8ckpmkwcg08vlwxxdncxk82f5wz75mzaesmqzl79xqsmedwgucwtuav5str6untqqmykcpn"}}}],"outputs":[{"address":"addr_test1qq0yavv5uve45rwvfaw96qynrqt8ckpmkwcg08vlwxxdncxk82f5wz75mzaesmqzl79xqsmedwgucwtuav5str6untqqmykcpn","amount":[{"unit":"lovelace","quantity":"1231231"}],"datum":null,"referenceScript":null}],"collaterals":[],"requiredSignatures":[],"referenceInputs":[],"mints":[],"changeAddress":"addr_test1qq0yavv5uve45rwvfaw96qynrqt8ckpmkwcg08vlwxxdncxk82f5wz75mzaesmqzl79xqsmedwgucwtuav5str6untqqmykcpn","metadata":[],"validityRange":{"invalidBefore":null,"invalidHereafter":null},"certificates":[],"signingKey":[],"withdrawals":[]}"#;
//     let params_json = r#"{"epoch":0,"coinsPerUTxOSize":"4310","priceMem":0.0577,"priceStep":0.0000721,"minFeeA":44,"minFeeB":155381,"keyDeposit":"2000000","maxTxSize":16384,"maxValSize":"5000","poolDeposit":"500000000","maxCollateralInputs":3,"decentralisation":0,"maxBlockSize":98304,"collateralPercent":150,"maxBlockHeaderSize":1100,"minPoolCost":"340000000","maxTxExMem":"16000000","maxTxExSteps":"10000000000","maxBlockExMem":"80000000","maxBlockExSteps":"40000000000"}"#;
//     let tx_hex = js_serialize_tx_body(mesh_tx_builder_body_json, params_json);
//     println!("tx_hex: {:?}", tx_hex);
// }

/// ## Transaction building method
///
/// Serialize the transaction body
///
/// ### Arguments
///
/// * `mesh_tx_builder_body` - The transaction builder body information
///
/// ### Returns
///
/// * `String` - the built transaction hex
pub fn serialize_tx_body(
    mesh_tx_builder_body: MeshTxBuilderBody,
    params: Option<Protocol>,
) -> Result<String, JsError> {
    let mut mesh_csl = MeshCSL::new(params);

    MeshTxBuilderCore::add_all_inputs(&mut mesh_csl, mesh_tx_builder_body.inputs.clone())?;
    MeshTxBuilderCore::add_all_outputs(&mut mesh_csl, mesh_tx_builder_body.outputs.clone())?;
    MeshTxBuilderCore::add_all_collaterals(
        &mut mesh_csl,
        mesh_tx_builder_body.collaterals.clone(),
    )?;
    MeshTxBuilderCore::add_all_reference_inputs(
        &mut mesh_csl,
        mesh_tx_builder_body.reference_inputs.clone(),
    )?;
    MeshTxBuilderCore::add_all_withdrawals(
        &mut mesh_csl,
        mesh_tx_builder_body.withdrawals.clone(),
    )?;
    MeshTxBuilderCore::add_all_mints(&mut mesh_csl, mesh_tx_builder_body.mints.clone())?;
    MeshTxBuilderCore::add_all_certificates(
        &mut mesh_csl,
        mesh_tx_builder_body.certificates.clone(),
    )?;
    MeshTxBuilderCore::add_validity_range(
        &mut mesh_csl,
        mesh_tx_builder_body.validity_range.clone(),
    );
    MeshTxBuilderCore::add_all_required_signature(
        &mut mesh_csl,
        mesh_tx_builder_body.required_signatures.clone(),
    )?;
    MeshTxBuilderCore::add_all_metadata(&mut mesh_csl, mesh_tx_builder_body.metadata.clone())?;

    mesh_csl.add_script_hash()?;
    // if self.mesh_tx_builder_body.change_address != "" {
    //     let collateral_inputs = self.mesh_tx_builder_body.collaterals.clone();
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
    //     &csl::address::Address::from_bech32(&self.mesh_tx_builder_body.change_address)
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
    //                 &self.mesh_tx_builder_body.change_address,
    //             )
    //             .unwrap(),
    //             &csl::utils::Value::new(&to_bignum(total_collateral)),
    //         ));

    //     self.tx_builder
    //         .set_total_collateral(&to_bignum(total_collateral));

    //     collateral_return_needed = true;
    // }
    // }
    // self.add_change(self.mesh_tx_builder_body.change_address.clone());
    // if collateral_return_needed {
    //     self.add_collateral_return(self.mesh_tx_builder_body.change_address.clone());
    // }
    // }
    mesh_csl.add_change(
        mesh_tx_builder_body.change_address.clone(),
        mesh_tx_builder_body.change_datum.clone(),
    )?;
    mesh_csl.build_tx()
}

impl IMeshTxBuilderCore for MeshTxBuilderCore {
    fn new_core(params: Option<Protocol>) -> Self {
        Self {
            mesh_csl: MeshCSL::new(params),
            mesh_tx_builder_body: MeshTxBuilderBody {
                inputs: vec![],
                outputs: vec![],
                collaterals: vec![],
                required_signatures: JsVecString::new(),
                reference_inputs: vec![],
                withdrawals: vec![],
                mints: vec![],
                change_address: "".to_string(),
                change_datum: None,
                certificates: vec![],
                metadata: vec![],
                validity_range: ValidityRange {
                    invalid_before: None,
                    invalid_hereafter: None,
                },
                signing_key: JsVecString::new(),
            },
            tx_evaluation_multiplier_percentage: 110,
        }
    }

    fn complete_signing(&mut self) -> String {
        let signing_keys = self.mesh_tx_builder_body.signing_key.clone();
        self.add_all_signing_keys(signing_keys);
        self.mesh_csl.tx_hex.to_string()
    }

    fn add_all_signing_keys(&mut self, signing_keys: JsVecString) {
        if !signing_keys.is_empty() {
            self.mesh_csl.add_signing_keys(signing_keys);
        }
    }

    fn add_all_inputs(mesh_csl: &mut MeshCSL, inputs: Vec<TxIn>) -> Result<(), JsError> {
        for input in inputs {
            match input {
                TxIn::PubKeyTxIn(pub_key_tx_in) => mesh_csl.add_tx_in(pub_key_tx_in)?,
                TxIn::ScriptTxIn(script_tx_in) => mesh_csl.add_script_tx_in(script_tx_in)?,
            };
        }
        mesh_csl.tx_builder.set_inputs(&mesh_csl.tx_inputs_builder);
        Ok(())
    }

    fn add_all_outputs(mesh_csl: &mut MeshCSL, outputs: Vec<Output>) -> Result<(), JsError> {
        for output in outputs {
            mesh_csl.add_output(output)?;
        }
        Ok(())
    }

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

    fn add_all_reference_inputs(
        mesh_csl: &mut MeshCSL,
        ref_inputs: Vec<RefTxIn>,
    ) -> Result<(), JsError> {
        for ref_input in ref_inputs {
            mesh_csl.add_reference_input(ref_input)?;
        }
        Ok(())
    }

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
            }
        }
        mesh_csl
            .tx_builder
            .set_withdrawals_builder(&mesh_csl.tx_withdrawals_builder);
        Ok(())
    }

    fn add_all_mints(mesh_csl: &mut MeshCSL, mints: Vec<MintItem>) -> Result<(), JsError> {
        let mut mint_builder = csl::MintBuilder::new();
        for (index, mint) in mints.into_iter().enumerate() {
            match mint.type_.as_str() {
                "Plutus" => mesh_csl.add_plutus_mint(&mut mint_builder, mint, index as u64)?,
                "Native" => mesh_csl.add_native_mint(&mut mint_builder, mint)?,
                _ => {}
            };
        }
        mesh_csl.tx_builder.set_mint_builder(&mint_builder);
        Ok(())
    }

    fn add_all_certificates(
        mesh_csl: &mut MeshCSL,
        certificates: Vec<Certificate>,
    ) -> Result<(), JsError> {
        let mut certificates_builder = csl::CertificatesBuilder::new();
        for cert in certificates {
            match cert {
                Certificate::RegisterPool(register_pool) => {
                    mesh_csl.add_register_pool_cert(&mut certificates_builder, register_pool)?
                }
                Certificate::RegisterStake(register_stake) => {
                    mesh_csl.add_register_stake_cert(&mut certificates_builder, register_stake)?
                }
                Certificate::DelegateStake(delegate_stake) => {
                    mesh_csl.add_delegate_stake_cert(&mut certificates_builder, delegate_stake)?
                }
                Certificate::DeregisterStake(deregister_stake) => mesh_csl
                    .add_deregister_stake_cert(&mut certificates_builder, deregister_stake)?,
                Certificate::RetirePool(retire_pool) => {
                    mesh_csl.add_retire_pool_cert(&mut certificates_builder, retire_pool)?
                }
                Certificate::VoteDelegation(vote_delegation) => {
                    mesh_csl.add_vote_delegation_cert(&mut certificates_builder, vote_delegation)?
                }
                Certificate::StakeAndVoteDelegation(stake_and_vote_delegation) => mesh_csl
                    .add_stake_and_vote_delegation_cert(
                        &mut certificates_builder,
                        stake_and_vote_delegation,
                    )?,
                Certificate::StakeRegistrationAndDelegation(stake_registration_and_delegation) => {
                    mesh_csl.add_stake_registration_and_delegation_cert(
                        &mut certificates_builder,
                        stake_registration_and_delegation,
                    )?
                }
                Certificate::VoteRegistrationAndDelegation(vote_registration_and_delegation) => {
                    mesh_csl.add_vote_registration_and_delgation_cert(
                        &mut certificates_builder,
                        vote_registration_and_delegation,
                    )?
                }
                Certificate::StakeVoteRegistrationAndDelegation(
                    stake_vote_registration_and_delegation,
                ) => mesh_csl.add_stake_vote_registration_and_delegation_cert(
                    &mut certificates_builder,
                    stake_vote_registration_and_delegation,
                )?,
                Certificate::CommitteeHotAuth(committee_hot_auth) => mesh_csl
                    .add_committee_hot_auth_cert(&mut certificates_builder, committee_hot_auth)?,
                Certificate::CommitteeColdResign(commitee_cold_resign) => mesh_csl
                    .add_commitee_cold_resign_cert(
                        &mut certificates_builder,
                        commitee_cold_resign,
                    )?,
                Certificate::DRepRegistration(drep_registration) => mesh_csl
                    .add_drep_registration_cert(&mut certificates_builder, drep_registration)?,
                Certificate::DRepDeregistration(drep_deregistration) => mesh_csl
                    .add_drep_deregistration_cert(&mut certificates_builder, drep_deregistration)?,
                Certificate::DRepUpdate(drep_update) => {
                    mesh_csl.add_drep_update_cert(&mut certificates_builder, drep_update)?
                }
            }
        }
        mesh_csl.tx_builder.set_certs_builder(&certificates_builder);
        Ok(())
    }

    fn add_validity_range(mesh_csl: &mut MeshCSL, validity_range: ValidityRange) {
        if validity_range.invalid_before.is_some() {
            mesh_csl.add_invalid_before(validity_range.invalid_before.unwrap())
        }
        if validity_range.invalid_hereafter.is_some() {
            mesh_csl.add_invalid_hereafter(validity_range.invalid_hereafter.unwrap())
        }
    }

    fn add_all_required_signature(
        mesh_csl: &mut MeshCSL,
        required_signatures: JsVecString,
    ) -> Result<(), JsError> {
        for pub_key_hash in required_signatures {
            mesh_csl.add_required_signature(pub_key_hash)?;
        }
        Ok(())
    }

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
}

impl Default for MeshTxBuilderCore {
    fn default() -> Self {
        Self::new_core(None)
    }
}
