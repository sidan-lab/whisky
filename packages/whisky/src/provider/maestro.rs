mod utils;

use async_trait::async_trait;
use maestro_rust_sdk::client::block_info::BlockInfo as MBlockInfo;
use maestro_rust_sdk::models::accounts::StakeAccountInformation;
use maestro_rust_sdk::models::addresses::{Utxo, UtxosAtAddress};
use maestro_rust_sdk::models::asset::{AddressesHoldingAsset, AssetInformations};
use maestro_rust_sdk::models::general::ProtocolParameters;
use maestro_rust_sdk::models::transactions::{RedeemerEvaluation, TransactionDetails};
use sidan_csl_rs::core::serializer::apply_double_cbor_encoding;
use sidan_csl_rs::csl::{
    self, Address, BaseAddress, JsError, NativeScript, PlutusScript, RewardAddress, ScriptRef,
};
use sidan_csl_rs::model::{
    AccountInfo, Asset, BlockInfo, Network, Protocol, TransactionInfo, UTxO, UtxoInput, UtxoOutput,
};
use sidan_csl_rs::{
    core::{serializer::calculate_tx_hash, tx_parser::TxParser},
    model::{Action, Budget, RedeemerTag},
};
use std::collections::HashMap;
use std::error::Error;
use uplc::tx::SlotConfig;
use utils::asset_utils::CollectionAssets;

use crate::service::{Evaluator, Fetcher, FetcherOptions};

use reqwest::RequestBuilder;
use serde::Serialize;

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

#[derive(Debug, Clone)]
pub enum Script {
    Plutus(PlutusScript),
    Native(NativeScript),
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

    pub fn to_utxo(&self, utxo: &Utxo) -> UTxO {
        UTxO {
            input: UtxoInput {
                output_index: utxo.index as u32,
                tx_hash: utxo.tx_hash.clone(),
            },
            output: UtxoOutput {
                address: utxo.address.clone(),
                amount: utxo
                    .assets
                    .iter()
                    .map(|asset| Asset::new(asset.unit.clone(), asset.amount.to_string()))
                    .collect(),
                data_hash: utxo.datum.as_ref().and_then(|datum| {
                    datum
                        .get("hash")
                        .and_then(|hash| hash.as_str().map(|s| s.to_string()))
                }),
                plutus_data: utxo.datum.as_ref().and_then(|datum| {
                    datum
                        .get("bytes")
                        .and_then(|hash| hash.as_str().map(|s| s.to_string()))
                }),
                script_ref: Some(self.resolve_script(utxo).unwrap()),
                script_hash: utxo
                    .reference_script
                    .as_ref()
                    .map(|script| script.hash.clone()),
            },
        }
    }

    pub fn resolve_script(&self, utxo: &Utxo) -> Result<String, JsError> {
        if let Some(ref_script) = &utxo.reference_script {
            match ref_script.r#type.as_str() {
                "native" => {
                    let script: NativeScript =
                        NativeScript::from_json(&serde_json::json!(&ref_script.json).to_string())?;
                    let script_ref = self.to_script_ref(&Script::Native(script));
                    Ok(script_ref.native_script().unwrap().to_hex())
                }
                "plutusv1" => {
                    let script_hex = &ref_script.bytes;
                    let normalized = self.normalize_plutus_script(script_hex)?;
                    let script: PlutusScript = PlutusScript::from_hex_with_version(
                        &normalized,
                        &csl::Language::new_plutus_v1(),
                    )?;
                    let script_ref = self.to_script_ref(&Script::Plutus(script));
                    Ok(script_ref.plutus_script().unwrap().to_hex())
                }
                "plutusv2" => {
                    let script_hex = &ref_script.bytes;
                    let normalized = self.normalize_plutus_script(script_hex)?;
                    let script: PlutusScript = PlutusScript::from_hex_with_version(
                        &normalized,
                        &csl::Language::new_plutus_v2(),
                    )?;
                    let script_ref = self.to_script_ref(&Script::Plutus(script));
                    Ok(script_ref.plutus_script().unwrap().to_hex())
                }
                _ => Err(JsError::from_str("Unsupported script type")),
            }
        } else {
            Err(JsError::from_str("TODO"))
        }
    }

    pub fn normalize_plutus_script(&self, script_hex: &str) -> Result<String, JsError> {
        apply_double_cbor_encoding(script_hex)
    }

    pub fn to_script_ref(&self, script: &Script) -> ScriptRef {
        match script {
            Script::Plutus(plutus) => ScriptRef::new_plutus_script(plutus),
            Script::Native(native) => ScriptRef::new_native_script(native),
        }
    }

    pub fn resolve_reward_address(&self, bech32: &str) -> Result<String, JsError> {
        let address = Address::from_bech32(bech32)?;

        if let Some(base_address) = BaseAddress::from_address(&address) {
            let stake_credential = BaseAddress::stake_cred(&base_address);

            let reward_address = RewardAddress::new(address.network_id()?, &stake_credential)
                .to_address()
                .to_bech32(None);
            Ok(reward_address?)
        } else {
            Err(JsError::from_str(
                "An error occurred during resolveRewardAddress",
            ))
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

        let result = self.maestro_client.evaluate_tx(tx, tx_out_cbors).await;
        match result {
            Ok(redeemer_evaluation) => {
                println!("success maestro call");
                Ok(redeemer_evaluation
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
                    .collect())
            }
            Err(e) => {
                println!("fail maestro call");
                Err(JsError::from_str(&format!("{}", e)))
            }
        }
    }
}

#[async_trait]
impl Fetcher for MaestroProvider {
    async fn fetch_account_info(&self, address: &str) -> Result<AccountInfo, Box<dyn Error>> {
        let reward_address = if address.starts_with("addr") {
            self.maestro_client.resolve_reward_address(address)?
        } else {
            address.to_string()
        };

        let url = format!("accounts/{}", reward_address);

        let resp = self.maestro_client.get(&url).await?;

        let stake_account_information: StakeAccountInformation =
            serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;

        let account_info: AccountInfo = utils::account_utils::account_information_to_account_info(
            stake_account_information.data,
        );
        Ok(account_info)
    }

    async fn fetch_address_utxos(
        &self,
        address: &str,
        asset: Option<&str>,
    ) -> Result<Vec<UTxO>, Box<dyn Error>> {
        let query_predicate =
            if address.starts_with("addr_vkh") || address.starts_with("addr_shared_vkh") {
                format!("addresses/cred/{}", address)
            } else {
                format!("addresses/{}", address)
            };

        let append_asset_string = match asset {
            Some(a) => format!("&asset={}", a),
            None => "".to_string(),
        };

        let url = format!("{}utxos?count=100{}", query_predicate, append_asset_string,);

        let resp = self.maestro_client.get(&url).await?;

        let mut utxos_at_address: UtxosAtAddress =
            serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;

        let mut added_utxos: Vec<UTxO> = utxos_at_address
            .data
            .iter()
            .map(|utxo| self.maestro_client.to_utxo(&utxo))
            .collect();

        while utxos_at_address.next_cursor.is_some() {
            let append_cursor_string = format!("&cursor={}", utxos_at_address.next_cursor.unwrap());
            let url = format!(
                "{}utxos?count=100{}{}",
                query_predicate, append_asset_string, append_cursor_string
            );
            let resp = self.maestro_client.get(&url).await?;
            utxos_at_address =
                serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;
            let uxtos: Vec<UTxO> = utxos_at_address
                .data
                .iter()
                .map(|utxo| self.maestro_client.to_utxo(&utxo))
                .collect();
            added_utxos.extend(uxtos);
        }

        Ok(added_utxos)
    }

    async fn fetch_address_txs(
        &self,
        _address: &str,
        _options: Option<FetcherOptions>,
    ) -> Result<Vec<TransactionInfo>, Box<dyn Error>> {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "Method not implemented",
        )));
        // TODO: open for contribution, see blockfrost.ts for reference
    }
    async fn fetch_asset_addresses(
        &self,
        asset: &str,
    ) -> Result<Vec<(String, String)>, Box<dyn Error>> {
        let (policy_id, asset_name) = Asset::unit_to_tuple(asset);
        let url = format!("/assets/{}{}/addresses?count=100", &policy_id, &asset_name);

        let resp = self.maestro_client.get(&url).await?;

        let mut addresses_holding_asset: AddressesHoldingAsset =
            serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;

        let mut added_assets: Vec<(String, String)> = addresses_holding_asset
            .data
            .iter()
            .map(|address_holding_asset| {
                (
                    address_holding_asset.address.clone(),
                    address_holding_asset.amount.to_string(),
                )
            })
            .collect();

        while addresses_holding_asset.next_cursor.is_some() {
            let append_cursor_string =
                format!("&cursor={}", addresses_holding_asset.next_cursor.unwrap());
            let url = format!(
                "/assets/{}{}/addresses?count=100{}",
                &policy_id, &asset_name, append_cursor_string
            );

            let resp = self.maestro_client.get(&url).await?;
            addresses_holding_asset =
                serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;
            let assets: Vec<(String, String)> = addresses_holding_asset
                .data
                .iter()
                .map(|address_holding_asset| {
                    (
                        address_holding_asset.address.clone(),
                        address_holding_asset.amount.to_string(),
                    )
                })
                .collect();
            added_assets.extend(assets);
        }

        Ok(added_assets)
    }
    async fn fetch_asset_metadata(
        &self,
        asset: &str,
    ) -> Result<HashMap<String, serde_json::Value>, Box<dyn Error>> {
        let (policy_id, asset_name) = Asset::unit_to_tuple(asset);
        let url = format!("/assets/{}{}", &policy_id, &asset_name);
        let resp = self.maestro_client.get(&url).await?;
        let asset_informations: AssetInformations =
            serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;

        let asset_metadata = asset_informations.data.latest_mint_tx_metadata;
        Ok(asset_metadata)
    }
    async fn fetch_block_info(&self, hash: &str) -> Result<BlockInfo, Box<dyn Error>> {
        let url = format!("/blocks/{}", hash);

        let resp = self.maestro_client.get(&url).await?;

        let m_block_info: MBlockInfo =
            serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;

        let block_info: BlockInfo =
            utils::block_utils::block_info_data_to_block_info(m_block_info.data);

        Ok(block_info)
    }
    async fn fetch_collection_assets(
        &self,
        policy_id: &str,
        cursor: Option<String>,
    ) -> Result<(Vec<(String, String)>, Option<String>), Box<dyn Error>> {
        let append_cursor_string = match cursor {
            Some(c) => format!("&cursor={}", c),
            None => "".to_string(),
        };
        let url = format!(
            "/policy/{}/assets?count=100${}",
            policy_id, append_cursor_string
        );

        let resp = self.maestro_client.get(&url).await?;

        let collection_assets: CollectionAssets =
            serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;

        let assets = collection_assets
            .data
            .iter()
            .map(|asset_data| {
                (
                    format!("{}{}", policy_id, asset_data.asset_name.clone()),
                    asset_data.total_supply.clone(),
                )
            })
            .collect();

        Ok((assets, collection_assets.next_cursor))
    }
    async fn fetch_protocol_parameters(
        &self,
        epoch: Option<u32>,
    ) -> Result<Protocol, Box<dyn Error>> {
        if let Some(_epoch) = epoch {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Maestro only supports fetching Protocol parameters of the latest completed epoch.",
            )));
        }

        let url = "/protocol-params";

        let resp = self.maestro_client.get(url).await?;

        let protocol_parameters: ProtocolParameters =
            serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;

        let protocol: Protocol =
            utils::protocol_utils::protocol_paras_data_to_protocol(protocol_parameters.data);
        Ok(protocol)
    }
    async fn fetch_tx_info(&self, hash: &str) -> Result<TransactionInfo, Box<dyn Error>> {
        let url = format!("/transactions/{}", hash);

        let resp = self.maestro_client.get(&url).await?;

        let transaction_details: TransactionDetails =
            serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;

        let transaction_info: TransactionInfo =
            utils::transaction_utils::transaction_detail_to_info(transaction_details.data);
        Ok(transaction_info)
    }
    async fn fetch_utxos(
        &self,
        hash: &str,
        index: Option<u32>,
    ) -> Result<Vec<UTxO>, Box<dyn Error>> {
        let url = format!("/transactions/{}", hash);

        let resp = self.maestro_client.get(&url).await?;

        let transaction_details: TransactionDetails =
            serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;

        let outputs: Vec<UTxO> = transaction_details
            .data
            .outputs
            .iter()
            .map(|utxo| self.maestro_client.to_utxo(&utxo))
            .collect();

        let utxos = match index {
            Some(i) => outputs
                .iter()
                .filter(|output| output.input.output_index == i)
                .cloned()
                .collect(),
            None => outputs,
        };

        Ok(utxos)
    }
    async fn get(&self, url: &str) -> Result<serde_json::Value, Box<dyn Error>> {
        let resp = self.maestro_client.get(url).await?;
        let any = serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(any)
    }
}
#[cfg(test)]
mod tests {
    // use super::*;

    #[tokio::test]
    async fn test_maestro_provider() {
        use dotenv::dotenv;
        // use std::env::var;
        dotenv().ok();
        println!("TODO: update with hardfork tx")
        // let provider = MaestroProvider::new(var("MAESTRO_API_KEY").unwrap().as_str(), "preprod");
        // let tx = "84a800848258202c255d39a6d448b408bdb1734c99dfc8c487ac23fd7ee5e8b431a99bc514980a0882582040e1afc8b735a9daf665926554b0e11902e3ed7e4a31a23b917483d4de42c05e04825820ffb4e04fd430ffd1bdf014990c6d63a5303c1745ff228b70823fc757a04b1c6402825820ffb4e04fd430ffd1bdf014990c6d63a5303c1745ff228b70823fc757a04b1c64030182a3005839104477981671d60af19c524824cacc0a9822ba2a7f32586e57c18156215ca749261aa3b17aa2cd4b026bc6566c4b14421d6083edce64ffe5cb01821a0016e360a1581c5066154a102ee037390c5236f78db23239b49c5748d3d349f3ccf04ba144555344581a0243d580028201d81843d87980a300583910634a34d9c1ec5dd0cae61e4c86a4e85214bafdc80c57214fc80745b55ca749261aa3b17aa2cd4b026bc6566c4b14421d6083edce64ffe5cb01821a007520dba1581c5066154a102ee037390c5236f78db23239b49c5748d3d349f3ccf04ba144555344581a1298be00028201d81858b1d8799fd8799fd87a9f581c57f7ddf8c822daad03fd80823153a61d913e5c9147bd478e3ccd70b3ffd8799fd8799fd8799f581c5ca749261aa3b17aa2cd4b026bc6566c4b14421d6083edce64ffe5cbffffffffd8799fd87a9f581c4477981671d60af19c524824cacc0a9822ba2a7f32586e57c1815621ffd8799fd8799fd8799f581c5ca749261aa3b17aa2cd4b026bc6566c4b14421d6083edce64ffe5cbffffffffd87a801a000985801a1dcd6500ff021a0004f12509a00b5820d14a26f50fba04067fae8c9bbd4c0dbaa77e582100fb89b6a140630945ab99d50d818258203fbdf2b0b4213855dd9b87f7c94a50cf352ba6edfdded85ecb22cf9ceb75f814070e82581cd161d64eef0eeb59f9124f520f8c8f3b717ed04198d54c8b17e604ae581c5ca51b304b1f79d92eada8c58c513e969458dcd27ce4f5bc47823ffa1286825820efe6fbbdd6b993d96883b96c572bfcaa0a4a138c83bd948dec1751d1bfda09b300825820ac7744adce4f25027f1ca009f5cab1d0858753e62c6081a3a3676cfd5333bb03008258202c255d39a6d448b408bdb1734c99dfc8c487ac23fd7ee5e8b431a99bc514980a08825820ffb4e04fd430ffd1bdf014990c6d63a5303c1745ff228b70823fc757a04b1c640282582040e1afc8b735a9daf665926554b0e11902e3ed7e4a31a23b917483d4de42c05e04825820ffb4e04fd430ffd1bdf014990c6d63a5303c1745ff228b70823fc757a04b1c6403a203800584840000d87980821a000557301a07270e00840001d87980821a000557301a07270e00840002d87980821a000557301a07270e00840003d87980821a000557301a07270e00f5f6";
        // let chained_tx: Vec<String>= vec!["84a800848258202c255d39a6d448b408bdb1734c99dfc8c487ac23fd7ee5e8b431a99bc514980a0882582040e1afc8b735a9daf665926554b0e11902e3ed7e4a31a23b917483d4de42c05e04825820ffb4e04fd430ffd1bdf014990c6d63a5303c1745ff228b70823fc757a04b1c6402825820ffb4e04fd430ffd1bdf014990c6d63a5303c1745ff228b70823fc757a04b1c64030182a3005839104477981671d60af19c524824cacc0a9822ba2a7f32586e57c18156215ca749261aa3b17aa2cd4b026bc6566c4b14421d6083edce64ffe5cb01821a0016e360a1581c5066154a102ee037390c5236f78db23239b49c5748d3d349f3ccf04ba144555344581a0243d580028201d81843d87980a300583910634a34d9c1ec5dd0cae61e4c86a4e85214bafdc80c57214fc80745b55ca749261aa3b17aa2cd4b026bc6566c4b14421d6083edce64ffe5cb01821a0075b8d4a1581c5066154a102ee037390c5236f78db23239b49c5748d3d349f3ccf04ba144555344581a1298be00028201d81858b1d8799fd8799fd87a9f581c57f7ddf8c822daad03fd80823153a61d913e5c9147bd478e3ccd70b3ffd8799fd8799fd8799f581c5ca749261aa3b17aa2cd4b026bc6566c4b14421d6083edce64ffe5cbffffffffd8799fd87a9f581c4477981671d60af19c524824cacc0a9822ba2a7f32586e57c1815621ffd8799fd8799fd8799f581c5ca749261aa3b17aa2cd4b026bc6566c4b14421d6083edce64ffe5cbffffffffd87a801a000985801a1dcd6500ff021a0004592c09a00b5820a68f004e69dfc4ed4ff789ceb9be63e9f2412e8d3d7fa0b0cb19e509c927a03c0d818258203fbdf2b0b4213855dd9b87f7c94a50cf352ba6edfdded85ecb22cf9ceb75f814070e82581cd161d64eef0eeb59f9124f520f8c8f3b717ed04198d54c8b17e604ae581c5ca51b304b1f79d92eada8c58c513e969458dcd27ce4f5bc47823ffa1286825820ac7744adce4f25027f1ca009f5cab1d0858753e62c6081a3a3676cfd5333bb03008258202c255d39a6d448b408bdb1734c99dfc8c487ac23fd7ee5e8b431a99bc514980a0882582040e1afc8b735a9daf665926554b0e11902e3ed7e4a31a23b917483d4de42c05e04825820ffb4e04fd430ffd1bdf014990c6d63a5303c1745ff228b70823fc757a04b1c6402825820ffb4e04fd430ffd1bdf014990c6d63a5303c1745ff228b70823fc757a04b1c6403825820efe6fbbdd6b993d96883b96c572bfcaa0a4a138c83bd948dec1751d1bfda09b300a30082825820aa8ce9e908f525c3b700a65669430ec68ca19615e7309e25bb6fa883964cfa9f5840a023ea4e2a266fca669cfdffe3718718c2b2c6e3fbc90da58785079583d94be98f20d2b87327edb940984a739c1fdb25e20e6b04374db299b4de66369208de038258207f4747ca0c20a1e5c28716c4a10fffbcbe8fe6253cb427ae2f0e24d231a9808458402aa02a8a0f2129d727e44cd21f4699b1b1deb43c974ebc6f484b3809e0b5a417e864c43c9be5327fba31fa8146c744c487b00748cb63daf3dc60114850321d0d03800584840000d87980821a000382f61a04d45a03840001d87980821a000382f61a04d45a03840002d87980821a000382f61a04d45a03840003d87980821a000382f61a04d45a03f5f6".to_string(), "84a800848258202c255d39a6d448b408bdb1734c99dfc8c487ac23fd7ee5e8b431a99bc514980a098258205150964d0bc32df047f1eb99c35f14e351f21b1303795ffe2b58ebf7de58f67b0082582085aa98980be06b0f5d926bee007301ba7a96d448dfa9dced091fb73b0bcd07bb03825820879f68fef00fa676abcfba0396916299eddbf29e1103442aee031b383ee0f3ad060182a3005839104477981671d60af19c524824cacc0a9822ba2a7f32586e57c18156215ca749261aa3b17aa2cd4b026bc6566c4b14421d6083edce64ffe5cb01821a0016e360a1581c5066154a102ee037390c5236f78db23239b49c5748d3d349f3ccf04ba144555344581a0243d580028201d81843d87980a300583910634a34d9c1ec5dd0cae61e4c86a4e85214bafdc80c57214fc80745b55ca749261aa3b17aa2cd4b026bc6566c4b14421d6083edce64ffe5cb01821a00756f63a1581c5066154a102ee037390c5236f78db23239b49c5748d3d349f3ccf04ba144555344581a1298be00028201d81858b1d8799fd8799fd87a9f581c57f7ddf8c822daad03fd80823153a61d913e5c9147bd478e3ccd70b3ffd8799fd8799fd8799f581c5ca749261aa3b17aa2cd4b026bc6566c4b14421d6083edce64ffe5cbffffffffd8799fd87a9f581c4477981671d60af19c524824cacc0a9822ba2a7f32586e57c1815621ffd8799fd8799fd8799f581c5ca749261aa3b17aa2cd4b026bc6566c4b14421d6083edce64ffe5cbffffffffd87a801a000985801a1dcd6500ff021a0004a29d09a00b58205eb15f7d48931475604b5491a294f5d914ecf03c41a520d80087e2938910d9e70d818258203fbdf2b0b4213855dd9b87f7c94a50cf352ba6edfdded85ecb22cf9ceb75f814070e82581cd161d64eef0eeb59f9124f520f8c8f3b717ed04198d54c8b17e604ae581c5ca51b304b1f79d92eada8c58c513e969458dcd27ce4f5bc47823ffa128682582085aa98980be06b0f5d926bee007301ba7a96d448dfa9dced091fb73b0bcd07bb038258202c255d39a6d448b408bdb1734c99dfc8c487ac23fd7ee5e8b431a99bc514980a09825820efe6fbbdd6b993d96883b96c572bfcaa0a4a138c83bd948dec1751d1bfda09b3008258205150964d0bc32df047f1eb99c35f14e351f21b1303795ffe2b58ebf7de58f67b00825820879f68fef00fa676abcfba0396916299eddbf29e1103442aee031b383ee0f3ad06825820ac7744adce4f25027f1ca009f5cab1d0858753e62c6081a3a3676cfd5333bb0300a30082825820aa8ce9e908f525c3b700a65669430ec68ca19615e7309e25bb6fa883964cfa9f5840904f798c3cbda08e499945f9e00e6b1a968166de063ad3ecb16139a0c5dc10541cc7a33304c60ed7fb350938d2b11fcacb56baf84330473b8544b669640229028258207f4747ca0c20a1e5c28716c4a10fffbcbe8fe6253cb427ae2f0e24d231a98084584016b15d782922177e29e1eae8f7f173db80508692292b6ff3e63c7d33ed1cc231bac0acbb963503e75b96b7c541189508e050fb64034ea4d47a13115f7483ce0d03800584840000d87980821a00045e1e1a0609fd16840001d87980821a00045e1e1a0609fd16840002d87980821a00045e1e1a0609fd16840003d87980821a00045e1e1a0609fd16f5f6".to_string()];
        // let result = provider
        //     .evaluate_tx(tx, &[], &chained_tx, &Network::Mainnet)
        //     .await;
        // match result {
        //     Ok(actions) => {
        //         println!("actions: {:?}", actions);
        //         assert_eq!(actions.len(), 4);
        //     }
        //     _ => panic!("Error evaluating tx"),
        // }
    }
}
