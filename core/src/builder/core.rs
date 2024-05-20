use crate::{
    core::builder::{IMeshCSL, MeshCSL},
    csl,
    model::*,
};

use super::interface::{IMeshTxBuilderCore, MeshTxBuilderCore};

impl IMeshTxBuilderCore for MeshTxBuilderCore {
    fn new_core() -> Self {
        Self {
            mesh_csl: MeshCSL::new(),
            mesh_tx_builder_body: MeshTxBuilderBody {
                inputs: vec![],
                outputs: vec![],
                collaterals: vec![],
                required_signatures: JsVecString::new(),
                reference_inputs: vec![],
                mints: vec![],
                change_address: "".to_string(),
                change_datum: None,
                metadata: vec![],
                validity_range: ValidityRange {
                    invalid_before: None,
                    invalid_hereafter: None,
                },
                signing_key: JsVecString::new(),
            },
            tx_evaluation_multiplier_percentage: 10,
        }
    }

    fn complete_signing(&mut self) -> String {
        let signing_keys = self.mesh_tx_builder_body.signing_key.clone();
        self.add_all_signing_keys(signing_keys);
        self.mesh_csl.tx_hex.to_string()
    }

    fn serialize_tx_body(&mut self) -> &mut Self {
        self.mesh_tx_builder_body
            .mints
            .sort_by(|a, b| a.policy_id.cmp(&b.policy_id));

        self.mesh_tx_builder_body.inputs.sort_by(|a, b| {
            let tx_in_data_a: &TxInParameter = match a {
                TxIn::PubKeyTxIn(pub_key_tx_in) => &pub_key_tx_in.tx_in,
                TxIn::ScriptTxIn(script_tx_in) => &script_tx_in.tx_in,
            };

            let tx_in_data_b: &TxInParameter = match b {
                TxIn::PubKeyTxIn(pub_key_tx_in) => &pub_key_tx_in.tx_in,
                TxIn::ScriptTxIn(script_tx_in) => &script_tx_in.tx_in,
            };

            tx_in_data_a
                .tx_hash
                .cmp(&tx_in_data_b.tx_hash)
                .then_with(|| tx_in_data_a.tx_index.cmp(&tx_in_data_b.tx_index))
        });

        self.add_all_inputs(self.mesh_tx_builder_body.inputs.clone());
        self.add_all_outputs(self.mesh_tx_builder_body.outputs.clone());
        self.add_all_collaterals(self.mesh_tx_builder_body.collaterals.clone());
        self.add_all_reference_inputs(self.mesh_tx_builder_body.reference_inputs.clone());
        self.add_all_mints(self.mesh_tx_builder_body.mints.clone());
        self.add_validity_range(self.mesh_tx_builder_body.validity_range.clone());
        self.add_all_required_signature(self.mesh_tx_builder_body.required_signatures.clone());
        self.add_all_metadata(self.mesh_tx_builder_body.metadata.clone());

        self.mesh_csl.add_script_hash();
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
        self.mesh_csl.add_change(
            self.mesh_tx_builder_body.change_address.clone(),
            self.mesh_tx_builder_body.change_datum.clone(),
        );
        self.mesh_csl.build_tx();
        self
    }

    fn add_all_signing_keys(&mut self, signing_keys: JsVecString) {
        if !signing_keys.is_empty() {
            self.mesh_csl.add_signing_keys(signing_keys);
        }
    }

    fn add_all_inputs(&mut self, inputs: Vec<TxIn>) {
        for input in inputs {
            match input {
                TxIn::PubKeyTxIn(pub_key_tx_in) => self.mesh_csl.add_tx_in(pub_key_tx_in),
                TxIn::ScriptTxIn(script_tx_in) => self.mesh_csl.add_script_tx_in(script_tx_in),
            };
        }
        self.mesh_csl
            .tx_builder
            .set_inputs(&self.mesh_csl.tx_inputs_builder);
    }

    fn add_all_outputs(&mut self, outputs: Vec<Output>) {
        for output in outputs {
            self.mesh_csl.add_output(output);
        }
    }

    fn add_all_collaterals(&mut self, collaterals: Vec<PubKeyTxIn>) {
        let mut collateral_builder = csl::TxInputsBuilder::new();
        for collateral in collaterals {
            self.mesh_csl
                .add_collateral(&mut collateral_builder, collateral)
        }
        self.mesh_csl.tx_builder.set_collateral(&collateral_builder)
    }

    fn add_all_reference_inputs(&mut self, ref_inputs: Vec<RefTxIn>) {
        for ref_input in ref_inputs {
            self.mesh_csl.add_reference_input(ref_input);
        }
    }

    fn add_all_mints(&mut self, mints: Vec<MintItem>) {
        let mut mint_builder = csl::MintBuilder::new();
        for (index, mint) in mints.into_iter().enumerate() {
            match mint.type_.as_str() {
                "Plutus" => self
                    .mesh_csl
                    .add_plutus_mint(&mut mint_builder, mint, index as u64),
                "Native" => self.mesh_csl.add_native_mint(&mut mint_builder, mint),
                _ => {}
            };
        }
        self.mesh_csl.tx_builder.set_mint_builder(&mint_builder)
    }

    fn add_validity_range(&mut self, validity_range: ValidityRange) {
        if validity_range.invalid_before.is_some() {
            self.mesh_csl
                .add_invalid_before(validity_range.invalid_before.unwrap())
        }
        if validity_range.invalid_hereafter.is_some() {
            self.mesh_csl
                .add_invalid_hereafter(validity_range.invalid_hereafter.unwrap())
        }
    }

    fn add_all_required_signature(&mut self, required_signatures: JsVecString) {
        for pub_key_hash in required_signatures {
            self.mesh_csl.add_required_signature(pub_key_hash);
        }
    }

    fn add_all_metadata(&mut self, all_metadata: Vec<Metadata>) {
        for metadata in all_metadata {
            self.mesh_csl.add_metadata(metadata);
        }
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
        Self::new_core()
    }
}
