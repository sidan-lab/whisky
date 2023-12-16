use cardano_serialization_lib as csl;

use crate::{
    builder::models::*,
    utils::csl::{build_tx_builder, to_value},
};

pub struct MeshTxBuilderCore {
    tx_builder: csl::tx_builder::TransactionBuilder,
    mesh_tx_builder_body: MeshTxBuilderBody,
}

impl MeshTxBuilderCore {
    pub fn new() -> Self {
        Self {
            tx_builder: build_tx_builder(),
            mesh_tx_builder_body: MeshTxBuilderBody {
                inputs: vec![],
                outputs: vec![],
                collaterals: vec![],
                required_signatures: vec![],
                reference_inputs: vec![],
                mints: vec![],
                change_address: String::from(""),
                metadata: vec![],
                validity_range: ValidityRange {
                    invalid_before: None,
                    invalid_hereafter: None,
                },
                signing_key: vec![],
            },
        }
    }

    pub fn complete_sync(
        &mut self,
        customized_tx: Option<MeshTxBuilderBody>,
    ) -> &mut MeshTxBuilderCore {
        if customized_tx.is_some() {
            self.mesh_tx_builder_body = customized_tx.unwrap();
        }
        return self.serialize_tx_body();
    }

    pub fn serialize_tx_body(&mut self) -> &mut MeshTxBuilderCore {
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
        self
    }

    fn add_all_inputs(&mut self, inputs: Vec<TxIn>) {
        for input in inputs {
            match input {
                TxIn::PubKeyTxIn(pub_key_tx_in) => self.add_tx_in(pub_key_tx_in),
                TxIn::ScriptTxIn(script_tx_in) => self.add_script_tx_in(script_tx_in),
            };
        }
    }

    fn add_tx_in(&mut self, input: PubKeyTxIn) {
        self.tx_builder.add_input(
            &csl::address::Address::from_bech32(&input.tx_in.address.unwrap()).unwrap(),
            &csl::TransactionInput::new(
                &csl::crypto::TransactionHash::from_hex(&input.tx_in.tx_hash).unwrap(),
                input.tx_in.tx_index,
            ),
            &to_value(&input.tx_in.amount.unwrap()),
        )
    }

    fn add_script_tx_in(&mut self, input: ScriptTxIn) {}
}
