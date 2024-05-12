use async_trait::async_trait;

use crate::{
    core::builder::MeshCSL,
    model::*,
    service::{IEvaluator, IFetcher, ISubmitter},
};

pub struct MeshTxBuilder {
    pub mesh_csl: MeshCSL,
    pub mesh_tx_builder_body: MeshTxBuilderBody,
    pub tx_in_item: Option<TxIn>,
    pub extra_inputs: Vec<UTxO>,
    pub selection_threshold: u64,
    pub mint_item: Option<MintItem>,
    pub collateral_item: Option<PubKeyTxIn>,
    pub tx_output: Option<Output>,
    pub adding_script_input: bool,
    pub adding_plutus_mint: bool,
    pub tx_evaluation_multiplier_percentage: u64,
    pub fetcher: Option<Box<dyn IFetcher>>,
    pub evaluator: Option<Box<dyn IEvaluator>>,
    pub submitter: Option<Box<dyn ISubmitter>>,
    pub chained_txs: Vec<String>,
    pub inputs_for_evaluation: Vec<UTxO>,
}

pub struct MeshTxBuilderParam {
    pub evaluator: Option<Box<dyn IEvaluator>>,
    pub fetcher: Option<Box<dyn IFetcher>>,
    pub submitter: Option<Box<dyn ISubmitter>>,
}

#[async_trait]
pub trait IMeshTxBuilder {
    /// ## Transaction building method
    ///
    /// Create a new MeshTxBuilder instance
    ///
    /// ### Returns
    ///
    /// * `Self` - A new MeshTxBuilder instance
    fn new(param: MeshTxBuilderParam) -> Self;
    /// ## Transaction building method
    ///  
    /// Complete the transaction building process with fetching missing information & tx evaluation
    ///
    /// ### Arguments
    ///
    /// * `customized_tx` - An optional customized transaction body
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    async fn complete(&mut self, customized_tx: Option<MeshTxBuilderBody>) -> &mut Self;
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
    /// Obtain the transaction hex
    ///
    /// ### Returns
    ///
    /// * tx_hex - The current transaction hex from build
    fn tx_hex(&mut self) -> String;

    /// ## Transaction building method
    ///
    /// Complete the transaction building process synchronously
    ///
    /// ### Arguments
    ///
    /// * `customized_tx` - An optional customized transaction body
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    fn complete_sync(&mut self, customized_tx: Option<MeshTxBuilderBody>) -> &mut Self;

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

    /// ## Transaction building method
    ///
    /// Add a transaction input to the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `tx_hash` - The transaction hash
    /// * `tx_index` - The transaction index
    /// * `amount` - The amount of assets
    /// * `address` - The address
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    fn tx_in(
        &mut self,
        tx_hash: &str,
        tx_index: u32,
        amount: Vec<Asset>,
        address: &str,
    ) -> &mut Self;

    /// ## Transaction building method
    ///
    /// Add a transaction input script to the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `script_cbor` - The script in CBOR format
    /// * `version` - The language version
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    fn tx_in_script(&mut self, script_cbor: &str, version: LanguageVersion) -> &mut Self;

    /// ## Transaction building method
    ///
    /// Set the transaction input datum value in the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `data` - The datum value
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    fn tx_in_datum_value(&mut self, data: &str) -> &mut Self;

    /// ## Transaction building method
    ///
    /// Indicate that the transaction input has an inline datum in the MeshTxBuilder instance
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    fn tx_in_inline_datum_present(&mut self) -> &mut Self;

    /// ## Transaction building method
    ///
    /// Set the transaction input redeemer value in the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `redeemer` - The redeemer value
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    fn tx_in_redeemer_value(&mut self, redeemer: Redeemer) -> &mut Self;

    /// ## Transaction building method
    ///
    /// Add a transaction output to the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `address` - The address
    /// * `amount` - The amount of assets
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    fn tx_out(&mut self, address: &str, amount: Vec<Asset>) -> &mut Self;

    /// ## Transaction building method
    ///
    /// Set the transaction output datum hash value in the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `data` - The datum hash value
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    fn tx_out_datum_hash_value(&mut self, data: &str) -> &mut Self;

    /// ## Transaction building method
    ///
    /// Set the transaction output inline datum value in the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `data` - The inline datum value
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    fn tx_out_inline_datum_value(&mut self, data: &str) -> &mut Self;

    /// ## Transaction building method
    ///
    /// Add a transaction output reference script to the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `script_cbor` - The script in CBOR format
    /// * `version` - The language version
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    fn tx_out_reference_script(&mut self, script_cbor: &str, version: LanguageVersion)
        -> &mut Self;

    /// ## Transaction building method
    ///
    /// Indicate that the transaction is spending a Plutus script v2 in the MeshTxBuilder instance
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    fn spending_plutus_script_v2(&mut self) -> &mut Self;

    /// ## Transaction building method
    ///
    /// Add a spending transaction input reference to the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `tx_hash` - The transaction hash
    /// * `tx_index` - The transaction index
    /// * `spending_script_hash` - The spending script hash
    /// * `version` - The language version
    /// * `scrip_size` - Size of the script
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    fn spending_tx_in_reference(
        &mut self,
        tx_hash: &str,
        tx_index: u32,
        spending_script_hash: &str,
        version: LanguageVersion,
        script_size: usize,
    ) -> &mut Self;

    /// ## Transaction building method
    ///
    /// Indicate that the spending reference transaction input has an inline datum in the MeshTxBuilder instance
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    fn spending_reference_tx_in_inline_datum_present(&mut self) -> &mut Self;

    /// ## Transaction building method
    ///
    /// Set the spending reference transaction input redeemer value in the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `redeemer` - The redeemer value
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    fn spending_reference_tx_in_redeemer_value(&mut self, redeemer: Redeemer) -> &mut Self;

    /// ## Transaction building method
    ///
    /// Add a read-only transaction input reference to the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `tx_hash` - The transaction hash
    /// * `tx_index` - The transaction index
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    fn read_only_tx_in_reference(&mut self, tx_hash: &str, tx_index: u32) -> &mut Self;

    /// ## Transaction building method
    ///
    /// Indicate that the transaction is minting a Plutus script v2 in the MeshTxBuilder instance
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    fn mint_plutus_script_v2(&mut self) -> &mut Self;

    /// ## Transaction building method
    ///
    /// Mint assets in the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `quantity` - The quantity of assets to mint
    /// * `policy` - The policy
    /// * `name` - The name of the asset
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    fn mint(&mut self, quantity: u64, policy: &str, name: &str) -> &mut Self;

    /// ## Transaction building method
    ///
    /// Add a minting script to the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `script_cbor` - The script in CBOR format
    /// * `version` - The language version
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    fn minting_script(&mut self, script_cbor: &str, version: LanguageVersion) -> &mut Self;

    /// ## Transaction building method
    ///
    /// Add a minting transaction input reference to the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `tx_hash` - The transaction hash
    /// * `tx_index` - The transaction index
    /// * `spending_script_hash` - The spending script hash
    /// * `version` - The language version
    /// * `script_size` - Size of the script
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    fn mint_tx_in_reference(
        &mut self,
        tx_hash: &str,
        tx_index: u32,
        spending_script_hash: &str,
        version: LanguageVersion,
        script_size: usize,
    ) -> &mut Self;

    /// ## Transaction building method
    ///
    /// Set the minting redeemer value in the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `redeemer` - The redeemer value
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    fn mint_redeemer_value(&mut self, redeemer: Redeemer) -> &mut Self;

    /// ## Transaction building method
    ///
    /// Set the minting reference transaction input redeemer value in the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `redeemer` - The redeemer value
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    fn mint_reference_tx_in_redeemer_value(&mut self, redeemer: Redeemer) -> &mut Self;

    /// ## Transaction building method
    ///
    /// Add a required signer hash to the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `pub_key_hash` - The public key hash
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    fn required_signer_hash(&mut self, pub_key_hash: &str) -> &mut Self;

    /// ## Transaction building method
    ///
    /// Add a transaction input collateral to the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `tx_hash` - The transaction hash
    /// * `tx_index` - The transaction index
    /// * `amount` - The amount of assets
    /// * `address` - The address
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    fn tx_in_collateral(
        &mut self,
        tx_hash: &str,
        tx_index: u32,
        amount: Vec<Asset>,
        address: &str,
    ) -> &mut Self;

    /// ## Transaction building method
    ///
    /// Change the address in the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `address` - The new address
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    fn change_address(&mut self, address: &str) -> &mut Self;

    /// ## Transaction building method
    ///
    /// Change the output datum in the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `data` - The new output datum
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    fn change_output_datum(&mut self, data: &str) -> &mut Self;

    /// ## Transaction building method
    ///
    /// Set the invalid_before slot in the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `slot` - The new invalid_before slot
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    fn invalid_before(&mut self, slot: u64) -> &mut Self;

    /// ## Transaction building method
    ///
    /// Set the invalid_hereafter slot in the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `slot` - The new invalid_hereafter slot
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    fn invalid_hereafter(&mut self, slot: u64) -> &mut Self;

    /// ## Transaction building method
    ///
    /// Add a metadata value to the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `tag` - The tag for the metadata
    /// * `metadata` - The metadata value
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    fn metadata_value(&mut self, tag: &str, metadata: &str) -> &mut Self;

    /// ## Transaction building method
    ///
    /// Add a signing key to the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `skey_hex` - The signing key in hexadecimal
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    fn signing_key(&mut self, skey_hex: &str) -> &mut Self;

    /// ## Transaction building method
    ///
    /// Add a transaction that used as input, but not yet reflected on global blockchain
    ///
    /// ### Arguments
    ///
    /// * `tx_hex` - The transaction hex of chained transaction
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    fn chain_tx(&mut self, tx_hex: &str) -> &mut Self;

    /// ## Transaction building method
    ///
    /// Add a transaction input to provide information for offline evaluation
    ///
    /// ### Arguments
    ///
    /// * `input` - The input to be added
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    fn input_for_evaluation(&mut self, input: UTxO) -> &mut Self;

    /// ## Internal method
    ///
    /// Selects utxos to fill output value and puts them into inputs
    ///
    /// ### Arguments
    ///
    /// * `inputs` - The inputs already placed into the object will remain, these extra inputs will be used to fill the remaining  value needed
    /// * `threshold` - Extra value needed to be selected for, usually for paying fees and min UTxO value of change output
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    fn select_utxos_from(&mut self, extra_inputs: Vec<UTxO>, threshold: u64) -> &mut Self;

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
    /// Perform the utxo selection process
    ///
    /// ### Arguments
    ///
    /// * `extra_inputs` - A vector of extra inputs provided
    /// * `threshold` - The threshold as configured
    fn add_utxos_from(&mut self, extra_inputs: Vec<UTxO>, threshold: u64);

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

    /// ## Internal method
    ///
    /// Queue an input in the MeshTxBuilder instance
    fn queue_input(&mut self);

    /// ## Internal method
    ///
    /// Queue a mint in the MeshTxBuilder instance
    fn queue_mint(&mut self);

    /// ## Internal method
    ///
    /// Queue all last items in the MeshTxBuilder instance
    fn queue_all_last_item(&mut self);
}
