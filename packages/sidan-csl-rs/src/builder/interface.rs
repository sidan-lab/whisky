use crate::{core::builder::MeshCSL, model::*};

pub struct MeshTxBuilderCore {
    pub mesh_csl: MeshCSL,
    pub mesh_tx_builder_body: MeshTxBuilderBody,
    pub tx_evaluation_multiplier_percentage: u64,
}

pub trait IMeshTxBuilderCore {
    /// ## Transaction building method
    ///
    /// Create a new MeshTxBuilder instance
    ///
    /// ### Returns
    ///
    /// * `Self` - A new MeshTxBuilder instance
    ///
    fn new_core() -> Self;

    /// ## Transaction building method
    ///
    /// Complete the signing process
    ///
    /// ### Returns
    ///
    /// * `String` - The signed transaction in hex

    fn complete_signing(&mut self) -> String;

    /// ## Internal method
    ///
    /// Add multiple signing keys to the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `signing_keys` - A vector of signing keys in hexadecimal
    fn add_all_signing_keys(&mut self, signing_keys: JsVecString);

    /// ## Internal method
    ///
    /// Add multiple inputs to the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `mesh_csl` - The MeshCSL instance
    /// * `inputs` - A vector of inputs
    fn add_all_inputs(mesh_csl: &mut MeshCSL, inputs: Vec<TxIn>);

    /// ## Internal method
    ///
    /// Add multiple outputs to the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `mesh_csl` - The MeshCSL instance
    /// * `outputs` - A vector of outputs
    fn add_all_outputs(mesh_csl: &mut MeshCSL, outputs: Vec<Output>);

    /// ## Internal method
    ///
    /// Add multiple collaterals to the MeshTxBuilder instance
    ///
    /// ## Arguments
    ///
    /// * `mesh_csl` - The MeshCSL instance
    /// * `collaterals` - A vector of collaterals
    fn add_all_collaterals(mesh_csl: &mut MeshCSL, collaterals: Vec<PubKeyTxIn>);

    /// ## Internal method
    ///
    /// Add multiple reference inputs to the MeshTxBuilder instance
    ///
    /// ## Arguments
    ///
    /// * `mesh_csl` - The MeshCSL instance
    /// * `ref_inputs` - A vector of reference inputs
    fn add_all_reference_inputs(mesh_csl: &mut MeshCSL, ref_inputs: Vec<RefTxIn>);

    /// ## Internal method
    ///
    /// Add multiple mints to the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `mesh_csl` - The MeshCSL instance
    /// * `mints` - A vector of mints
    fn add_all_mints(mesh_csl: &mut MeshCSL, mints: Vec<MintItem>);

    /// ## Internal method
    ///
    /// Add a validity range to the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `mesh_csl` - The MeshCSL instance
    /// * `validity_range` - The validity range
    fn add_validity_range(mesh_csl: &mut MeshCSL, validity_range: ValidityRange);

    /// ## Internal method
    ///
    /// Add multiple required signatures to the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `mesh_csl` - The MeshCSL instance
    /// * `required_signatures` - A vector of required signatures
    fn add_all_required_signature(mesh_csl: &mut MeshCSL, required_signatures: JsVecString);

    /// ## Internal method
    ///
    /// Add multiple metadata to the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `mesh_csl` - The MeshCSL instance
    /// * `all_metadata` - A vector of metadata
    fn add_all_metadata(mesh_csl: &mut MeshCSL, all_metadata: Vec<Metadata>);
}
