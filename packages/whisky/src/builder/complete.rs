use crate::*;
use futures::future;
use uplc::tx::SlotConfig;

use super::{TxBuilder, TxEvaluation};

impl TxBuilder {
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
    /// * `Self` - The TxBuilder instance
    pub async fn complete(
        &mut self,
        customized_tx: Option<TxBuilderBody>,
    ) -> Result<&mut Self, WError> {
        // Check and complete all inputs
        self.complete_tx_parts().await?;

        let inputs = self.tx_builder_body.inputs.clone();
        for input in inputs {
            self.input_for_evaluation(&input.to_utxo());
        }

        // NOTE: need add sanitize outputs?

        self.complete_sync(customized_tx)?;
        match &self.evaluator {
            Some(evaluator) => {
                let network = match &self.tx_builder_body.network {
                    Some(builder_network) => builder_network,
                    None => &Network::Mainnet,
                };
                let inputs_for_evaluation: Vec<_> =
                    self.inputs_for_evaluation.values().cloned().collect();
                let tx_evaluation_result = evaluator
                    .evaluate_tx(
                        &self.serializer.tx_hex,
                        &inputs_for_evaluation,
                        &self.chained_txs.clone(),
                        network,
                        &SlotConfig::default(), // TODO: accept slot config as argument for evaluator
                    )
                    .await;
                match tx_evaluation_result {
                    Ok(actions) => self.update_redeemer(actions),
                    Err(err) => {
                        return Err(WError::new(
                            "TxBuilder - complete",
                            &format!(
                        "Error evaluating transaction - tx_hex: [ {} ] , Error message: [ {:?} ]",
                        self.tx_hex(),
                        err
                    ),
                        ))
                    }
                }
            }
            None => self,
        };
        self.complete_sync(None)
    }

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
    /// * `Self` - The TxBuilder instance
    pub fn complete_sync(
        &mut self,
        customized_tx: Option<TxBuilderBody>,
    ) -> Result<&mut Self, WError> {
        if customized_tx.is_some() {
            self.tx_builder_body = customized_tx.unwrap();
        } else {
            self.queue_all_last_item();
            if !self.extra_inputs.is_empty() {
                self.add_utxos_from(self.extra_inputs.clone(), self.selection_threshold)?;
            }
        }

        self.tx_builder_body.mints.sort_by(|a, b| {
            let a_mint = match a {
                MintItem::ScriptMint(a_script_mint) => &a_script_mint.mint,
                MintItem::SimpleScriptMint(a_simple_script_mint) => &a_simple_script_mint.mint,
            };
            let b_mint = match b {
                MintItem::ScriptMint(b_script_mint) => &b_script_mint.mint,
                MintItem::SimpleScriptMint(b_simple_script_mint) => &b_simple_script_mint.mint,
            };
            a_mint.policy_id.cmp(&b_mint.policy_id)
        });

        self.tx_builder_body.inputs.sort_by(|a, b| {
            let tx_in_data_a: &TxInParameter = match a {
                TxIn::PubKeyTxIn(pub_key_tx_in) => &pub_key_tx_in.tx_in,
                TxIn::SimpleScriptTxIn(simple_script_tx_in) => &simple_script_tx_in.tx_in,
                TxIn::ScriptTxIn(script_tx_in) => &script_tx_in.tx_in,
            };

            let tx_in_data_b: &TxInParameter = match b {
                TxIn::PubKeyTxIn(pub_key_tx_in) => &pub_key_tx_in.tx_in,
                TxIn::SimpleScriptTxIn(simple_script_tx_in) => &simple_script_tx_in.tx_in,
                TxIn::ScriptTxIn(script_tx_in) => &script_tx_in.tx_in,
            };

            tx_in_data_a
                .tx_hash
                .cmp(&tx_in_data_b.tx_hash)
                .then_with(|| tx_in_data_a.tx_index.cmp(&tx_in_data_b.tx_index))
        });

        let tx_hex = self
            .serializer
            .set_protocol_params(self.protocol_params.clone().unwrap_or_default())
            .set_tx_builder_body(self.tx_builder_body.clone())
            .serialize_tx_body()?;
        self.serializer.reset_builder();
        self.serializer.tx_hex = tx_hex;
        Ok(self)
    }

    async fn complete_tx_parts(&mut self) -> Result<&mut Self, WError> {
        Self::queue_all_last_item(self);

        let incomplete_tx_ins: Vec<TxIn> = self
            .tx_builder_body
            .inputs
            .iter()
            .cloned()
            .chain(
                self.tx_builder_body
                    .collaterals
                    .iter()
                    .cloned()
                    .map(TxIn::PubKeyTxIn),
            )
            .filter(|tx_in| !Self::is_input_complete(tx_in))
            .collect();

        self.query_all_tx_info(&incomplete_tx_ins).await?;

        for tx_in in incomplete_tx_ins {
            self.complete_tx_in_info(&tx_in)?;
        }

        if self.tx_builder_body.inputs.is_empty() && self.tx_builder_body.collaterals.is_empty() {
            return Err(WError::new(
                "TxBuilder - complete_tx_parts",
                "No inputs or collaterals provided for the transaction",
            ));
        }

        Ok(self)
    }
    async fn query_all_tx_info(&mut self, incomplete_tx_ins: &Vec<TxIn>) -> Result<(), WError> {
        if (!incomplete_tx_ins.is_empty()) && self.fetcher.is_none() {
            return Err(WError::new(
                "TxBuilder - complete_tx_parts",
                "Transaction information is incomplete while no fetcher instance is provided. Provide a `fetcher`.",
            ));
        }

        let mut to_be_queried: Vec<&str> = Vec::new();

        for current_tx_in in incomplete_tx_ins {
            let tx_hash = match current_tx_in {
                TxIn::PubKeyTxIn(pub_key_tx_in) => &pub_key_tx_in.tx_in.tx_hash,
                TxIn::SimpleScriptTxIn(simple_script_tx_in) => &simple_script_tx_in.tx_in.tx_hash,
                TxIn::ScriptTxIn(script_tx_in) => &script_tx_in.tx_in.tx_hash,
            };

            if !self.queried_tx_hashes.contains(tx_hash) {
                to_be_queried.push(tx_hash);
                self.queried_tx_hashes.insert(tx_hash.to_string());
            }
        }

        let futures_vec = to_be_queried
            .into_iter()
            .map(|tx_hash| self.fetcher.as_ref().unwrap().fetch_utxos(tx_hash, None))
            .collect::<Vec<_>>();

        let results = future::join_all(futures_vec).await;
        for result in &results {
            match result {
                Err(e) => {
                    return Err(WError::new(
                        "TxBuilder - query_all_tx_info",
                        &format!("Failed to query transaction information: {}", e),
                    ));
                }

                Ok(utxos) => {
                    self.queried_utxos
                        .insert(utxos.first().unwrap().input.tx_hash.clone(), utxos.clone());
                }
            }
        }

        Ok(())
    }

    fn complete_tx_in_info(&mut self, tx_in: &TxIn) -> Result<(), WError> {
        if !Self::is_input_info_complete(&tx_in) {
            self.complete_input_info(tx_in)?;
        }
        Ok(())
    }

    fn complete_input_info(&mut self, tx_in: &TxIn) -> Result<(), WError> {
        let (tx_hash, tx_index) = match &tx_in {
            TxIn::PubKeyTxIn(pub_key_tx_in) => (
                pub_key_tx_in.tx_in.tx_hash.clone(),
                pub_key_tx_in.tx_in.tx_index.clone(),
            ),
            TxIn::SimpleScriptTxIn(simple_script_tx_in) => (
                simple_script_tx_in.tx_in.tx_hash.clone(),
                simple_script_tx_in.tx_in.tx_index.clone(),
            ),
            TxIn::ScriptTxIn(script_tx_in) => (
                script_tx_in.tx_in.tx_hash.clone(),
                script_tx_in.tx_in.tx_index.clone(),
            ),
        };

        let utxos = self.queried_utxos.get(&tx_hash).ok_or_else(|| {
            WError::new(
                "TxBuilder - complete_input_info",
                &format!("No UTxOs found for {}", tx_hash),
            )
        })?;

        let utxo = utxos
            .iter()
            .find(|utxo| utxo.input.output_index == tx_index)
            .ok_or_else(|| {
                WError::new(
                    "TxBuilder - complete_input_info",
                    &format!("Couldn't find UTxO for {}#{}", tx_hash, tx_index),
                )
            })?;

        let amount = &utxo.output.amount;
        let address = &utxo.output.address;

        if amount.is_empty() {
            return Err(WError::new(
                "TxBuilder - complete_input_info",
                &format!(
                    "Couldn't find value information for {}#{}",
                    tx_hash, tx_index
                ),
            ));
        }

        if address.is_empty() {
            return Err(WError::new(
                "TxBuilder - complete_input_info",
                &format!(
                    "Couldn't find address information for {}#{}",
                    tx_hash, tx_index
                ),
            ));
        }

        for input in &mut self.tx_builder_body.inputs {
            match input {
                TxIn::PubKeyTxIn(pub_key_tx_in)
                    if pub_key_tx_in.tx_in.tx_hash == tx_hash
                        && pub_key_tx_in.tx_in.tx_index == tx_index =>
                {
                    pub_key_tx_in.tx_in.amount = Some(amount.clone());
                    pub_key_tx_in.tx_in.address = Some(address.clone());
                }
                TxIn::SimpleScriptTxIn(simple_script_tx_in)
                    if simple_script_tx_in.tx_in.tx_hash == tx_hash
                        && simple_script_tx_in.tx_in.tx_index == tx_index =>
                {
                    simple_script_tx_in.tx_in.amount = Some(amount.clone());
                    simple_script_tx_in.tx_in.address = Some(address.clone());
                }
                TxIn::ScriptTxIn(script_tx_in)
                    if script_tx_in.tx_in.tx_hash == tx_hash
                        && script_tx_in.tx_in.tx_index == tx_index =>
                {
                    script_tx_in.tx_in.amount = Some(amount.clone());
                    script_tx_in.tx_in.address = Some(address.clone());
                }
                _ => {}
            }
        }

        for input in &mut self.tx_builder_body.collaterals {
            if input.tx_in.tx_hash == tx_hash && input.tx_in.tx_index == tx_index {
                input.tx_in.amount = Some(amount.clone());
                input.tx_in.address = Some(address.clone());
            }
        }

        Ok(())
    }

    fn is_input_complete(tx_in: &TxIn) -> bool {
        match tx_in {
            TxIn::PubKeyTxIn(_) => Self::is_input_info_complete(tx_in),
            TxIn::SimpleScriptTxIn(_) => true,
            TxIn::ScriptTxIn(script_tx_in) => {
                if let Some(script_source) = &script_tx_in.script_tx_in.script_source {
                    Self::is_input_info_complete(tx_in)
                        && Self::is_ref_script_into_complete(script_source)
                } else {
                    false
                }
            }
        }
    }

    fn is_input_info_complete(tx_in: &TxIn) -> bool {
        match tx_in {
            TxIn::PubKeyTxIn(pub_key_tx_in) => {
                pub_key_tx_in.tx_in.amount.is_some() && pub_key_tx_in.tx_in.address.is_some()
            }
            TxIn::SimpleScriptTxIn(simple_script_tx_in) => {
                simple_script_tx_in.tx_in.amount.is_some()
                    && simple_script_tx_in.tx_in.address.is_some()
            }
            TxIn::ScriptTxIn(script_tx_in) => script_tx_in.script_tx_in.script_source.is_some(),
        }
    }

    fn is_ref_script_into_complete(script_source: &ScriptSource) -> bool {
        match script_source {
            ScriptSource::ProvidedScriptSource(_) => true,
            ScriptSource::InlineScriptSource(inline_script_source) => {
                if inline_script_source.script_hash.is_empty()
                    || inline_script_source.script_size == 0
                {
                    false
                } else {
                    true
                }
            }
        }
    }
    /// ## Transaction building method
    ///
    /// Complete the signing process
    ///
    /// ### Returns
    ///
    /// * `String` - The signed transaction in hex
    pub fn complete_signing(&mut self) -> Result<String, WError> {
        self.serializer.complete_signing()
    }

    /// ## Transaction building method
    ///
    /// Obtain the transaction hex
    ///
    /// ### Returns
    ///
    /// * tx_hex - The current transaction hex from build
    pub fn tx_hex(&mut self) -> String {
        self.serializer.tx_hex()
    }
}
