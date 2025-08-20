use async_trait::async_trait;

use futures::future;
use std::collections::HashMap;
use whisky_common::models::{AccountInfo, BlockInfo, Protocol, TransactionInfo, UTxO};

use whisky_common::*;

use crate::kupo::models::KupoUtxo;
use crate::kupo::KupoProvider;

#[async_trait]
impl Fetcher for KupoProvider {
    async fn fetch_account_info(&self, _address: &str) -> Result<AccountInfo, WError> {
        todo!()
    }

    async fn fetch_address_utxos(
        &self,
        address: &str,
        asset: Option<&str>,
    ) -> Result<Vec<UTxO>, WError> {
        let url = format!("/matches/{}/unspent", address);

        let resp = self
            .kupo_client
            .get(&url)
            .await
            .map_err(WError::from_err("kupo::fetch_address_utxos get"))?;

        let kupo_utxos: Vec<KupoUtxo> = serde_json::from_str(&resp)
            .map_err(WError::from_err("kupo::fetch_address_utxos type error"))?;

        let utxos: Vec<UTxO> = future::join_all(kupo_utxos.iter().map(|utxo| self.to_utxo(utxo)))
            .await
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?;

        let utxos = match asset {
            Some(asset) => utxos
                .into_iter()
                .filter(|utxo| utxo.output.amount.iter().any(|a| a.unit() == asset))
                .collect(),
            None => utxos,
        };
        Ok(utxos)
    }

    async fn fetch_asset_addresses(&self, _asset: &str) -> Result<Vec<(String, String)>, WError> {
        todo!()
    }

    async fn fetch_asset_metadata(
        &self,
        _asset: &str,
    ) -> Result<Option<HashMap<String, serde_json::Value>>, WError> {
        todo!()
    }

    async fn fetch_block_info(&self, _hash: &str) -> Result<BlockInfo, WError> {
        todo!()
    }

    async fn fetch_collection_assets(
        &self,
        _policy_id: &str,
        _cursor: Option<String>,
    ) -> Result<(Vec<(String, String)>, Option<String>), WError> {
        todo!()
    }

    async fn fetch_protocol_parameters(&self, _epoch: Option<u32>) -> Result<Protocol, WError> {
        todo!()
    }

    async fn fetch_tx_info(&self, _hash: &str) -> Result<TransactionInfo, WError> {
        todo!()
    }

    async fn fetch_utxos(&self, hash: &str, index: Option<u32>) -> Result<Vec<UTxO>, WError> {
        let url = match index {
            Some(index) => format!("/matches/{}@{}/unspent", index, hash),
            None => format!("/matches/*@{}/unspent", hash),
        };

        let resp = self
            .kupo_client
            .get(&url)
            .await
            .map_err(WError::from_err("kupo::fetch_address_utxos get"))?;

        let kupo_utxos: Vec<KupoUtxo> = serde_json::from_str(&resp)
            .map_err(WError::from_err("kupo::fetch_address_utxos type error"))?;

        let utxos: Vec<UTxO> = future::join_all(kupo_utxos.iter().map(|utxo| self.to_utxo(utxo)))
            .await
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?;

        Ok(utxos)
    }

    async fn get(&self, url: &str) -> Result<serde_json::Value, WError> {
        let resp = self
            .kupo_client
            .get(url)
            .await
            .map_err(WError::from_err("kupo::get"))?;
        let any = serde_json::from_str(&resp).map_err(WError::from_err("kupo::get error type"))?;
        Ok(any)
    }
}
