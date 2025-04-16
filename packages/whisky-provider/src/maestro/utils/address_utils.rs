use maestro_rust_sdk::models::addresses::{Asset as MAsset, Utxo};
use whisky_common::models::{Asset, UTxO, UtxoInput, UtxoOutput};
use whisky_csl::csl::{Address, BaseAddress, JsError, RewardAddress};

pub fn maestro_asset_to_asset(asset: MAsset) -> Asset {
    Asset::new(asset.unit.clone(), asset.amount.to_string())
}

pub fn maestro_utxo_to_utxo(utxo: Utxo) -> UTxO {
    UTxO {
        input: UtxoInput {
            output_index: utxo.index as u32,
            tx_hash: utxo.tx_hash,
        },
        output: UtxoOutput {
            address: utxo.address,
            amount: utxo
                .assets
                .iter()
                .map(|asset| maestro_asset_to_asset(asset.clone()))
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
            script_ref: utxo
                .reference_script
                .as_ref()
                .map(|script| script.bytes.clone()),
            script_hash: utxo
                .reference_script
                .as_ref()
                .map(|script| script.hash.clone()),
        },
    }
}

pub fn resolve_reward_address(bech32: &str) -> Result<String, JsError> {
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
