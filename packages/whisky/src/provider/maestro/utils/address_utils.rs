use maestro_rust_sdk::models::addresses::{Asset as MAsset, Utxo};
use sidan_csl_rs::model::{Asset, UTxO, UtxoInput, UtxoOutput};

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
