use async_trait::async_trait;
use cardano_serialization_lib::JsError;

use sidan_csl_rs::model::{Action, Redeemer, RedeemerTag, ScriptTxIn, TxIn, UTxO};

use crate::builder::MeshTxBuilder;

#[async_trait]
pub trait IEvaluator: Send {
    async fn evaluate_tx(
        &self,
        tx_hex: &str,
        inputs: &[UTxO],           // Change the type from &Vec<UTxO> to &[UTxO]
        additional_txs: &[String], // Change the type from &Vec<String> to &[String]
    ) -> Result<Vec<Action>, JsError>;

    fn evaluate_tx_sync(
        &self,
        tx_hex: &str,
        inputs: &[UTxO],           // Change the type from &Vec<UTxO> to &[UTxO]
        additional_txs: &[String], // Change the type from &Vec<String> to &[String]
    ) -> Result<Vec<Action>, JsError>;
}

pub trait ITxEvaluation {
    fn update_redeemer(&mut self, tx_evaluation: Vec<Action>) -> &mut Self;
}

impl ITxEvaluation for MeshTxBuilder {
    fn update_redeemer(&mut self, tx_evaluation: Vec<Action>) -> &mut Self {
        let multiplier = self.core.tx_evaluation_multiplier_percentage;
        for redeemer_evaluation in tx_evaluation {
            match redeemer_evaluation.tag {
                RedeemerTag::Spend => {
                    let input = &mut self.core.mesh_tx_builder_body.inputs
                        [redeemer_evaluation.index as usize];
                    if let TxIn::ScriptTxIn(ScriptTxIn { script_tx_in, .. }) = input {
                        let redeemer: &mut Redeemer = script_tx_in.redeemer.as_mut().unwrap();
                        redeemer.ex_units.mem = redeemer_evaluation.budget.mem * multiplier / 100;
                        redeemer.ex_units.steps =
                            redeemer_evaluation.budget.steps * multiplier / 100;
                    }
                }
                RedeemerTag::Mint => {
                    let mint = &mut self.core.mesh_tx_builder_body.mints
                        [redeemer_evaluation.index as usize];
                    let redeemer: &mut Redeemer = mint.redeemer.as_mut().unwrap();
                    redeemer.ex_units.mem = redeemer_evaluation.budget.mem * multiplier / 100;
                    redeemer.ex_units.steps = redeemer_evaluation.budget.steps * multiplier / 100;
                }
                RedeemerTag::Cert => {
                    // TODO
                }
                RedeemerTag::Reward => {
                    // TODO
                }
            }
        }
        self
    }
}
