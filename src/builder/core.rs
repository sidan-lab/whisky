use cardano_serialization_lib::tx_builder::TransactionBuilder;

use crate::{builder::models::*, utils::csl::build_tx_builder};

pub struct MeshTxBuilderCore {
    tx_builder: TransactionBuilder,
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

    pub fn complete_sync(mut self, customized_tx: Option<MeshTxBuilderBody>) -> MeshTxBuilderCore {
        if customized_tx.is_some() {
            self.mesh_tx_builder_body = customized_tx.unwrap();
        }
        self
    }

    pub fn serialize_tx_body(self, tx_body: MeshTxBuilderBody) -> MeshTxBuilderCore {
        self
    }
}
