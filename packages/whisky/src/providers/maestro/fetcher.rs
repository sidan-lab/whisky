use super::models::account::StakeAccountInformation;
use super::models::address::UtxosAtAddress;
use super::models::asset::{AssetInformations, CollectionAssets};
use super::models::protocol_parameters::ProtocolParameters;
use super::models::transaction::TransactionDetails;
use super::utils::*;
use super::MaestroProvider;
use async_trait::async_trait;
use maestro_rust_sdk::client::block_info::BlockInfo as MBlockInfo;
use maestro_rust_sdk::models::asset::AddressesHoldingAsset;
use maestro_rust_sdk::models::epochs::EpochResp;

use std::collections::HashMap;
use whisky_common::models::{AccountInfo, Asset, BlockInfo, Protocol, TransactionInfo, UTxO};

use whisky_common::*;

#[async_trait]
impl Fetcher for MaestroProvider {
    async fn fetch_account_info(&self, address: &str) -> Result<AccountInfo, WError> {
        let reward_address = if address.starts_with("addr") {
            resolve_reward_address(address).map_err(WError::from_err(
                "maestro::fetch_account_info resolve reward address",
            ))?
        } else {
            address.to_string()
        };

        let url = format!("/accounts/{}", reward_address);

        let resp = self
            .maestro_client
            .get(&url)
            .await
            .map_err(WError::from_err("maestro::fetch_account_info get"))?;

        let stake_account_information: StakeAccountInformation = serde_json::from_str(&resp)
            .map_err(WError::from_err("maestro::fetch_account_info type error"))?;

        let account_info: AccountInfo =
            account_information_to_account_info(stake_account_information.data);
        Ok(account_info)
    }

    async fn fetch_address_utxos(
        &self,
        address: &str,
        asset: Option<&str>,
    ) -> Result<Vec<UTxO>, WError> {
        let query_predicate =
            if address.starts_with("addr_vkh") || address.starts_with("addr_shared_vkh") {
                format!("/addresses/cred/{}", address)
            } else {
                format!("/addresses/{}", address)
            };

        let append_asset_string = match asset {
            Some(a) => format!("&asset={}", a),
            None => "".to_string(),
        };

        let url = format!("{}/utxos?count=100{}", query_predicate, append_asset_string,);

        let resp = self
            .maestro_client
            .get(&url)
            .await
            .map_err(WError::from_err("maestro::fetch_address_utxos get"))?;

        let mut utxos_at_address: UtxosAtAddress = serde_json::from_str(&resp)
            .map_err(WError::from_err("maestro::fetch_address_utxos type error"))?;

        let mut added_utxos: Vec<UTxO> = utxos_at_address
            .data
            .iter()
            .map(to_utxo)
            .collect::<Result<Vec<_>, _>>()
            .map_err(WError::from_err("maestro::fetch_address_utxos to_utxo"))?;
        println!("uxtos: {:?}", added_utxos);

        while utxos_at_address.next_cursor.is_some() {
            let append_cursor_string = format!(
                "&cursor={}",
                utxos_at_address.next_cursor.ok_or_else(WError::from_opt(
                    "fetch_address_utxos",
                    "append_cursor_string"
                ))?
            );
            let url = format!(
                "{}utxos?count=100{}{}",
                query_predicate, append_asset_string, append_cursor_string
            );
            let resp = self
                .maestro_client
                .get(&url)
                .await
                .map_err(WError::from_err("maestro::fetch_address_utxos get"))?;
            utxos_at_address = serde_json::from_str(&resp)
                .map_err(WError::from_err("maestro::fetch_address_utxos type error"))?;
            let uxtos: Vec<UTxO> = utxos_at_address
                .data
                .iter()
                .map(to_utxo)
                .collect::<Result<Vec<UTxO>, _>>()
                .map_err(WError::from_err("maestro::fetch_address_utxos to_utxo"))?;
            added_utxos.extend(uxtos);
        }

        Ok(added_utxos)
    }

    async fn fetch_asset_addresses(&self, asset: &str) -> Result<Vec<(String, String)>, WError> {
        let (policy_id, asset_name) = Asset::unit_to_tuple(asset);
        let url = format!("/assets/{}{}/addresses?count=100", &policy_id, &asset_name);

        let resp = self
            .maestro_client
            .get(&url)
            .await
            .map_err(WError::from_err("maestro::fetch_asset_addresses get"))?;

        let mut addresses_holding_asset: AddressesHoldingAsset = serde_json::from_str(&resp)
            .map_err(WError::from_err(
                "maestro::fetch_asset_addresses type error",
            ))?;

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
            let append_cursor_string = format!(
                "&cursor={}",
                addresses_holding_asset
                    .next_cursor
                    .ok_or_else(WError::from_opt(
                        "fetch_address_utxos",
                        "append_cursor_string"
                    ))?
            );
            let url = format!(
                "/assets/{}{}/addresses?count=100{}",
                &policy_id, &asset_name, append_cursor_string
            );

            let resp = self
                .maestro_client
                .get(&url)
                .await
                .map_err(WError::from_err("maestro::fetch_asset_addresses get"))?;
            addresses_holding_asset = serde_json::from_str(&resp).map_err(WError::from_err(
                "maestro::fetch_asset_addresses type error",
            ))?;
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
    ) -> Result<Option<HashMap<String, serde_json::Value>>, WError> {
        let (policy_id, asset_name) = Asset::unit_to_tuple(asset);
        let url = format!("/assets/{}{}", &policy_id, &asset_name);
        let resp = self
            .maestro_client
            .get(&url)
            .await
            .map_err(WError::from_err("maestro::fetch_asset_metadata get"))?;

        let asset_informations: AssetInformations = serde_json::from_str(&resp)
            .map_err(WError::from_err("maestro::fetch_asset_metadata type error"))?;

        let asset_metadata = asset_informations.data.latest_mint_tx_metadata;
        Ok(asset_metadata)
    }

    async fn fetch_block_info(&self, hash: &str) -> Result<BlockInfo, WError> {
        let url = format!("/blocks/{}", hash);

        let resp = self
            .maestro_client
            .get(&url)
            .await
            .map_err(WError::from_err("maestro::fetch_block_info get"))?;
        let m_block_info: MBlockInfo = serde_json::from_str(&resp)
            .map_err(WError::from_err("maestro::fetch_block_info type error"))?;

        let block_info: BlockInfo = block_info_data_to_block_info(m_block_info.data);

        Ok(block_info)
    }

    async fn fetch_collection_assets(
        &self,
        policy_id: &str,
        cursor: Option<String>,
    ) -> Result<(Vec<(String, String)>, Option<String>), WError> {
        let append_cursor_string = match cursor {
            Some(c) => format!("&cursor={}", c),
            None => "".to_string(),
        };
        let url = format!(
            "/policy/{}/assets?count=100{}",
            policy_id, append_cursor_string
        );

        let resp = self
            .maestro_client
            .get(&url)
            .await
            .map_err(WError::from_err("maestro::fetch_collection_assets get"))?;
        let collection_assets: CollectionAssets = serde_json::from_str(&resp).map_err(
            WError::from_err("maestro::fetch_collection_assets type error"),
        )?;

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

    async fn fetch_protocol_parameters(&self, epoch: Option<u32>) -> Result<Protocol, WError> {
        if let Some(_epoch) = epoch {
            return Err(WError::new(
                "",
                "Maestro only supports fetching Protocol parameters of the latest completed epoch.",
            ));
        }

        let protocol_url = "/protocol-parameters";

        let protocol_resp = self
            .maestro_client
            .get(protocol_url)
            .await
            .map_err(WError::from_err("maestro::fetch_protocol_parameters get"))?;

        let protocol_parameters: ProtocolParameters = serde_json::from_str(&protocol_resp)
            .map_err(WError::from_err(
                "maestro::fetch_protocol_parameters type error",
            ))?;

        let epoch_url = "/epochs/current";

        let epoch_resp = self
            .maestro_client
            .get(epoch_url)
            .await
            .map_err(WError::from_err("maestro::fetch_current_epoch get"))?;

        let epochs: EpochResp = serde_json::from_str(&epoch_resp)
            .map_err(WError::from_err("maestro::fetch_current_epoch type error"))?;

        let protocol: Protocol =
            protocol_paras_data_to_protocol(protocol_parameters.data, epochs.data).map_err(
                WError::from_err(
                    "maestro::fetch_protocol_parameters protocol_paras_data_to_protocol",
                ),
            )?;
        Ok(protocol)
    }

    async fn fetch_tx_info(&self, hash: &str) -> Result<TransactionInfo, WError> {
        let url = format!("/transactions/{}", hash);

        let resp = self
            .maestro_client
            .get(&url)
            .await
            .map_err(WError::from_err("maestro::fetch_tx_info get"))?;

        let transaction_details: TransactionDetails = serde_json::from_str(&resp)
            .map_err(WError::from_err("maestro::fetch_tx_info type error"))?;

        let transaction_info: TransactionInfo =
            transaction_detail_to_info(transaction_details.data).map_err(WError::from_err(
                "maestro::fetch_tx_info transaction_detail_to_info",
            ))?;
        Ok(transaction_info)
    }

    async fn fetch_utxos(&self, hash: &str, index: Option<u32>) -> Result<Vec<UTxO>, WError> {
        let url = format!("/transactions/{}", hash);

        let resp = self
            .maestro_client
            .get(&url)
            .await
            .map_err(WError::from_err("maestro::fetch_utxos get"))?;

        let transaction_details: TransactionDetails = serde_json::from_str(&resp)
            .map_err(WError::from_err("maestro::fetch_utxos type error"))?;

        let outputs: Vec<UTxO> = transaction_details
            .data
            .outputs
            .iter()
            .map(to_utxo)
            .collect::<Result<Vec<UTxO>, _>>()
            .map_err(WError::from_err("maestro::fetch_utxos  - to_utxo"))?;

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
            .maestro_client
            .get(url)
            .await
            .map_err(WError::from_err("maestro::get"))?;
        let any = serde_json::from_str(&resp).map_err(WError::from_err("maestro::get"))?;
        Ok(any)
    }
}
