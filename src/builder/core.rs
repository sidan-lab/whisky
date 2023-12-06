use crate::builder::models::*;

pub struct MeshTxBuilderCore {
    mesh_tx_builder_body: MeshTxBuilderBody
}

impl MeshTxBuilderCore {
    pub fn complete_sync(
        self,
        customized_tx: Option<MeshTxBuilderBody>
    ) -> MeshTxBuilderCore {
        self
    }
}