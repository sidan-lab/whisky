use async_trait::async_trait;
use sidan_csl_rs::{
    builder::MeshTxBuilderCore,
    model::{
        Asset, LanguageVersion, MeshTxBuilderBody, MintItem, Output, PoolParams, Protocol, PubKeyTxIn, Redeemer, TxIn, UTxO, Withdrawal
    },
};

use crate::service::{IEvaluator, IFetcher, ISubmitter};

pub struct MeshTxBuilder {
    pub core: MeshTxBuilderCore,
    pub protocol_params: Option<Protocol>,
    pub tx_in_item: Option<TxIn>,
    pub extra_inputs: Vec<UTxO>,
    pub selection_threshold: u64,
    pub withdrawal_item: Option<Withdrawal>,
    pub mint_item: Option<MintItem>,
    pub collateral_item: Option<PubKeyTxIn>,
    pub tx_output: Option<Output>,
    pub adding_script_input: bool,
    pub adding_plutus_mint: bool,
    pub adding_plutus_withdrawal: bool,
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
    pub params: Option<Protocol>,
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
    /// Obtain the transaction hex
    ///
    /// ### Returns
    ///
    /// * tx_hex - The current transaction hex from build
    fn tx_hex(&mut self) -> String;

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
    /// Indicate that the transaction is withdrawing using a plutus staking script in the MeshTxBuilder instance
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    fn withdrawal_plutus_script_v2(&mut self) -> &mut Self;

    /// ## Transaction building method
    ///
    /// Add a withdrawal reference to the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `tx_hash` - The transaction hash
    /// * `tx_index` - The transaction index
    /// * `withdrawal_script_hash` - The withdrawal script hash
    /// * `version` - The language version
    /// * `scrip_size` - Size of the script
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    fn withdrawal_tx_in_reference(
        &mut self,
        tx_hash: &str,
        tx_index: u32,
        withdrawal_script_hash: &str,
        version: LanguageVersion,
        script_size: usize,
    ) -> &mut Self;

    /// ## Transaction building method
    ///
    /// Withdraw stake rewards in the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `stake_address` - The address corresponding to the stake key
    /// * `coin` - The amount of lovelaces in the withdrawal
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    fn withdrawal(&mut self, stake_address: &str, coin: u64) -> &mut Self;

    /// ## Transaction building method
    ///
    /// Add a withdrawal script to the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `script_cbor` - The script in CBOR format
    /// * `version` - The language version
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    fn withdrawal_script(&mut self, script_cbor: &str, version: LanguageVersion) -> &mut Self;

    /// ## Transaction building method
    ///
    /// Set the transaction withdrawal redeemer value in the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `redeemer` - The redeemer value
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    fn withdrawal_redeemer_value(&mut self, redeemer: Redeemer) -> &mut Self;

    /// ## Transaction building method
    ///
    /// Set the withdrawal reference redeemer value in the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `redeemer` - The redeemer value
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    fn withdrawal_reference_tx_in_redeemer_value(&mut self, redeemer: Redeemer) -> &mut Self;

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
    /// Add a pool registration certificate to the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `pool_params` - Parameters of pool to be registered
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    fn register_pool_certificate(&mut self, pool_params: PoolParams) -> &mut Self;

    /// ## Transaction building method
    ///
    /// Add a stake registration certificate to the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `stake_key_hash` - Hash of the stake key
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    fn register_stake_certificate(&mut self, stake_key_hash: &str) -> &mut Self;

    /// ## Transaction building method
    ///
    /// Add a stake delegation certificate to the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `stake_key_hash` - Hash of the stake key
    /// * `pool_id` - id of the pool that will be delegated to
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    fn delegate_stake_certificate(&mut self, stake_key_hash: &str, pool_id: &str) -> &mut Self;

    /// ## Transaction building method
    ///
    /// Add a stake deregistration certificate to the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `stake_key_hash` - Hash of the stake key
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    fn deregister_stake_certificate(&mut self, stake_key_hash: &str) -> &mut Self;

    /// ## Transaction building method
    ///
    /// Add a pool retire certificate to the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `pool_id` - id of the pool that will be retired
    /// * `epoch` - The epoch that the pool will be retired from
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    fn retire_pool_certificate(&mut self, pool_id: &str, epoch: u32) -> &mut Self;

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

    /// ## Transaction building method
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
    /// Queue an input in the MeshTxBuilder instance
    fn queue_input(&mut self);

    /// ## Internal method
    ///
    /// Queue a withdrawal in the MeshTxBuilder instance
    fn queue_withdrawal(&mut self);

    /// ## Internal method
    ///
    /// Queue a mint in the MeshTxBuilder instance
    fn queue_mint(&mut self);

    /// ## Internal method
    ///
    /// Queue all last items in the MeshTxBuilder instance
    fn queue_all_last_item(&mut self);

    /// ## Internal method
    ///
    /// Perform the utxo selection process
    ///
    /// ### Arguments
    ///
    /// * `extra_inputs` - A vector of extra inputs provided
    /// * `threshold` - The threshold as configured
    fn add_utxos_from(&mut self, extra_inputs: Vec<UTxO>, threshold: u64);
}
