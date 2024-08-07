use async_trait::async_trait;
use cardano_serialization_lib::JsError;
use sidan_csl_rs::{
    builder::MeshTxBuilderCore,
    model::{
        Anchor, Asset, DRep, LanguageVersion, MeshTxBuilderBody, MintItem, Output, PoolParams,
        Protocol, PubKeyTxIn, TxIn, UTxO, Withdrawal,
    },
};

use crate::service::{Evaluator, Fetcher, Submitter};

use super::{WData, WRedeemer};

pub struct MeshTxBuilder {
    pub core: MeshTxBuilderCore,
    pub protocol_params: Option<Protocol>,
    pub tx_in_item: Option<TxIn>,
    pub withdrawal_item: Option<Withdrawal>,
    pub mint_item: Option<MintItem>,
    pub collateral_item: Option<PubKeyTxIn>,
    pub tx_output: Option<Output>,
    pub adding_script_input: bool,
    pub adding_plutus_mint: bool,
    pub adding_plutus_withdrawal: bool,
    pub fetcher: Option<Box<dyn Fetcher>>,
    pub evaluator: Option<Box<dyn Evaluator>>,
    pub submitter: Option<Box<dyn Submitter>>,
    pub extra_inputs: Vec<UTxO>,
    pub selection_threshold: u64,
    pub chained_txs: Vec<String>,
    pub inputs_for_evaluation: Vec<UTxO>,
}

pub struct MeshTxBuilderParam {
    pub evaluator: Option<Box<dyn Evaluator>>,
    pub fetcher: Option<Box<dyn Fetcher>>,
    pub submitter: Option<Box<dyn Submitter>>,
    pub params: Option<Protocol>,
}

#[async_trait]
pub trait IMeshTxBuilder {
    /// ## Transaction building method
    ///
    /// Create a new MeshTxBuilder instance with option params
    ///
    /// ### Arguments
    ///
    /// * `param` - Parameters for setting up the MeshTxBuilder instance, including evaluator, fetcher, submitter, and protocol parameters
    ///
    /// ### Returns
    ///
    /// * `Self` - A new MeshTxBuilder instance
    fn new(param: MeshTxBuilderParam) -> Self;

    /// ## Transaction building method
    ///
    /// Create a new MeshTxBuilder instance without option params
    ///
    /// ### Returns
    ///
    /// * `Self` - A new MeshTxBuilder instance
    fn new_core() -> Self;

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
    async fn complete(
        &mut self,
        customized_tx: Option<MeshTxBuilderBody>,
    ) -> Result<&mut Self, JsError>;

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
    fn complete_sync(
        &mut self,
        customized_tx: Option<MeshTxBuilderBody>,
    ) -> Result<&mut Self, JsError>;

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
    /// * `version` - The language version, leave as None for Native scripts
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    fn tx_in_script(&mut self, script_cbor: &str, version: Option<LanguageVersion>) -> &mut Self;

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
    fn tx_in_datum_value(&mut self, data: WData) -> &mut Self;

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
    fn tx_in_redeemer_value(&mut self, redeemer: WRedeemer) -> &mut Self;

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
    fn tx_out_datum_hash_value(&mut self, data: WData) -> &mut Self;

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
    fn tx_out_inline_datum_value(&mut self, data: WData) -> &mut Self;

    /// ## Transaction building method
    ///
    /// Add a transaction output reference script to the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `script_cbor` - The script in CBOR format
    /// * `version` - The language version, if the language version is None, the script is assumed to be a Native Script
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    fn tx_out_reference_script(
        &mut self,
        script_cbor: &str,
        version: Option<LanguageVersion>,
    ) -> &mut Self;

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
    fn spending_reference_tx_in_redeemer_value(&mut self, redeemer: WRedeemer) -> &mut Self;

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
    /// * `version` - The language version, if the language version is None, the script is assumed to be a Native Script
    /// * `script_size` - Size of the script
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    fn withdrawal_tx_in_reference(
        &mut self,
        tx_hash: &str,
        tx_index: u32,
        withdrawal_script_hash: &str,
        version: Option<LanguageVersion>,
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
    /// * `version` - The language version, if the language version is None, the script is assumed to be a Native Script
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    fn withdrawal_script(
        &mut self,
        script_cbor: &str,
        version: Option<LanguageVersion>,
    ) -> &mut Self;

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
    fn withdrawal_redeemer_value(&mut self, redeemer: WRedeemer) -> &mut Self;

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
    fn withdrawal_reference_tx_in_redeemer_value(&mut self, redeemer: WRedeemer) -> &mut Self;

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
    fn mint(&mut self, quantity: i128, policy: &str, name: &str) -> &mut Self;

    /// ## Transaction building method
    ///
    /// Add a minting script to the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `script_cbor` - The script in CBOR format
    /// * `version` - The language version, if the language version is None, the script is assumed to be a Native Script
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    fn minting_script(&mut self, script_cbor: &str, version: Option<LanguageVersion>) -> &mut Self;

    /// ## Transaction building method
    ///
    /// Add a minting transaction input reference to the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `tx_hash` - The transaction hash
    /// * `tx_index` - The transaction index
    /// * `spending_script_hash` - The spending script hash
    /// * `version` - The language version, if the language version is None, the script is assumed to be a Native Script
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
        version: Option<LanguageVersion>,
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
    fn mint_redeemer_value(&mut self, redeemer: WRedeemer) -> &mut Self;

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
    fn mint_reference_tx_in_redeemer_value(&mut self, redeemer: WRedeemer) -> &mut Self;

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
    /// * `stake_key_address` - Address of the stake key
    /// * `coin` - Deposit for certificate registration
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    fn register_stake_certificate(&mut self, stake_key_address: &str, coin: u64) -> &mut Self;

    /// ## Transaction building method
    ///
    /// Add a stake delegation certificate to the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `stake_key_address` - Address of the stake key
    /// * `pool_id` - id of the pool that will be delegated to
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    fn delegate_stake_certificate(&mut self, stake_key_address: &str, pool_id: &str) -> &mut Self;

    /// ## Transaction building method
    ///
    /// Add a stake deregistration certificate to the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `stake_key_address` - Address of the stake key
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    fn deregister_stake_certificate(&mut self, stake_key_address: &str) -> &mut Self;

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
    /// Add a vote delegation certificate to the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `stake_key_address` - Address of the stake key
    /// * `drep` - The drep that will be voted for, or always abstain / always no confidence
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    fn vote_delegation_certificate(&mut self, stake_key_address: &str, drep: DRep) -> &mut Self;

    /// ## Transaction building method
    ///
    /// Add a stake and vote delegation certificate to the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `stake_key_address` - Address of the stake key
    /// * `pool_key_hash` - Hash of pool key that will be delegated to, same as pool id
    /// * `drep` - The drep that will be voted for, or always abstain / always no confidence
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    fn stake_and_vote_delegation_certificate(
        &mut self,
        stake_key_address: &str,
        pool_key_hash: &str,
        drep: DRep,
    ) -> &mut Self;

    /// ## Transaction building method
    ///
    /// Add a stake registration and delegation certificate to the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `stake_key_address` - Address of the stake key
    /// * `pool_key_hash` - Hash of pool key that will be delegated to, same as pool id
    /// * `coin` - Deposit for certificate registration
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    fn stake_registration_and_delegation(
        &mut self,
        stake_key_address: &str,
        pool_key_hash: &str,
        coin: u64,
    ) -> &mut Self;

    /// ## Transaction building method
    ///
    /// Add a vote registration and delegation certificate to the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `stake_key_address` - Address of the stake key
    /// * `drep` - The drep that will be voted for, or always abstain / always no confidence
    /// * `coin` - Deposit for certificate registration
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    fn vote_registration_and_delegation(
        &mut self,
        stake_key_address: &str,
        drep: DRep,
        coin: u64,
    ) -> &mut Self;

    /// ## Transaction building method
    ///
    /// Add a stake vote registration and delegation certificate to the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `stake_key_address` - Address of the stake key
    /// * `pool_key_hash` - Hash of pool key that will be delegated to, same as pool id
    /// * `drep` - The drep that will be voted for, or always abstain / always no confidence
    /// * `coin` - Deposit for certificate registration
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    fn stake_vote_registration_and_delegation(
        &mut self,
        stake_key_address: &str,
        pool_key_hash: &str,
        drep: DRep,
        coin: u64,
    ) -> &mut Self;

    /// ## Transaction building method
    ///
    /// Add commitee hot auth certificate to the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `committee_cold_key_address` - Address of the committee cold key
    /// * `committee_hot_key_address` - Address of the commitee hot key
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    fn committee_hot_auth(
        &mut self,
        committee_cold_key_address: &str,
        committee_hot_key_address: &str,
    ) -> &mut Self;

    /// ## Transaction building method
    ///
    /// Add commitee cold resign certificate to the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `committee_cold_key_address` - Address of the committee cold key
    /// * `anchor` - The Anchor, this is a URL and a hash of the doc at this URL
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    fn commitee_cold_resign(
        &mut self,
        committee_cold_key_address: &str,
        anchor: Option<Anchor>,
    ) -> &mut Self;

    /// ## Transaction building method
    ///
    /// Add DRep registration certificate to the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `voting_key_address` - Address of the voting key
    /// * `coin` - Deposit for certificate registration
    /// * `anchor` - The Anchor, this is a URL and a hash of the doc at this URL
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    fn drep_registration(
        &mut self,
        voting_key_address: &str,
        coin: u64,
        anchor: Option<Anchor>,
    ) -> &mut Self;

    /// ## Transaction building method
    ///
    /// Add DRep deregistration certificate to the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `voting_key_address` - Address of the voting key
    /// * `coin` - Deposit for certificate registration
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    fn drep_deregistration(&mut self, voting_key_address: &str, coin: u64) -> &mut Self;

    /// ## Transaction building method
    ///
    /// Add DRep update certificate to the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `voting_key_address` - Address of the voting key
    /// * `anchor` - The Anchor, this is a URL and a hash of the doc at this URL
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    fn drep_update(&mut self, voting_key_address: &str, anchor: Option<Anchor>) -> &mut Self;

    /// ## Transaction building method
    ///
    /// Add script witness to certificate
    ///
    /// ### Arguments
    ///
    /// * `script_cbor` - The script in CBOR format
    /// * `version` - The language version, if the language version is None, the script is assumed to be a Native Script
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    fn certificate_script(
        &mut self,
        script_cbor: &str,
        version: Option<LanguageVersion>,
    ) -> &mut Self;

    /// ## Transaction building method
    ///
    /// Add a Certificate transaction input reference to the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `tx_hash` - The transaction hash
    /// * `tx_index` - The transaction index
    /// * `spending_script_hash` - The spending script hash
    /// * `version` - The language version, if the language version is None, the script is assumed to be a Native Script
    /// * `script_size` - Size of the script
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    fn certificate_tx_in_reference(
        &mut self,
        tx_hash: &str,
        tx_index: u32,
        spending_script_hash: &str,
        version: Option<LanguageVersion>,
        script_size: usize,
    ) -> &mut Self;

    /// ## Transaction building method
    ///
    /// Add a Certificate Redeemer to the MeshTxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `redeemer` - The redeemer value
    ///
    /// ### Returns
    ///
    /// * `Self` - The MeshTxBuilder instance
    fn certificate_redeemer_value(&mut self, redeemer: WRedeemer) -> &mut Self;

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
    fn change_output_datum(&mut self, data: WData) -> &mut Self;

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
    fn add_utxos_from(&mut self, extra_inputs: Vec<UTxO>, threshold: u64) -> Result<(), JsError>;
}
