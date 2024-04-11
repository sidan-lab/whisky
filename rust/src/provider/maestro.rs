use async_trait::async_trait;
use cardano_serialization_lib::error::JsError;
use maestro::{models::transactions::RedeemerEvaluation, Maestro};

use crate::{
    model::{Action, Budget, RedeemerTag},
    service::IEvaluator,
};

pub struct MaestroProvider {
    pub maestro_client: Maestro,
}

impl MaestroProvider {
    pub fn new(api_key: String, network: String) -> MaestroProvider {
        let maestro_client = Maestro::new(api_key, network);
        MaestroProvider { maestro_client }
    }
}

#[async_trait]
impl IEvaluator for MaestroProvider {
    async fn evaluate_tx(&self, tx: String) -> Result<Vec<Action>, JsError> {
        match self.maestro_client.evaluate_tx(&tx, vec![]).await {
            Ok(redeemer_evaluation) => Ok(redeemer_evaluation
                .iter()
                .map(|r: &RedeemerEvaluation| Action {
                    index: r.redeemer_index as u16,
                    budget: Budget {
                        mem: r.ex_units.mem as u64,
                        steps: r.ex_units.steps as u64,
                    },
                    tag: match r.redeemer_tag.as_str() {
                        "spend" => RedeemerTag::Spend,
                        "mint" => RedeemerTag::Mint,
                        "cert" => RedeemerTag::Cert,
                        "wdrl" => RedeemerTag::Reward,
                        _ => panic!("Unknown redeemer tag from maestro service"),
                    },
                })
                .collect()),
            Err(e) => Err(JsError::from_str(&format!("{}", e))),
        }
    }
}
