use cardano_serialization_lib as csl;

use crate::model::builder::PubKeyTxIn;

use super::utils::{build_tx_builder, to_value};

pub struct MeshCSL {
    pub tx_builder: csl::tx_builder::TransactionBuilder,
    pub tx_inputs_builder: csl::tx_builder::tx_inputs_builder::TxInputsBuilder,
}

impl MeshCSL {
    pub fn new() -> MeshCSL {
        MeshCSL {
            tx_builder: build_tx_builder(),
            tx_inputs_builder: csl::tx_builder::tx_inputs_builder::TxInputsBuilder::new(),
        }
    }

    pub fn add_tx_in(&mut self, input: PubKeyTxIn) {
        self.tx_inputs_builder.add_input(
            &csl::address::Address::from_bech32(&input.tx_in.address.unwrap()).unwrap(),
            &csl::TransactionInput::new(
                &csl::crypto::TransactionHash::from_hex(&input.tx_in.tx_hash).unwrap(),
                input.tx_in.tx_index,
            ),
            &to_value(&input.tx_in.amount.unwrap()),
        );
    }
}

impl Default for MeshCSL {
    fn default() -> Self {
        Self::new()
    }
}
