use builder::TxBuildResult;
use cardano_serialization_lib::JsError;

use crate::{
    core::builder::{IMeshCSL, MeshCSL},
    csl,
    model::*,
    *,
};

use super::{interface::MeshTxBuilderCore, IMeshTxBuilderCore};

/// ## WASM Transaction building method
///
/// Serialize the transaction body
///
/// ### Arguments
///
/// * `mesh_tx_builder_body_json` - The transaction builder body information, serialized as JSON string
/// * `params_json` - Optional protocol parameters, default as Cardano mainnet configuration, serialized as JSON string
///
/// ### Returns
///
/// * `String` - the built transaction hex
#[wasm_bindgen]
pub fn js_serialize_tx_body(mesh_tx_builder_body_json: &str, params_json: &str) -> TxBuildResult {
    let mesh_tx_builder_body: MeshTxBuilderBody =
        match serde_json::from_str(mesh_tx_builder_body_json) {
            Ok(mesh_tx_builder_body) => mesh_tx_builder_body,
            Err(e) => {
                return TxBuildResult::new("failure".to_string(), format!("Invalid JSON: {:?}", e))
            }
        };

    let params: Option<Protocol> = match serde_json::from_str(params_json) {
        Ok(params) => Some(params),
        Err(e) => {
            return TxBuildResult::new(
                "failure".to_string(),
                format!("Invalid Protocol Param JSON: {:?} \n {:?}", params_json, e),
            )
        }
    };

    match serialize_tx_body(mesh_tx_builder_body, params) {
        Ok(tx_hex) => TxBuildResult::new("success".to_string(), tx_hex.to_string()),
        Err(e) => TxBuildResult::new("failure".to_string(), format!("{:?}", e)),
    }
}

#[test]
fn test_js_serialize_tx_body() {
    let mesh_tx_builder_body_json = r#"{"inputs":[{"pubKeyTxIn":{"txIn":{"txHash":"989c9d1b383a79dbfc7ff197f0cbf2e38cb35ae2c982ba0b5b0d3bee92be7b19","txIndex":0,"amount":[{"unit":"lovelace","quantity":"1000000000"}],"address":"addr_test1qrjvlw7rzlr33audzdak2dwcjsrp3npa9tqy78e44nmcu5ap2gwze63uc7tk94t4tq0y06nqhr42qdpsw9k06c2qc7tqzjkxjr"}}}],"outputs":[{"address":"addr_test1qrjvlw7rzlr33audzdak2dwcjsrp3npa9tqy78e44nmcu5ap2gwze63uc7tk94t4tq0y06nqhr42qdpsw9k06c2qc7tqzjkxjr","amount":[{"unit":"67dd133868f14107b25772f3c5abaa1e0549f4b400b5e0e3a1136152000643b05465737431","quantity":"1"}],"datum":null,"referenceScript":null},{"address":"addr_test1wpna6yecdrc5zpaj2ae083dt4g0q2j05ksqttc8r5yfkz5sxwvzyq","amount":[{"unit":"67dd133868f14107b25772f3c5abaa1e0549f4b400b5e0e3a1136152000de1405465737431","quantity":"1"}],"datum":{"inline":"d8799fa4446e616d6545546573743145696d6167655835697066733a2f2f516d527a6963705265757477436b4d36616f74754b6a4572464355443231334470775071364279757a4d4a617561496d656469615479706549696d6167652f6a70674b6465736372697074696f6e5348656c6c6f20776f726c64202d20434950363802ff"},"referenceScript":null}],"collaterals":[],"requiredSignatures":["e4cfbbc317c718f78d137b6535d8940618cc3d2ac04f1f35acf78e53"],"referenceInputs":[],"mints":[{"scriptMint":{"mint":{"policyId":"67dd133868f14107b25772f3c5abaa1e0549f4b400b5e0e3a1136152","assetName":"000643b05465737431","amount":1},"redeemer":{"data":"d8799f446d657368ff","exUnits":{"mem":7000000,"steps":3000000000}},"scriptSource":{"providedScriptSource":{"scriptCbor":"5883588101000032323232323232322232533300632323232533300a3370e9000000899b8f375c601c601000e911046d6573680014a0601000260180026018002600800429309b2b19299980319b87480000044c8c94ccc02cc03400852616375c601600260080062c60080044600a6ea80048c00cdd5000ab9a5573aaae7955cfaba15745","languageVersion":"v2"}}}},{"scriptMint":{"mint":{"policyId":"67dd133868f14107b25772f3c5abaa1e0549f4b400b5e0e3a1136152","assetName":"000de1405465737431","amount":1},"redeemer":{"data":"d8799f446d657368ff","exUnits":{"mem":7000000,"steps":3000000000}},"scriptSource":{"providedScriptSource":{"scriptCbor":"5883588101000032323232323232322232533300632323232533300a3370e9000000899b8f375c601c601000e911046d6573680014a0601000260180026018002600800429309b2b19299980319b87480000044c8c94ccc02cc03400852616375c601600260080062c60080044600a6ea80048c00cdd5000ab9a5573aaae7955cfaba15745","languageVersion":"v2"}}}}],"changeAddress":"addr_test1qq7lutcvhg5dl0z4rurdpdy9c0khp7ymel83wf8jt6d9qgap2gwze63uc7tk94t4tq0y06nqhr42qdpsw9k06c2qc7tql6q7cf","metadata":[],"validityRange":{"invalidBefore":null,"invalidHereafter":null},"certificates":[],"signingKey":[],"withdrawals":[]}"#;
    // let mesh_tx_builder_body_json = r#"{"inputs":[{"pubKeyTxIn":{"txIn":{"txHash":"5b0145fe7b0212a7807e7dba24997049374d965f587300a2039b73cd30806c78","txIndex":1,"amount":[{"unit":"lovelace","quantity":"1132923230"}],"address":"addr_test1qq0yavv5uve45rwvfaw96qynrqt8ckpmkwcg08vlwxxdncxk82f5wz75mzaesmqzl79xqsmedwgucwtuav5str6untqqmykcpn"}}}],"outputs":[{"address":"addr_test1wpnlxv2xv9a9ucvnvzqakwepzl9ltx7jzgm53av2e9ncv4sysemm8","amount":[{"unit":"lovelace","quantity":"1600000"}],"datum":{"hash":{"type":"Mesh","content":"supersecret"}},"referenceScript":null}],"collaterals":[{"txIn":{"txHash":"ec0c2e70b898cf531b03c9db937602e98c45378d9fa8e8a5b5a91ec5c1d7540d","txIndex":5,"amount":[{"unit":"lovelace","quantity":"5000000"}],"address":"addr_test1qq0yavv5uve45rwvfaw96qynrqt8ckpmkwcg08vlwxxdncxk82f5wz75mzaesmqzl79xqsmedwgucwtuav5str6untqqmykcpn"}}],"requiredSignatures":[],"referenceInputs":[],"mints":[],"changeAddress":"addr_test1qq0yavv5uve45rwvfaw96qynrqt8ckpmkwcg08vlwxxdncxk82f5wz75mzaesmqzl79xqsmedwgucwtuav5str6untqqmykcpn","metadata":[],"validityRange":{"invalidBefore":null,"invalidHereafter":null},"certificates":[],"signingKey":[],"withdrawals":[]}"#;
    let params_json = r#"{"epoch":0,"coinsPerUtxoSize":4310,"priceMem":0.0577,"priceStep":0.0000721,"minFeeA":44,"minFeeB":155381,"keyDeposit":2000000,"maxTxSize":16384,"maxValSize":5000,"poolDeposit":500000000,"maxCollateralInputs":3,"decentralisation":0,"maxBlockSize":98304,"collateralPercent":150,"maxBlockHeaderSize":1100,"minPoolCost":"340000000","maxTxExMem":"16000000","maxTxExSteps":"10000000000","maxBlockExMem":"80000000","maxBlockExSteps":"40000000000"}"#;
    let tx_build_result = js_serialize_tx_body(mesh_tx_builder_body_json, params_json);
    println!("{:?}", tx_build_result);
}

/// ## Transaction building method
///
/// Serialize the transaction body
///
/// ### Arguments
///
/// * `mesh_tx_builder_body` - The transaction builder body information
/// * `params` - Optional protocol parameters, default as Cardano mainnet configuration
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
                TxIn::SimpleScriptTxIn(simple_script_tx_in) => {
                    mesh_csl.add_simple_script_tx_in(simple_script_tx_in)?
                }
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
            match mint {
                MintItem::ScriptMint(script_mint) => {
                    mesh_csl.add_plutus_mint(&mut mint_builder, script_mint, index as u64)?
                }
                MintItem::SimpleScriptMint(simple_script_mint) => {
                    mesh_csl.add_native_mint(&mut mint_builder, simple_script_mint)?
                }
            };
        }
        println!("6-3");
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
