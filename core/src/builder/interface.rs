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
    fn new_core() -> Self;

    /// ## Transaction building method
    ///
    /// Complete the signing process
    ///
    /// ### Returns
    ///
    /// * `String` - The signed transaction in hex
    fn complete_signing(&mut self) -> String;

    /// ## Transaction building method
    ///
    /// Serialize the transaction body
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    fn serialize_tx_body(&mut self) -> &mut Self;

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
    /// * `inputs` - A vector of inputs
    fn add_all_inputs(&mut self, inputs: Vec<TxIn>);

    /// ## Internal method
    ///
    /// Add multiple outputs to the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `outputs` - A vector of outputs
    fn add_all_outputs(&mut self, outputs: Vec<Output>);

    /// ## Internal method
    ///
    /// Add multiple collaterals to the MeshTxBuilder instance
    ///
    /// ## Arguments
    ///
    /// * `collaterals` - A vector of collaterals
    fn add_all_collaterals(&mut self, collaterals: Vec<PubKeyTxIn>);

    /// ## Internal method
    ///
    /// Add multiple reference inputs to the MeshTxBuilder instance
    ///
    /// ## Arguments
    ///
    /// * `ref_inputs` - A vector of reference inputs
    fn add_all_reference_inputs(&mut self, ref_inputs: Vec<RefTxIn>);

    /// ## Internal method
    ///
    /// Add multiple mints to the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `mints` - A vector of mints
    fn add_all_mints(&mut self, mints: Vec<MintItem>);

    /// ## Internal method
    ///
    /// Add a validity range to the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `validity_range` - The validity range
    fn add_validity_range(&mut self, validity_range: ValidityRange);

    /// ## Internal method
    ///
    /// Add multiple required signatures to the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `required_signatures` - A vector of required signatures
    fn add_all_required_signature(&mut self, required_signatures: JsVecString);

    /// ## Internal method
    ///
    /// Add multiple metadata to the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `all_metadata` - A vector of metadata
    fn add_all_metadata(&mut self, all_metadata: Vec<Metadata>);
}
