use crate::model::{Action, Redeemer, RedeemerTag, ScriptTxIn, TxIn};

use super::MeshTxBuilder;

pub trait ITxEvaluation {
    fn update_redeemer(&mut self, tx_evaluation: Vec<Action>) -> &mut Self;
}

impl ITxEvaluation for MeshTxBuilder {
    fn update_redeemer(&mut self, tx_evaluation: Vec<Action>) -> &mut Self {
        let multiplier = self.tx_evaluation_multiplier_percentage;
        for redeemer_evaluation in tx_evaluation {
            match redeemer_evaluation.tag {
                RedeemerTag::Spend => {
                    let input =
                        &mut self.mesh_tx_builder_body.inputs[redeemer_evaluation.index as usize];
                    match input {
                        TxIn::ScriptTxIn(ScriptTxIn { script_tx_in, .. }) => {
                            let redeemer: &mut Redeemer = script_tx_in.redeemer.as_mut().unwrap();
                            redeemer.ex_units.mem =
                                redeemer_evaluation.budget.mem * multiplier / 100;
                            redeemer.ex_units.steps =
                                redeemer_evaluation.budget.steps * multiplier / 100;
                        }
                        _ => {}
                    }
                }
                RedeemerTag::Mint => {
                    let mint =
                        &mut self.mesh_tx_builder_body.mints[redeemer_evaluation.index as usize];
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
                _ => {}
            }
        }
        self
    }
}
