use crate::*;
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
                        &self.serializer.tx_hex(),
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
        self.serializer
            .set_protocol_params(self.protocol_params.clone().unwrap_or_default());
        self.serializer
            .set_tx_builder_body(self.tx_builder_body.clone());
        let tx_hex = self.serializer.serialize_tx_body()?;
        self.serializer.reset_builder();
        self.serializer.set_tx_hex(tx_hex);
        Ok(self)
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
