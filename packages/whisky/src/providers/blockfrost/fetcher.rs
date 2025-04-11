use super::models::account::BlockfrostAccountInfo;
use super::models::asset::{AssetAddresses, AssetPolicy, BlockfrostAsset};
use super::models::block::BlockContent;
use super::models::epoch::EpochParam;
use super::models::transaction::{BlockfrostTxInfo, BlockfrostTxUtxo};
use super::models::utxo::BlockfrostUtxo;
use super::utils::*;
use super::BlockfrostProvider;
use async_trait::async_trait;

use futures::future;
use std::collections::HashMap;
use whisky_common::models::{AccountInfo, BlockInfo, Protocol, TransactionInfo, UTxO};

use whisky_common::*;

#[async_trait]
impl Fetcher for BlockfrostProvider {
    async fn fetch_account_info(&self, address: &str) -> Result<AccountInfo, WError> {
        let reward_address = if address.starts_with("addr") {
            resolve_reward_address(address).map_err(WError::from_err("resolve_reward_address"))?
        } else {
            address.to_string()
        };

        let url = format!("/accounts/{}", reward_address);

        let resp = self
            .blockfrost_client
            .get(&url)
            .await
            .map_err(WError::from_err("blockfrost::fetch_account_info get"))?;

        let blockfrost_account_info: BlockfrostAccountInfo = serde_json::from_str(&resp).map_err(
            WError::from_err("blockfrost::fetch_account_info type error"),
        )?;

        let account_info: AccountInfo =
            blockfrost_account_info_to_account_info(blockfrost_account_info);
        Ok(account_info)
    }

    async fn fetch_address_utxos(
        &self,
        address: &str,
        asset: Option<&str>,
    ) -> Result<Vec<UTxO>, WError> {
        let mut page = 1;
        let mut added_utxos: Vec<UTxO> = Vec::new();

        loop {
            let append_asset_string = asset.map_or_else(String::new, |a| format!("{}", a));
            let append_page_string = format!("?page={}", page);

            let url = format!(
                "/addresses/{}/utxos/{}{}",
                address, append_asset_string, append_page_string
            );

            let resp = self
                .blockfrost_client
                .get(&url)
                .await
                .map_err(WError::from_err("blockfrost::fetch_address_utxos get"))?;

            let blockfrost_utxos: Vec<BlockfrostUtxo> = serde_json::from_str(&resp).map_err(
                WError::from_err("blockfrost::fetch_address_utxos type error"),
            )?;

            let uxtos: Vec<UTxO> =
                future::join_all(blockfrost_utxos.iter().map(|utxo| self.to_utxo(utxo)))
                    .await
                    .into_iter()
                    .collect::<Result<Vec<_>, _>>()?;

            added_utxos.extend(uxtos);

            if blockfrost_utxos.len() < 100 {
                break;
            }

            page += 1;
        }

        Ok(added_utxos)
    }

    async fn fetch_asset_addresses(&self, asset: &str) -> Result<Vec<(String, String)>, WError> {
        let mut page = 1;
        let mut added_assets: Vec<(String, String)> = Vec::new();

        loop {
            let (policy_id, asset_name) = Asset::unit_to_tuple(asset);
            let append_page_string = format!("?page={}", page);

            let url = format!(
                "/assets/{}{}/addresses{}",
                policy_id, asset_name, append_page_string
            );

            let resp = self
                .blockfrost_client
                .get(&url)
                .await
                .map_err(WError::from_err("blockfrost::fetch_asset_addresses get"))?;

            let blockfrost_assets: Vec<AssetAddresses> = serde_json::from_str(&resp).map_err(
                WError::from_err("blockfrost::fetch_asset_addresses type error"),
            )?;

            let assets: Vec<(String, String)> = blockfrost_assets
                .iter()
                .map(|asset| (asset.address.clone(), asset.quantity.to_string()))
                .collect();

            added_assets.extend(assets);

            if blockfrost_assets.len() < 100 {
                break;
            }

            page += 1;
        }
        Ok(added_assets)
    }

    async fn fetch_asset_metadata(
        &self,
        asset: &str,
    ) -> Result<Option<HashMap<String, serde_json::Value>>, WError> {
        let (policy_id, asset_name) = Asset::unit_to_tuple(asset);
        let url = format!("/assets/{}{}", &policy_id, &asset_name);
        let resp = self
            .blockfrost_client
            .get(&url)
            .await
            .map_err(WError::from_err("blockfrost::fetch_asset_metadata get"))?;

        let blockfrost_asset: BlockfrostAsset = serde_json::from_str(&resp).map_err(
            WError::from_err("blockfrost::fetch_asset_metadata type error"),
        )?;

        let asset_metadata: HashMap<String, serde_json::Value> =
            serde_json::to_value(&blockfrost_asset)
                .expect("Failed to convert object to JSON")
                .as_object()
                .map(|obj| {
                    obj.iter()
                        .map(|(k, v)| (k.to_string(), v.clone()))
                        .collect()
                })
                .unwrap_or_default();

        Ok(Some(asset_metadata))
    }

    async fn fetch_block_info(&self, hash: &str) -> Result<BlockInfo, WError> {
        let url = format!("/blocks/{}", hash);

        let resp = self
            .blockfrost_client
            .get(&url)
            .await
            .map_err(WError::from_err("blockfrost::fetch_block_info get"))?;
        let block_content: BlockContent = serde_json::from_str(&resp)
            .map_err(WError::from_err("blockfrost::fetch_block_info type error"))?;

        let block_info: BlockInfo = block_content_to_block_info(block_content)
            .map_err(WError::from_err("blockfrost::fetch_block_info"))?;

        Ok(block_info)
    }

    async fn fetch_collection_assets(
        &self,
        policy_id: &str,
        cursor: Option<String>,
    ) -> Result<(Vec<(String, String)>, Option<String>), WError> {
        let cursor = cursor.unwrap_or("1".to_string());

        let append_page_string = format!("?page={}", cursor);

        let url = format!("/assets/policy/{}{}", policy_id, append_page_string);

        let resp = self
            .blockfrost_client
            .get(&url)
            .await
            .map_err(WError::from_err("blockfrost::fetch_collection_assets get"))?;

        let asset_policies: Vec<AssetPolicy> = serde_json::from_str(&resp).map_err(
            WError::from_err("blockfrost::fetch_collection_assets type error"),
        )?;

        let assets: Vec<(String, String)> = asset_policies
            .iter()
            .map(|asset| (asset.asset.clone(), asset.quantity.clone()))
            .collect();

        let updated_cursor: Option<String> = if asset_policies.len() == 100 {
            Some((cursor.parse::<i32>().unwrap_or(1) + 1).to_string())
        } else {
            None
        };

        Ok((assets, updated_cursor))
    }

    async fn fetch_protocol_parameters(&self, epoch: Option<u32>) -> Result<Protocol, WError> {
        let append_epoch_string = match epoch {
            Some(c) => format!("{}", c),
            None => "latest".to_string(),
        };

        let url = format!("/epochs/{}/parameters", append_epoch_string);

        let resp = self
            .blockfrost_client
            .get(&url)
            .await
            .map_err(WError::from_err(
                "blockfrost::fetch_protocol_parameters get",
            ))?;

        let epoch_param: EpochParam = serde_json::from_str(&resp).map_err(WError::from_err(
            "blockfrost::fetch_protocol_parameters type error",
        ))?;

        let protocol: Protocol = epoch_param_to_protocol(epoch_param)
            .map_err(WError::from_err("blockfrost::fetch_protocol_parameters"))?;

        Ok(protocol)
    }

    async fn fetch_tx_info(&self, hash: &str) -> Result<TransactionInfo, WError> {
        let tx_url = format!("/txs/{}", hash);

        let tx_resp = self
            .blockfrost_client
            .get(&tx_url)
            .await
            .map_err(WError::from_err("blockfrost::fetch_tx_info get"))?;

        let blockfrost_tx_info: BlockfrostTxInfo = serde_json::from_str(&tx_resp)
            .map_err(WError::from_err("blockfrost::fetch_tx_info type error"))?;

        let utxo_url = format!("/txs/{}/utxos", hash);

        let utxo_resp = self
            .blockfrost_client
            .get(&utxo_url)
            .await
            .map_err(WError::from_err("blockfrost_::fetch_utxos get"))?;

        let blockfrost_tx_utxo: BlockfrostTxUtxo = serde_json::from_str(&utxo_resp)
            .map_err(WError::from_err("blockfrost_::fetch_utxos type error"))?;

        let blockfrost_inputs: Vec<BlockfrostUtxo> = blockfrost_tx_utxo
            .outputs
            .iter()
            .map(|utxo| {
                blockfrost_tx_output_utxo_to_blockfrost_utxo(utxo, &blockfrost_tx_utxo.hash)
            })
            .collect();

        let inputs: Vec<UTxO> =
            future::join_all(blockfrost_inputs.iter().map(|utxo| self.to_utxo(utxo)))
                .await
                .into_iter()
                .collect::<Result<Vec<_>, _>>()?;

        let blockfrost_outputs: Vec<BlockfrostUtxo> = blockfrost_tx_utxo
            .outputs
            .iter()
            .map(|utxo| {
                blockfrost_tx_output_utxo_to_blockfrost_utxo(utxo, &blockfrost_tx_utxo.hash)
            })
            .collect();

        let outputs: Vec<UTxO> =
            future::join_all(blockfrost_outputs.iter().map(|utxo| self.to_utxo(utxo)))
                .await
                .into_iter()
                .collect::<Result<Vec<_>, _>>()?;

        let transaction_info: TransactionInfo =
            blockfrost_txinfo_to_txinfo(blockfrost_tx_info, inputs, outputs);

        Ok(transaction_info)
    }

    async fn fetch_utxos(&self, hash: &str, index: Option<u32>) -> Result<Vec<UTxO>, WError> {
        let url = format!("/txs/{}/utxos", hash);

        let resp = self
            .blockfrost_client
            .get(&url)
            .await
            .map_err(WError::from_err("blockfrost_::fetch_utxos get"))?;

        let blockfrost_tx_utxo: BlockfrostTxUtxo = serde_json::from_str(&resp)
            .map_err(WError::from_err("blockfrost_::fetch_utxos type error"))?;

        let blockfrost_utxos: Vec<BlockfrostUtxo> = blockfrost_tx_utxo
            .outputs
            .iter()
            .map(|utxo| {
                blockfrost_tx_output_utxo_to_blockfrost_utxo(utxo, &blockfrost_tx_utxo.hash)
            })
            .collect();

        let outputs: Vec<UTxO> =
            future::join_all(blockfrost_utxos.iter().map(|utxo| self.to_utxo(utxo)))
                .await
                .into_iter()
                .collect::<Result<Vec<_>, _>>()?;

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

    async fn get(&self, url: &str) -> Result<serde_json::Value, WError> {
        let resp = self
            .blockfrost_client
            .get(&url)
            .await
            .map_err(WError::from_err("blockfrost::get"))?;
        let any =
            serde_json::from_str(&resp).map_err(WError::from_err("blockfrost::get error type"))?;
        Ok(any)
    }
}

#[cfg(test)]
mod fetcher {
    use super::{BlockfrostProvider, Fetcher};
    use dotenv::dotenv;
    use std::env::var;

    #[tokio::test]
    async fn test_fetch_account_info() {
        dotenv().ok();
        let provider = BlockfrostProvider::new(
            var("BLOCKFROST_PREPROD_PROJECT_ID").unwrap().as_str(),
            "preprod",
        );
        let address: &str = "addr_test1qzhm3fg7v9t9e4nrlw0z49cysmvzfy3xpmvxuht80aa3rvnm5tz7rfnph9ntszp2fclw5m334udzq49777gkhwkztsks4c69rg";
        let result = provider.fetch_account_info(address).await;
        match result {
            Ok(account_info) => {
                println!("account_info: {:?}", account_info);
                assert_eq!(account_info.active, false);
            }
            _ => panic!("Error fetching account info"),
        }
    }

    #[tokio::test]
    async fn test_fetch_address_utxos() {
        dotenv().ok();
        let provider = BlockfrostProvider::new(
            var("BLOCKFROST_PREPROD_PROJECT_ID").unwrap().as_str(),
            "preprod",
        );
        let address: &str = "addr_test1wrhn0024gx9ndkmg5sfu4r6f79ewf0w42qdrd2clyuuvgjgylk345";
        let result = provider.fetch_address_utxos(address, None).await;
        println!("result: {:?}", result);
        match result {
            Ok(address_utxos) => {
                println!("address_utxos: {:?}", address_utxos);
                assert!(true);
            }
            _ => panic!("Error fetching address utxos"),
        }
    }

    #[tokio::test]
    async fn test_fetch_asset_addresses() {
        dotenv().ok();
        let provider: BlockfrostProvider = BlockfrostProvider::new(
            var("BLOCKFROST_PREPROD_PROJECT_ID").unwrap().as_str(),
            "preprod",
        );
        let asset = format!(
            "{}{}",
            "1c24687602c866101d41aa64e39685ee7092f26af15c5329104141fd", "6d657368"
        );

        let result = provider.fetch_asset_addresses(&asset).await;
        println!("result: {:?}", result);
        match result {
            Ok(asset_addresses) => {
                println!("asset_addresses: {:?}", asset_addresses);
                assert!(asset_addresses[0] == ("addr_test1qzhm3fg7v9t9e4nrlw0z49cysmvzfy3xpmvxuht80aa3rvnm5tz7rfnph9ntszp2fclw5m334udzq49777gkhwkztsks4c69rg".to_string(),"1".to_string()));
            }
            _ => panic!("Error fetching asset addresses"),
        }
    }

    #[tokio::test]
    async fn test_fetch_asset_metadata() {
        dotenv().ok();
        let provider: BlockfrostProvider = BlockfrostProvider::new(
            var("BLOCKFROST_PREPROD_PROJECT_ID").unwrap().as_str(),
            "preprod",
        );
        let asset = format!(
            "{}{}",
            "1c24687602c866101d41aa64e39685ee7092f26af15c5329104141fd", "6d657368"
        );

        let result = provider.fetch_asset_metadata(&asset).await;
        println!("result: {:?}", result);
        match result {
            Ok(asset_metadata) => {
                println!("asset_metadata: {:?}", asset_metadata);
                assert!(true);
            }
            _ => panic!("Error fetching asset metadata"),
        }
    }

    #[tokio::test]
    async fn test_fetch_block_info() {
        dotenv().ok();
        let provider = BlockfrostProvider::new(
            var("BLOCKFROST_PREPROD_PROJECT_ID").unwrap().as_str(),
            "preprod",
        );
        let block: &str = "3132189";

        let result = provider.fetch_block_info(block).await;
        println!("result: {:?}", result);
        match result {
            Ok(block_info) => {
                println!("block_info: {:?}", block_info);
                assert!(
                    block_info.hash
                        == "d527a0d00d917cb997c680a2dadd2b3642f26e7572e6074db98c45b2d270b1f1"
                );
            }
            _ => panic!("Error fetching block info"),
        }
    }

    #[tokio::test]
    async fn test_fetch_collection_assets() {
        dotenv().ok();
        let provider = BlockfrostProvider::new(
            var("BLOCKFROST_PREPROD_PROJECT_ID").unwrap().as_str(),
            "preprod",
        );

        let policy_id: &str = "1c24687602c866101d41aa64e39685ee7092f26af15c5329104141fd";

        let result = provider.fetch_collection_assets(policy_id, None).await;
        println!("result: {:?}", result);
        match result {
            Ok(collection_assets) => {
                println!("collection_assets: {:?}", collection_assets);
                assert!(true);
            }
            _ => panic!("Error fetching collection assets"),
        }
    }

    #[tokio::test]
    async fn test_fetch_protocol_parameters() {
        dotenv().ok();
        let provider = BlockfrostProvider::new(
            var("BLOCKFROST_PREPROD_PROJECT_ID").unwrap().as_str(),
            "preprod",
        );

        let result = provider.fetch_protocol_parameters(None).await;
        println!("result: {:?}", result);
        match result {
            Ok(protocol_para) => {
                println!("protocol_para: {:?}", protocol_para);
                assert!(true);
            }
            _ => panic!("Error fetching protocol para"),
        }
    }

    #[tokio::test]
    async fn test_fetch_tx_info() {
        dotenv().ok();
        let provider = BlockfrostProvider::new(
            var("BLOCKFROST_PREPROD_PROJECT_ID").unwrap().as_str(),
            "preprod",
        );
        let hash: &str = "ccdf490c8b7fd1e67f81b59eb98791d910cc785c23498a82ec845540467dc3ba";

        let result = provider.fetch_tx_info(hash).await;
        println!("result: {:?}", result);
        match result {
            Ok(tx_info) => {
                println!("tx_info: {:?}", tx_info);
                assert!(
                    tx_info.block
                        == "d527a0d00d917cb997c680a2dadd2b3642f26e7572e6074db98c45b2d270b1f1"
                );
            }
            _ => panic!("Error fetching tx info"),
        }
    }

    #[tokio::test]
    async fn test_fetch_utxo() {
        dotenv().ok();
        let provider = BlockfrostProvider::new(
            var("BLOCKFROST_PREPROD_PROJECT_ID").unwrap().as_str(),
            "preprod",
        );

        let hash: &str = "bda0866e2edc3778191960d4200a982af5530fee8e5c2efc75f6b35e5e546800";

        let result = provider.fetch_utxos(hash, Some(1)).await;
        println!("result: {:?}", result);
        match result {
            Ok(utxos) => {
                println!("utxos: {:?}", utxos);
                assert!(utxos[0].input.output_index == 1);
            }
            _ => panic!("Error fetching utxos"),
        }
    }
}
