use crate::*;
use async_trait::async_trait;
use maestro_rust_sdk::models::transactions::RedeemerEvaluation;
use serde_json::to_string;
use sidan_csl_rs::core::utils::apply_double_cbor_encoding;
use sidan_csl_rs::csl::{
    self, Address, BaseAddress, JsError, NativeScript, PlutusScript, RewardAddress, ScriptRef,
};
use sidan_csl_rs::model::{
    AccountInfo, Asset, AssetMetadata, BlockInfo, GovernanceProposalInfo, Network, Protocol,
    TransactionInfo, UTxO, UtxoInput, UtxoOutput,
};
use sidan_csl_rs::{
    core::{serializer::calculate_tx_hash, tx_parser::TxParser},
    model::{Action, Budget, RedeemerTag},
};
use std::error::Error;
use uplc::tx::SlotConfig;

use crate::service::{Evaluator, Fetcher, FetcherOptions};

use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct EvaluateTx {
    cbor: String,
    additional_utxos: Vec<AdditionalUtxo>,
}

#[derive(Serialize)]
pub struct AdditionalUtxo {
    pub tx_hash: String,
    pub index: u32,
    pub txout_cbor: String,
}

#[derive(Debug, Clone)]
pub struct Maestro {
    api_key: String,
    http_client: reqwest::Client,
    pub base_url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum MaestroDatumOptionType {
    Hash,
    Inline,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MaestroDatumOption {
    datum_type: MaestroDatumOptionType,
    hash: String,
    bytes: Option<String>,
    json: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum MaestroScriptType {
    Native,
    PlutusvOne,
    PlutusvTwo,
}

#[derive(Debug, Clone)]
pub enum Script {
    Plutus(PlutusScript),
    Native(NativeScript),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MaestroScript {
    hash: String,
    script_type: MaestroScriptType,
    bytes: Option<String>,
    json: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MaestroAsset {
    unit: String,
    amount: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MaestroUTxO {
    tx_hash: String,
    index: u32,
    assets: Vec<MaestroAsset>,
    address: String,
    datum: Option<MaestroDatumOption>,
    reference_script: Option<MaestroScript>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MaestroAssetExtended {
    asset_name: String,
    asset_name_ascii: String,
    fingerprint: String,
    total_supply: String,
    asset_standards: Cip25Metadata,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Cip25Metadata {
    data: Vec<String>,
    idx: u32,
    name: String,
    uid: String,
}

impl Maestro {
    pub fn new(api_key: String, network: String) -> Self {
        let base_url = format!("https://{}.gomaestro-api.org/v1", &network,);
        let http_client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(300))
            .build()
            .expect("Failed to create HTTP client");

        Maestro {
            api_key,
            http_client,
            base_url,
        }
    }

    async fn send_request(
        &self,
        req: RequestBuilder,
        response_body: &mut String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let req = req
            .header("Accept", "application/json")
            .header("api-key", &self.api_key)
            .build()?;

        println!("req: {:?}", req);

        let response = self.http_client.execute(req).await?;

        println!("response: {:?}", response);

        if response.status().is_success() {
            *response_body = response.text().await?;
            Ok(())
        } else {
            Err(format!("Error: {}", response.status()).into())
        }
    }

    pub async fn get(&self, url: &str) -> Result<String, Box<dyn std::error::Error>> {
        let req = self.http_client.get(format!("{}{}", &self.base_url, url));
        let mut response_body = String::new();
        self.send_request(req, &mut response_body).await?;
        Ok(response_body)
    }

    pub async fn post<T: Serialize>(
        &self,
        url: &str,
        body: T,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let json_body = serde_json::to_string(&body)?;

        let req = self
            .http_client
            .post(format!("{}{}", &self.base_url, url))
            .header("Content-Type", "application/json")
            .body(json_body);

        let mut response_body = String::new();
        self.send_request(req, &mut response_body).await?;
        Ok(response_body)
    }

    pub async fn evaluate_tx(
        &self,
        tx_cbor: &str,
        additional_utxos: Vec<AdditionalUtxo>,
    ) -> Result<Vec<RedeemerEvaluation>, Box<dyn Error>> {
        let url = "/transactions/evaluate";
        let body = EvaluateTx {
            cbor: tx_cbor.to_string(),
            additional_utxos,
        };
        let resp = self.post(url, &body).await?;
        let redeemer_evaluations =
            serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(redeemer_evaluations)
    }

    fn to_utxo(&self, utxo: &MaestroUTxO) -> UTxO {
        UTxO {
            input: UtxoInput {
                output_index: utxo.index,
                tx_hash: utxo.tx_hash.clone(),
            },
            output: UtxoOutput {
                address: utxo.address.clone(),
                amount: utxo
                    .assets
                    .iter()
                    .map(|asset| Asset::new(asset.unit.clone(), asset.amount.clone()))
                    .collect(),
                data_hash: Some(utxo.datum.clone().unwrap().hash),
                plutus_data: Some(utxo.datum.clone().unwrap().bytes.unwrap()),
                script_ref: Some(self.resolve_script(utxo).unwrap()),
                script_hash: utxo
                    .reference_script
                    .as_ref()
                    .map(|ref_script| ref_script.hash.clone()),
            },
        }
    }

    fn resolve_script(&self, utxo: &MaestroUTxO) -> Result<String, JsError> {
        if let Some(ref_script) = &utxo.reference_script {
<<<<<<< HEAD
            match ref_script.r#type {
                // MaestroScriptType::Native => {
                //     let script: NativeScript = NativeScript::from_bytes(ref_script.json.clone().);
                    Ok(self.to_script_ref(&Script::Native(script))
=======
            match ref_script.script_type {
                MaestroScriptType::Native => {
                    let script: NativeScript =
                        NativeScript::from_json(&ref_script.json.clone().unwrap())?;
                    Ok(self
                        .to_script_ref(&Script::Native(script))
>>>>>>> a84ad3a (added: to utxo)
                        .native_script()
                        .unwrap()
                        .to_hex())
                }
                MaestroScriptType::PlutusvOne => {
                    if let Some(script_hex) = &ref_script.bytes {
                        let normalized = self.normalize_plutus_script(&script_hex)?;
                        let script: PlutusScript = PlutusScript::from_hex_with_version(
                            &normalized,
                            &csl::Language::new_plutus_v1(),
                        )?;
                        Ok(self
                            .to_script_ref(&Script::Plutus(script))
                            .plutus_script()
                            .unwrap()
                            .to_hex())
                    } else {
                        Err(JsError::from_str(
                            "Expected plutusV1 script but received None",
                        ))
                    }
                }
                MaestroScriptType::PlutusvTwo => {
                    if let Some(script_hex) = &ref_script.bytes {
                        let normalized = self.normalize_plutus_script(&script_hex)?;
                        let script: PlutusScript = PlutusScript::from_hex_with_version(
                            &normalized,
                            &csl::Language::new_plutus_v2(),
                        )?;
                        Ok(self
                            .to_script_ref(&Script::Plutus(script))
                            .plutus_script()
                            .unwrap()
                            .to_hex())
                    } else {
                        Err(JsError::from_str(
                            "Expected plutusV2 script but received None",
                        ))
                    }
                }
            }
        } else {
            Err(JsError::from_str("TODO"))
        }
    }

    fn normalize_plutus_script(&self, script_hex: &str) -> Result<String, JsError> {
        apply_double_cbor_encoding(script_hex)
    }

    fn to_script_ref(&self, script: &Script) -> ScriptRef {
        match script {
            Script::Plutus(plutus) => ScriptRef::new_plutus_script(plutus),
            Script::Native(native) => ScriptRef::new_native_script(native),
        }
    }

    fn resolve_reward_address(bech32: &str) -> Result<String, JsError> {
        let address = Address::from_bech32(bech32)?;

        if let Some(base_address) = BaseAddress::from_address(&address) {
            let stake_credential = BaseAddress::stake_cred(&base_address);

            let reward_address = RewardAddress::new(address.network_id()?, &stake_credential)
                .to_address()
                .to_bech32(None);
            Ok(reward_address?)
        } else {
            Err(JsError::from_str("TODO"))
        }
    }
}

#[derive(Clone, Debug)]
pub struct MaestroProvider {
    pub maestro_client: Maestro,
}

impl MaestroProvider {
    pub fn new(api_key: &str, network: &str) -> MaestroProvider {
        let maestro_client = Maestro::new(api_key.to_string(), network.to_string());
        MaestroProvider { maestro_client }
    }
}

#[async_trait]
impl Evaluator for MaestroProvider {
    async fn evaluate_tx(
        &self,
        tx: &str,
        _inputs: &[UTxO], // TODO: parse this also into additional_txs
        additional_txs: &[String],
        _network: &Network,
        _slot_config: &SlotConfig,
    ) -> Result<Vec<Action>, WError> {
        let tx_out_cbors: Vec<AdditionalUtxo> = additional_txs
            .iter()
            .flat_map(|tx| {
                let parsed_tx = TxParser::new(tx);
                parsed_tx
                    .unwrap() //TODO: add error handling
                    .get_tx_outs_cbor()
                    .iter()
                    .enumerate() // Add this line to get the index
                    .map(|(index, txout_cbor)| AdditionalUtxo {
                        tx_hash: calculate_tx_hash(tx).unwrap(), // TODO: add error handling
                        index: index as u32,                     // Use the index here
                        txout_cbor: txout_cbor.to_string(),
                    })
                    .collect::<Vec<AdditionalUtxo>>()
            })
            .collect();

        let ex_units: Vec<Action> = self
            .maestro_client
            .evaluate_tx(tx, tx_out_cbors)
            .await
            .map_err(WError::from_err("MaestroProvider - evaluate_tx"))?
            .iter()
            .map(|r: &RedeemerEvaluation| Action {
                index: r.redeemer_index as u32,
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
            .collect();
        Ok(ex_units)
    }
}
