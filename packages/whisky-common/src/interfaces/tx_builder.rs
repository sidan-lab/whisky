use crate::errors::*;
use crate::models::*;

pub trait TxBuildable {
    fn add_tx_in(&mut self, input: PubKeyTxIn) -> Result<(), WError>;
    fn add_simple_script_tx_in(&mut self, input: SimpleScriptTxIn) -> Result<(), WError>;
    fn add_script_tx_in(&mut self, input: ScriptTxIn) -> Result<(), WError>;
    fn add_output(&mut self, output: Output) -> Result<(), WError>;
    fn add_collateral(&mut self, collateral: PubKeyTxIn) -> Result<(), WError>;
    fn add_reference_input(&mut self, ref_input: RefTxIn) -> Result<(), WError>;
    fn add_pub_key_withdrawal(&mut self, withdrawal: PubKeyWithdrawal) -> Result<(), WError>;
    fn add_plutus_withdrawal(&mut self, withdrawal: PlutusScriptWithdrawal) -> Result<(), WError>;
    fn add_simple_script_withdrawal(
        &mut self,
        withdrawal: SimpleScriptWithdrawal,
    ) -> Result<(), WError>;
    fn add_plutus_mint(&mut self, script_mint: ScriptMint, index: u64) -> Result<(), WError>;
    fn add_native_mint(&mut self, native_mint: SimpleScriptMint) -> Result<(), WError>;
    fn add_cert(
        &mut self,
        // certificates_builder: &mut csl::CertificatesBuilder,
        cert: Certificate,
        index: u64,
    ) -> Result<(), WError>;
    fn add_vote(
        &mut self,
        // vote_builder: &mut csl::VotingBuilder,
        vote: Vote,
        index: u64,
    ) -> Result<(), WError>;
    fn add_invalid_before(&mut self, invalid_before: u64);
    fn add_invalid_hereafter(&mut self, invalid_hereafter: u64);
    fn set_fee(&mut self, fee: String);
    fn add_change(
        &mut self,
        change_address: String,
        change_datum: Option<Datum>,
    ) -> Result<(), WError>;
    fn add_signing_keys(&mut self, signing_keys: &[&str]) -> Result<(), WError>;
    fn add_required_signature(&mut self, pub_key_hash: &str) -> Result<(), WError>;
    fn add_metadata(&mut self, metadata: Metadata) -> Result<(), WError>;
    fn add_script_hash(&mut self, network: Network) -> Result<(), WError>;
    fn build_tx(&mut self) -> Result<String, WError>;
}
