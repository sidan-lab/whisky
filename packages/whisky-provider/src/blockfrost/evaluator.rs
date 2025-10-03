use async_trait::async_trait;
use whisky_csl::CSLParser;

use uplc::tx::SlotConfig;
use whisky_common::models::{Network, UTxO};

use whisky_common::*;

use super::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Debug)]
pub struct AdditionalUtxo {
    tx_hash: String,
    index: u32,
    address: String,
    value: Vec<Asset>,
}

impl AdditionalUtxo {
    pub fn to_ogmios(&self) -> serde_json::Value {
        let mut value: HashMap<String, u64> = HashMap::new();
        self.value.iter().for_each(|asset| {
            value.insert(asset.unit().clone(), asset.quantity_i128() as u64);
        });

        serde_json::json!([
            {
                "transaction": {"id": self.tx_hash},
                "output": {"index": self.index},
            },
            {
                "address": self.address,
                "value": value
            }
        ])
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EvaluateTx {
    cbor: String,
    additional_utxo_set: Vec<serde_json::Value>,
}
#[derive(Serialize, Deserialize)]
pub struct OgmiosBudget {
    pub memory: u64,
    pub steps: u64,
}

#[async_trait]
impl Evaluator for BlockfrostProvider {
    async fn evaluate_tx(
        &self,
        tx: &str,
        _inputs: &[UTxO],
        additional_txs: &[String],
        _network: &Network,
        _slot_config: &SlotConfig,
    ) -> Result<Vec<Action>, WError> {
        let mut tx_out_cbors = Vec::new();

        for tx_str in additional_txs {
            let utxos = CSLParser::extract_output_utxos(tx_str).map_err(|e| {
                WError::new("evaluate_tx", &format!("Failed to get output UTXOs: {}", e))
            })?;

            for (index, utxo) in utxos.iter().enumerate() {
                let additional_utxo = AdditionalUtxo {
                    tx_hash: utxo.input.tx_hash.to_string(),
                    index: index as u32,
                    address: utxo.output.address.to_string(),
                    value: utxo.output.amount.clone(),
                };
                tx_out_cbors.push(additional_utxo.to_ogmios());
            }
        }

        let url = "/utils/txs/evaluate/utxos";
        let body = EvaluateTx {
            cbor: tx.to_string(),
            additional_utxo_set: tx_out_cbors,
        };

        let resp = self
            .blockfrost_client
            .post(url, &body)
            .await
            .map_err(WError::from_err("Blockfrost - evaluate_tx"))?;

        let parsed_json: serde_json::Value = serde_json::from_str(&resp)
            .map_err(WError::from_err("Blockfrost - evaluate_tx type error"))?;

        println!("parsed_json: {:?}", parsed_json);

        let evaluation_result = parsed_json
            .get("result")
            .ok_or_else(WError::from_opt(
                "Blockfrost - evaluate_tx",
                "failed to get EvaluationResult from resp",
            ))?
            .get("EvaluationResult")
            .ok_or_else(WError::from_opt(
                "Blockfrost - evaluate_tx",
                "failed to get EvaluationResult from resp",
            ))?;

        let evaluation_map: HashMap<String, OgmiosBudget> =
            serde_json::from_value(evaluation_result.clone())
                .map_err(WError::from_err("Blockfrost - evaluate_tx type error"))?;

        println!("Blockfrost evaluate_tx response: {}", resp);

        let actions: Result<Vec<Action>, WError> = evaluation_map
            .into_iter()
            .map(|(key, budget)| {
                // Parse the key to extract the tag and index
                let parts: Vec<&str> = key.split(':').collect();
                if parts.len() != 2 {
                    return Err(WError::new(
                        "evaluate_tx",
                        &format!("Invalid key format: {}", key),
                    ));
                }
                let tag = match parts[0] {
                    "spend" => RedeemerTag::Spend,
                    "mint" => RedeemerTag::Mint,
                    "cert" => RedeemerTag::Cert,
                    "reward" => RedeemerTag::Reward,
                    "vote" => RedeemerTag::Vote,
                    "propose" => RedeemerTag::Propose,
                    _ => {
                        return Err(WError::new(
                            "evaluate_tx",
                            &format!("Unknown tag: {}", parts[0]),
                        ))
                    }
                };
                let index = parts[1]
                    .parse::<u32>()
                    .map_err(|e| WError::new("evaluate_tx", &format!("Invalid index: {}", e)))?;

                // Create an Action
                Ok(Action {
                    index,
                    budget: Budget {
                        mem: budget.memory,
                        steps: budget.steps,
                    },
                    tag,
                })
            })
            .collect();
        actions
    }
}

#[test]
fn test_additional_utxo_to_ogmios() {
    let utxo = AdditionalUtxo {
        tx_hash: "hash".to_string(),
        index: 0,
        address: "addr1".to_string(),
        value: vec![
            Asset::new_from_str("lovelace", "1000000"),
            Asset::new_from_str("asset1", "5"),
        ],
    };
    let ogmios_value = utxo.to_ogmios();
    let expected_json = r#"[
        {
            "transaction": {"id": "hash"},
            "output": {"index": 0}
        },
        {
            "address": "addr1",
            "value": {
                "lovelace": 1000000,
                "asset1": 5
            }
        }
    ]"#;

    let expected_value: serde_json::Value = serde_json::from_str(expected_json).unwrap();
    let actual_value: serde_json::Value = ogmios_value;

    assert_eq!(actual_value, expected_value);
}
