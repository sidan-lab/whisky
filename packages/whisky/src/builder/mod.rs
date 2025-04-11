mod certificate;
mod complete;
mod data;
mod mint;
mod service;
mod tx_eval;
mod tx_in;
mod tx_out;
mod vote;
mod withdrawal;

use std::collections::HashMap;

use crate::*;
pub use data::*;
pub use tx_eval::*;

use crate::services::*;

pub struct TxBuilder {
    pub serializer: WhiskyCSL,
    pub tx_builder_body: TxBuilderBody,
    pub protocol_params: Option<Protocol>,
    pub tx_in_item: Option<TxIn>,
    pub withdrawal_item: Option<Withdrawal>,
    pub vote_item: Option<Vote>,
    pub mint_item: Option<MintItem>,
    pub collateral_item: Option<PubKeyTxIn>,
    pub tx_output: Option<Output>,
    pub adding_script_input: Option<LanguageVersion>,
    pub adding_plutus_mint: Option<LanguageVersion>,
    pub adding_plutus_withdrawal: Option<LanguageVersion>,
    pub adding_plutus_vote: Option<LanguageVersion>,
    pub fetcher: Option<Box<dyn Fetcher>>,
    pub evaluator: Option<Box<dyn Evaluator>>,
    pub submitter: Option<Box<dyn Submitter>>,
    pub extra_inputs: Vec<UTxO>,
    pub selection_threshold: u64,
    pub chained_txs: Vec<String>,
    pub inputs_for_evaluation: HashMap<String, UTxO>,
}

pub struct TxBuilderParam {
    pub evaluator: Option<Box<dyn Evaluator>>,
    pub fetcher: Option<Box<dyn Fetcher>>,
    pub submitter: Option<Box<dyn Submitter>>,
    pub params: Option<Protocol>,
}

impl TxBuilder {
    /// ## Transaction building method
    ///
    /// Create a new TxBuilder instance with option params
    ///
    /// ### Arguments
    ///
    /// * `param` - Parameters for setting up the TxBuilder instance, including evaluator, fetcher, submitter, and protocol parameters
    ///
    /// ### Returns
    ///
    /// * `Self` - A new TxBuilder instance
    pub fn new(param: TxBuilderParam) -> Self {
        TxBuilder {
            serializer: WhiskyCSL::new(param.params.clone()).unwrap(),
            tx_builder_body: TxBuilderBody::new(),
            protocol_params: param.params.clone(),
            tx_in_item: None,
            withdrawal_item: None,
            vote_item: None,
            mint_item: None,
            collateral_item: None,
            tx_output: None,
            adding_script_input: None,
            adding_plutus_mint: None,
            adding_plutus_withdrawal: None,
            adding_plutus_vote: None,
            fetcher: param.fetcher,
            evaluator: match param.evaluator {
                Some(evaluator) => Some(evaluator),
                None => Some(Box::new(MeshTxEvaluator::new())),
            },
            submitter: param.submitter,
            extra_inputs: vec![],
            selection_threshold: 5_000_000,
            chained_txs: vec![],
            inputs_for_evaluation: HashMap::new(),
        }
    }

    /// ## Transaction building method
    ///
    /// Create a new TxBuilder instance without option params
    ///
    /// ### Returns
    ///
    /// * `Self` - A new TxBuilder instance
    pub fn new_core() -> Self {
        Self::new(TxBuilderParam {
            evaluator: None,
            fetcher: None,
            submitter: None,
            params: None,
        })
    }

    /// ## Transaction building method
    ///
    /// Add a required signer hash to the TxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `pub_key_hash` - The public key hash
    ///
    /// ### Returns
    ///
    /// * `Self` - The TxBuilder instance
    pub fn required_signer_hash(&mut self, pub_key_hash: &str) -> &mut Self {
        self.tx_builder_body
            .required_signatures
            .push(pub_key_hash.to_string());
        self
    }

    /// ## Transaction building method
    ///
    /// Change the address in the TxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `address` - The new address
    ///
    /// ### Returns
    ///
    /// * `Self` - The TxBuilder instance
    pub fn change_address(&mut self, address: &str) -> &mut Self {
        self.tx_builder_body.change_address = address.to_string();
        self
    }

    /// ## Transaction building method
    ///
    /// Change the output datum in the TxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `data` - The new output datum
    ///
    /// ### Returns
    ///
    /// * `Self` - The TxBuilder instance
    pub fn change_output_datum(&mut self, data: WData) -> &mut Self {
        match data.to_cbor() {
            Ok(raw_data) => {
                self.tx_builder_body.change_datum = Some(Datum::Inline(raw_data));
            }
            Err(_) => {
                panic!("Error converting datum to CBOR");
            }
        }
        self
    }

    /// ## Transaction building method
    ///
    /// Set the invalid_before slot in the TxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `slot` - The new invalid_before slot
    ///
    /// ### Returns
    ///
    /// * `Self` - The TxBuilder instance
    pub fn invalid_before(&mut self, slot: u64) -> &mut Self {
        self.tx_builder_body.validity_range.invalid_before = Some(slot);
        self
    }

    /// ## Transaction building method
    ///
    /// Set the invalid_hereafter slot in the TxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `slot` - The new invalid_hereafter slot
    ///
    /// ### Returns
    ///
    /// * `Self` - The TxBuilder instance
    pub fn invalid_hereafter(&mut self, slot: u64) -> &mut Self {
        self.tx_builder_body.validity_range.invalid_hereafter = Some(slot);
        self
    }

    /// ## Transaction building method
    ///
    /// Add a metadata value to the TxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `tag` - The tag for the metadata
    /// * `metadata` - The metadata value
    ///
    /// ### Returns
    ///
    /// * `Self` - The TxBuilder instance
    pub fn metadata_value(&mut self, tag: &str, metadata: &str) -> &mut Self {
        self.tx_builder_body.metadata.push(Metadata {
            tag: tag.to_string(),
            metadata: metadata.to_string(),
        });
        self
    }

    /// ## Transaction building method
    ///
    /// Add a cli signing key to the TxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `skey_hex` - The signing key in hexadecimal
    ///
    /// ### Returns
    ///
    /// * `Self` - The TxBuilder instance
    pub fn signing_key(&mut self, skey_hex: &str) -> &mut Self {
        self.tx_builder_body.signing_key.push(skey_hex.to_string());
        self
    }

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
    /// * `Self` - The TxBuilder instance
    pub fn chain_tx(&mut self, tx_hex: &str) -> &mut Self {
        self.chained_txs.push(tx_hex.to_string());
        self
    }

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
    /// * `Self` - The TxBuilder instance
    pub fn input_for_evaluation(&mut self, input: &UTxO) -> &mut Self {
        let utxo_id = format!("{}{}", input.input.tx_hash, input.input.output_index);
        let current_utxo = self.inputs_for_evaluation.get(&utxo_id);
        match current_utxo {
            Some(current_utxo) => {
                let UtxoOutput {
                    address,
                    amount,
                    data_hash,
                    plutus_data,
                    script_ref,
                    script_hash,
                } = input.clone().output;
                let UtxoOutput {
                    data_hash: current_data_hash,
                    plutus_data: current_plutus_data,
                    script_ref: current_script_ref,
                    script_hash: current_script_hash,
                    ..
                } = current_utxo.output.clone();
                let updated_utxo = UTxO {
                    output: UtxoOutput {
                        address,
                        amount,
                        data_hash: match data_hash {
                            Some(_) => data_hash,
                            None => current_data_hash,
                        },
                        plutus_data: match plutus_data {
                            Some(_) => plutus_data,
                            None => current_plutus_data,
                        },
                        script_ref: match script_ref {
                            Some(_) => script_ref,
                            None => current_script_ref,
                        },
                        script_hash: match script_hash {
                            Some(_) => script_hash,
                            None => current_script_hash,
                        },
                    },
                    ..input.clone()
                };
                self.inputs_for_evaluation.insert(utxo_id, updated_utxo);
            }
            None => {
                self.inputs_for_evaluation.insert(utxo_id, input.clone());
            }
        }
        self
    }

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
    /// * `Self` - The TxBuilder instance
    pub fn select_utxos_from(&mut self, extra_inputs: &[UTxO], threshold: u64) -> &mut Self {
        self.selection_threshold = threshold;
        self.extra_inputs.extend(extra_inputs.to_vec());
        self
    }

    /// ## Transaction building method
    ///
    /// Sets a specific fee amount, if the fee is insufficient, the transaction will fail to build
    ///
    /// ### Arguments
    ///
    /// * `fee` - The fee amount
    ///
    /// ### Returns
    ///
    /// * `Self` - The TxBuilder instance
    pub fn set_fee(&mut self, fee: &str) -> &mut Self {
        self.tx_builder_body.fee = Some(fee.to_string());
        self
    }

    /// ## Transaction building method
    ///
    /// Selects the network to use, primarily to decide which cost models to use for evaluation and calculating script integrity hash
    ///
    /// ### Arguments
    ///
    /// * `network` - The network the current Tx is being built for. Custom Network takes in a vec of cost models
    ///
    /// ### Returns
    ///
    /// * `Self` - The TxBuilder instance
    pub fn network(&mut self, network: Network) -> &mut Self {
        self.tx_builder_body.network = Some(network);
        self
    }

    /// ## Internal method
    ///
    /// Queue an input in the TxBuilder instance
    pub fn queue_input(&mut self) {
        let tx_in_item = self.tx_in_item.clone().unwrap();
        match tx_in_item {
            TxIn::ScriptTxIn(tx_in) => {
                match (
                    tx_in.script_tx_in.datum_source,
                    tx_in.script_tx_in.redeemer,
                    tx_in.script_tx_in.script_source,
                ) {
                    (None, _, _) => panic!("Datum in a script input cannot be None"),
                    (_, None, _) => panic!("Redeemer in script input cannot be None"),
                    (_, _, None) => panic!("Script source in script input cannot be None"),
                    _ => {}
                }
            }
            TxIn::SimpleScriptTxIn(_) => {}
            TxIn::PubKeyTxIn(_) => {}
        }
        let input = self.tx_in_item.clone().unwrap();
        self.input_for_evaluation(&input.to_utxo());
        self.tx_builder_body.inputs.push(input);
        self.tx_in_item = None
    }

    /// ## Internal method
    ///
    /// Queue a withdrawal in the TxBuilder instance
    pub fn queue_withdrawal(&mut self) {
        let withdrawal_item = self.withdrawal_item.clone().unwrap();
        match withdrawal_item {
            Withdrawal::PlutusScriptWithdrawal(withdrawal) => {
                match (withdrawal.redeemer, withdrawal.script_source) {
                    (None, _) => panic!("Redeemer in script withdrawal cannot be None"),
                    (_, None) => panic!("Script source in script withdrawal cannot be None"),
                    _ => {}
                }
            }
            Withdrawal::SimpleScriptWithdrawal(withdrawal) => {
                if withdrawal.script_source.is_none() {
                    panic!("Script source missing from native script withdrawal")
                }
            }
            Withdrawal::PubKeyWithdrawal(_) => {}
        }
        self.tx_builder_body
            .withdrawals
            .push(self.withdrawal_item.clone().unwrap());
        self.withdrawal_item = None;
    }

    /// ## Internal method
    ///
    /// Queue a vote in the TxBuilder instance
    pub fn queue_vote(&mut self) {
        let vote_item = self.vote_item.clone().unwrap();
        match vote_item {
            Vote::ScriptVote(script_vote) => {
                match (script_vote.redeemer, script_vote.script_source) {
                    (None, _) => panic!("Redeemer in script vote cannot be None"),
                    (_, None) => panic!("Script source in script vote cannot be None"),
                    _ => {}
                }
            }
            Vote::SimpleScriptVote(simple_script_vote) => {
                if simple_script_vote.simple_script_source.is_none() {
                    panic!("Script source is missing from native script vote")
                }
            }
            Vote::BasicVote(_) => {}
        }
        self.tx_builder_body
            .votes
            .push(self.vote_item.clone().unwrap());
        self.vote_item = None;
    }

    /// ## Internal method
    ///
    /// Queue a mint in the TxBuilder instance
    pub fn queue_mint(&mut self) {
        let mint_item = self.mint_item.take().unwrap();
        match mint_item {
            MintItem::ScriptMint(script_mint) => {
                if script_mint.script_source.is_none() {
                    panic!("Missing mint script information");
                }
                self.tx_builder_body
                    .mints
                    .push(MintItem::ScriptMint(script_mint));
            }
            MintItem::SimpleScriptMint(simple_script_mint) => {
                if simple_script_mint.script_source.is_none() {
                    panic!("Missing mint script information");
                }
                self.tx_builder_body
                    .mints
                    .push(MintItem::SimpleScriptMint(simple_script_mint));
            }
        }
        self.mint_item = None;
    }

    /// ## Internal method
    ///
    /// Queue all last items in the TxBuilder instance
    pub fn queue_all_last_item(&mut self) {
        if self.tx_output.is_some() {
            self.tx_builder_body
                .outputs
                .push(self.tx_output.clone().unwrap());
            self.tx_output = None;
        }
        if self.tx_in_item.is_some() {
            self.queue_input();
        }
        if self.collateral_item.is_some() {
            self.tx_builder_body
                .collaterals
                .push(self.collateral_item.clone().unwrap());
            self.collateral_item = None;
        }
        if self.withdrawal_item.is_some() {
            self.queue_withdrawal();
        }
        if self.vote_item.is_some() {
            self.queue_vote();
        }
        if self.mint_item.is_some() {
            self.queue_mint();
        }
    }

    /// ## Internal method
    ///
    /// Perform the utxo selection process
    ///
    /// ### Arguments
    ///
    /// * `extra_inputs` - A vector of extra inputs provided
    /// * `threshold` - The threshold as configured
    pub fn add_utxos_from(
        &mut self,
        extra_inputs: Vec<UTxO>,
        threshold: u64,
    ) -> Result<(), WError> {
        let mut required_assets = Value::new();

        for output in &self.tx_builder_body.outputs {
            let mut output_value = Value::from_asset_vec(&output.amount);
            let pp = self.protocol_params.clone().unwrap_or_default();
            if output_value.get("lovelace") == 0 {
                output_value.merge(&Value::from_asset(&Asset::new(
                    "lovelace".to_string(),
                    get_min_utxo_value(output, &pp.coins_per_utxo_size)?,
                )));
            }
            required_assets.merge(&output_value);
        }
        for input in &self.tx_builder_body.inputs {
            match input {
                TxIn::PubKeyTxIn(pub_key_tx_in) => {
                    let input_value =
                        Value::from_asset_vec(pub_key_tx_in.tx_in.amount.as_ref().unwrap());
                    required_assets.negate_value(&input_value);
                }
                TxIn::SimpleScriptTxIn(simple_script_tx_in) => {
                    let input_value =
                        Value::from_asset_vec(simple_script_tx_in.tx_in.amount.as_ref().unwrap());
                    required_assets.negate_value(&input_value);
                }
                TxIn::ScriptTxIn(script_tx_in) => {
                    let input_value =
                        Value::from_asset_vec(script_tx_in.tx_in.amount.as_ref().unwrap());
                    required_assets.negate_value(&input_value);
                }
            }
        }

        for mint_item in &self.tx_builder_body.mints {
            let mint = match mint_item {
                MintItem::ScriptMint(script_mint) => &script_mint.mint,
                MintItem::SimpleScriptMint(simple_script_mint) => &simple_script_mint.mint,
            };

            required_assets.negate_asset(
                &format!("{}{}", mint.policy_id, &mint.asset_name),
                mint.amount as u64,
            );
        }

        let selected_inputs = select_utxos(&extra_inputs, required_assets, &threshold.to_string())
            .map_err(WError::from_err("add_txos_from - select_utxos"))?;

        for input in selected_inputs {
            self.serializer.core.add_tx_in(PubKeyTxIn {
                tx_in: TxInParameter {
                    tx_hash: input.input.tx_hash.clone(),
                    tx_index: input.input.output_index,
                    amount: Some(input.output.amount.clone()),
                    address: Some(input.output.address.clone()),
                },
            })?;
            let pub_key_input = TxIn::PubKeyTxIn(PubKeyTxIn {
                tx_in: TxInParameter {
                    tx_hash: input.input.tx_hash.clone(),
                    tx_index: input.input.output_index,
                    amount: Some(input.output.amount.clone()),
                    address: Some(input.output.address.clone()),
                },
            });
            self.tx_builder_body.inputs.push(pub_key_input.clone());
            self.input_for_evaluation(&input);
        }
        Ok(())
    }
}

impl Default for TxBuilder {
    fn default() -> Self {
        Self::new_core()
    }
}
